use crate::class::Class;

pub struct InstanceClassLoaderClass {
    class: Class,
}

impl From<Class> for InstanceClassLoaderClass {
    fn from(class: Class) -> Self {
        InstanceClassLoaderClass { class }
    }
}

impl_instance_class!(InstanceClassLoaderClass);
