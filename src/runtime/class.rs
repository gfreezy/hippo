use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::{
    ClassFile, ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC,
};
use crate::runtime::field::Field;
use crate::runtime::method::Method;
use nom::lib::std::collections::HashMap;
use nom::lib::std::fmt::Formatter;
use std::cell::Cell;
use std::fmt;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct Class {
    inner: Arc<InnerClass>,
}

#[derive(Debug)]
struct InnerClass {
    name: String,
    constant_pool: ConstPool,
    access_flags: u16,
    super_class: Option<Class>,
    interfaces: Vec<Class>,
    fields: HashMap<String, Field>,
    methods: Vec<Method>,
    object_class: Option<Class>,
    inited: Cell<bool>,
}

impl Class {
    pub fn new(
        name: String,
        class_file: ClassFile,
        super_class: Option<Class>,
        interfaces: Vec<Class>,
        object_class: Option<Class>,
    ) -> Self {
        let ClassFile {
            constant_pool,
            access_flags,
            fields: field_infos,
            methods: method_infos,
            ..
        } = class_file;
        let fields = field_infos
            .into_iter()
            .map(|filed| {
                let f = Field::new(&constant_pool, filed);
                (f.name().to_string(), f)
            })
            .collect();
        let methods = method_infos
            .into_iter()
            .map(|method| Method::new(&constant_pool, method))
            .collect();
        Class {
            inner: Arc::new(InnerClass {
                name,
                constant_pool,
                access_flags,
                super_class,
                fields,
                methods,
                interfaces,
                object_class,
                inited: Cell::new(false),
            }),
        }
    }

    pub fn set_inited(&self) {
        self.inner.inited.replace(true);
    }

    pub fn is_inited(&self) -> bool {
        self.inner.inited.get()
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags() & ACC_INTERFACE != 0
    }

    pub fn is_class(&self) -> bool {
        self.access_flags() & ACC_INTERFACE == 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags() & ACC_STATIC != 0
    }

    pub fn is_public(&self) -> bool {
        self.access_flags() & ACC_PUBLIC != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags() & ACC_PRIVATE != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags() & ACC_PROTECTED != 0
    }
    pub fn is_final(&self) -> bool {
        self.access_flags() & ACC_FINAL != 0
    }

    pub fn constant_pool(&self) -> &ConstPool {
        &self.inner.constant_pool
    }

    pub fn access_flags(&self) -> u16 {
        self.inner.access_flags
    }

    pub fn super_class(&self) -> Option<Class> {
        self.inner.super_class.clone()
    }

    pub fn fields(&self) -> &HashMap<String, Field> {
        &self.inner.fields
    }

    pub fn methods(&self) -> &[Method] {
        &self.inner.methods
    }

    pub fn interfaces(&self) -> &[Class] {
        &self.inner.interfaces
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn main_method(&self) -> Option<Method> {
        self.get_self_method("main", "([Ljava/lang/String;)V", true)
    }

    pub fn cinit_method(&self) -> Option<Method> {
        self.get_self_method("<clinit>", "()V", true)
    }

    fn get_self_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        let method = self
            .methods()
            .iter()
            .find(|x| {
                x.is_static() == is_static && x.name() == name && x.descriptor() == descriptor
            })
            .cloned();
        debug!(name, descriptor, is_static, ?method, "get_self_method");
        method
    }

    fn get_class_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        assert!(self.is_class());
        // todo: polymorphic method
        if let Some(method) = self.get_self_method(name, descriptor, is_static) {
            return Some(method);
        }

        self.super_class()
            .and_then(|super_class| super_class.get_class_method(name, descriptor, is_static))
    }

    pub fn get_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        tracing::debug!(%name, %descriptor, is_static, "get_method");

        // todo: polymorphic method
        if let Some(method) = self.get_class_method(name, descriptor, is_static) {
            return Some(method);
        }
        self.interfaces()
            .iter()
            .find_map(|interface| interface.get_interface_method(name, descriptor))
    }

    fn get_self_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        self.fields()
            .get(name)
            .filter(|f| f.descriptor() == descriptor)
            .cloned()
    }

    fn get_interface_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        assert!(self.is_interface());
        if let Some(field) = self.get_self_field(name, descriptor) {
            return Some(field);
        }

        if let Some(field) = self
            .interfaces()
            .iter()
            .find_map(|interface| interface.get_self_field(name, descriptor))
        {
            return Some(field);
        }

        None
    }

    pub fn get_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        if let Some(field) = self.get_self_field(name, descriptor) {
            return Some(field);
        }

        if let Some(field) = self
            .interfaces()
            .iter()
            .find_map(|interface| interface.get_interface_field(name, descriptor))
        {
            return Some(field);
        }

        if let Some(field) = self
            .super_class()
            .and_then(|class| class.get_field(name, descriptor))
        {
            return Some(field);
        }
        None
    }

    fn get_interface_method(&self, name: &str, descriptor: &str) -> Option<Method> {
        assert!(self.is_interface());
        if let Some(method) = self.get_self_method(name, descriptor, false) {
            return Some(method);
        }

        if let Some(method) = self
            .inner
            .object_class
            .as_ref()
            .and_then(|o| o.get_method(name, descriptor, false))
        {
            if method.is_public() {
                return Some(method);
            }
        }

        self.interfaces()
            .iter()
            .filter_map(|interface| interface.get_interface_method(name, descriptor))
            .filter(|method| !method.is_abstract() && !method.is_private() && !method.is_static())
            .take(1)
            .collect::<Vec<_>>()
            .first()
            .cloned()
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
