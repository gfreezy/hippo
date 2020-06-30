use crate::class::{Class, InstanceClass};

#[derive(Clone)]
pub struct InstanceClassLoaderClass {
    class: InstanceClass,
}

impl_instance_class!(InstanceClassLoaderClass);

impl InstanceClassLoaderClass {
    pub fn instance_size(&self) -> usize {
        self.class.instance_size()
    }
}

impl From<InstanceClassLoaderClass> for Class {
    fn from(cls: InstanceClassLoaderClass) -> Class {
        Class::InstanceClassLoaderClass(cls)
    }
}
