use crate::class::{Class, InnerClass};
use std::sync::Arc;

pub struct InstanceClassLoaderClass {
    class: InnerClass,
}

impl_instance_class!(InstanceClassLoaderClass);

impl InstanceClassLoaderClass {
    pub fn instance_size(&self) -> usize {
        self.class.instance_size()
    }
}

impl From<InstanceClassLoaderClass> for Class {
    fn from(cls: InstanceClassLoaderClass) -> Class {
        Class::InstanceClassLoaderClass(Arc::new(cls))
    }
}
