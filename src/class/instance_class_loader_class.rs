use crate::class::{Class, InstanceClass};
use crate::gc::global_definition::JObject;

#[derive(Clone)]
pub struct InstanceClassLoaderClass {
    class: InstanceClass,
}

impl_instance_class!(InstanceClassLoaderClass);

impl InstanceClassLoaderClass {
    pub fn instance_size(&self) -> usize {
        self.class.instance_size()
    }

    pub fn name(&self) -> &str {
        self.class.name()
    }

    pub fn mirror_class(&self) -> JObject {
        self.class.mirror_class()
    }
}

impl From<InstanceClassLoaderClass> for Class {
    fn from(cls: InstanceClassLoaderClass) -> Class {
        Class::InstanceClassLoaderClass(cls)
    }
}
