use crate::class::Class;
use crate::class_loader::{get_class_by_name, load_class};
use crate::gc::oop::InstanceOop;
use crate::java_const::JAVA_LANG_CLASS;

pub struct InstanceMirrorClass {
    class: Class,
}

impl From<Class> for InstanceMirrorClass {
    fn from(class: Class) -> Self {
        InstanceMirrorClass { class }
    }
}

impl_instance_class!(InstanceMirrorClass);

impl InstanceMirrorClass {
    pub fn new(loader: InstanceOop) -> Self {
        let class = load_class(loader, JAVA_LANG_CLASS);
        InstanceMirrorClass { class }
    }
}
