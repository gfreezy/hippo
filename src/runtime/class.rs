use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::ClassFile;
use crate::runtime::field::Field;
use crate::runtime::method::Method;
use nom::lib::std::collections::HashMap;
use std::cell::Cell;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Class {
    inner: Arc<InnerClass>,
    inited: Cell<bool>,
}

#[derive(Debug)]
struct InnerClass {
    constant_pool: ConstPool,
    access_flags: u16,
    super_class: Option<Class>,
    interfaces: Vec<Class>,
    fields: HashMap<String, Field>,
    methods: Vec<Method>,
}

impl Class {
    pub fn new(class_file: ClassFile, super_class: Option<Class>, interfaces: Vec<Class>) -> Self {
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
                constant_pool,
                access_flags,
                super_class,
                fields,
                methods,
                interfaces,
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

    pub fn main_method(&self) -> Option<Method> {
        self.get_method("main", "([Ljava/lang/String;)V", true)
    }

    pub fn cinit_method(&self) -> Method {
        self.get_method("<clinit>", "()V", true).unwrap()
    }

    fn get_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        println!(
            "name {} descriptor {} is_static {}",
            name, descriptor, is_static
        );
        self.methods()
            .iter()
            .find(|x| {
                x.is_static() == is_static && x.name() == name && x.descriptor() == descriptor
            })
            .cloned()
    }

    pub fn get_user_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        println!(
            "name {} descriptor {} is_static {}",
            name, descriptor, is_static
        );
        if name == "<init>" || name == "<clinit>" {
            return None;
        }

        self.get_method(name, descriptor, is_static)
    }

    fn get_field(&self, name: &str) -> Option<Field> {
        self.fields().get(name).cloned()
    }
}
