use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::{
    is_bit_set, ClassFile, ACC_ABSTRACT, ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED,
    ACC_PUBLIC, ACC_STATIC,
};
use crate::runtime::class::Class;
use crate::runtime::field::Field;
use crate::runtime::method::Method;
use nom::lib::std::collections::HashMap;
use nom::lib::std::fmt::Formatter;
use std::cell::Cell;
use std::fmt;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct Interface {
    inner: Arc<InnerInterface>,
    inited: Cell<bool>,
}

#[derive(Debug)]
struct InnerInterface {
    name: String,
    constant_pool: ConstPool,
    access_flags: u16,
    super_interfaces: Vec<Interface>,
    fields: HashMap<String, Field>,
    methods: Vec<Method>,
    object_class: Class,
}

impl Interface {
    pub fn new(
        name: String,
        class_file: ClassFile,
        super_interfaces: Vec<Interface>,
        object_class: Class,
    ) -> Self {
        let ClassFile {
            constant_pool,
            access_flags,
            fields: field_infos,
            methods: method_infos,
            ..
        } = class_file;
        assert!(is_bit_set(access_flags, ACC_INTERFACE));
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
        Interface {
            inner: Arc::new(InnerInterface {
                name,
                constant_pool,
                access_flags,
                super_interfaces,
                fields,
                methods,
                object_class,
            }),
            inited: Cell::new(false),
        }
    }

    pub fn set_inited(&self) {
        self.inited.replace(true);
    }

    pub fn is_inited(&self) -> bool {
        self.inited.get()
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
    pub fn is_abstract(&self) -> bool {
        self.access_flags() & ACC_ABSTRACT != 0
    }

    pub fn constant_pool(&self) -> &ConstPool {
        &self.inner.constant_pool
    }

    pub fn access_flags(&self) -> u16 {
        self.inner.access_flags
    }

    pub fn interfaces(&self) -> &[Interface] {
        &self.inner.super_interfaces
    }

    pub fn fields(&self) -> &HashMap<String, Field> {
        &self.inner.fields
    }

    pub fn methods(&self) -> &[Method] {
        &self.inner.methods
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn cinit_method(&self) -> Option<Method> {
        self.get_self_interface_method("<clinit>", "()V")
    }

    pub fn get_self_interface_method(&self, name: &str, descriptor: &str) -> Option<Method> {
        let method = self
            .methods()
            .iter()
            .find(|x| x.name() == name && x.descriptor() == descriptor)
            .cloned();
        debug!(name, descriptor, ?method, "get_self_method");
        method
    }

    pub fn get_interface_method(&self, name: &str, descriptor: &str) -> Option<Method> {
        if let Some(method) = self.get_self_interface_method(name, descriptor) {
            return Some(method);
        }

        if let Some(method) = self.inner.object_class.get_method(name, descriptor, false) {
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

    pub fn get_self_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        self.fields()
            .get(name)
            .filter(|f| f.descriptor() == descriptor)
            .cloned()
    }

    pub fn get_interface_field(&self, name: &str, descriptor: &str) -> Option<Field> {
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
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
