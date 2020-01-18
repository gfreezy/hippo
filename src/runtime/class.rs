use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::ClassFile;
use crate::runtime::field::Field;
use crate::runtime::method::Method;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Class {
    inner: Arc<InnerClass>,
}

#[derive(Debug)]
struct InnerClass {
    constant_pool: ConstPool,
    access_flags: u16,
    super_class: Option<Class>,
    fields: Vec<Field>,
    methods: Vec<Method>,
}

impl Class {
    pub fn new(class_file: ClassFile, super_class: Option<Class>) -> Self {
        let ClassFile {
            constant_pool,
            access_flags,
            fields: field_infos,
            methods: method_infos,
            ..
        } = class_file;
        let fields = field_infos
            .into_iter()
            .map(|filed| Field::new(&constant_pool, filed))
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
            }),
        }
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

    pub fn fields(&self) -> &[Field] {
        &self.inner.fields
    }

    pub fn methods(&self) -> &[Method] {
        &self.inner.methods
    }

    pub fn main_method(&self) -> Option<Method> {
        self.get_method("main", "([Ljava/lang/String;)V", true)
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

    fn get_field(&self, name: &str) -> Option<Field> {
        self.fields().iter().find(|x| x.name() == name).cloned()
    }
}
