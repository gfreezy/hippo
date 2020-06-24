use crate::class::{Class, ClassType, InnerClass};
use std::sync::Arc;

pub struct InstanceClassLoaderClass {
    class: InnerClass,
}

impl_instance_class!(InstanceClassLoaderClass);

impl From<InstanceClassLoaderClass> for Class {
    fn from(cls: InstanceClassLoaderClass) -> Class {
        Class::InstanceClassLoaderClass(Arc::new(cls))
    }
}
