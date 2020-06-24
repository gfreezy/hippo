use crate::class::{Class, InnerClass};
use crate::class_loader::load_class;
use crate::gc::global_definition::JObject;
use crate::gc::mem::align_usize;
use crate::gc::oop::{InstanceOop, Oop};
use crate::java_const::JAVA_LANG_CLASS;
use std::sync::Arc;

pub struct InstanceMirrorClass {
    class: InnerClass,
    base_static_offset: usize,
    class_name: String,
}

impl From<Class> for InstanceMirrorClass {
    fn from(class: Class) -> Self {
        let loader = class.class_loader();
        let class = load_class(loader, JAVA_LANG_CLASS);
        let java_class = load_class(loader, class.name());
        let java_class_static_size = java_class.instance_size();
        let self_instance_size = class.instance_size();
        let offset = align_usize(self_instance_size, 8);
        class.set_instance_size(offset + java_class_static_size);
        InstanceMirrorClass {
            class_name: class.name().to_string(),
            class: class.inner(),
            base_static_offset: offset,
        }
    }
}

impl_instance_class!(InstanceMirrorClass);

impl InstanceMirrorClass {
    pub fn new(name: &str, loader: JObject) -> Self {
        let class = load_class(loader, JAVA_LANG_CLASS);
        let java_class = load_class(loader, name);
        let java_class_static_size = java_class.instance_size();
        let self_instance_size = class.instance_size();
        let offset = align_usize(self_instance_size, 8);
        class.set_instance_size(offset + java_class_static_size);
        InstanceMirrorClass {
            class: class.inner(),
            base_static_offset: offset,
            class_name: name.to_string(),
        }
    }

    pub fn instance_size(&self) -> usize {
        self.class.instance_size()
    }

    pub fn mirrored_class_name(&self) -> &str {
        &self.class_name
    }
}

impl From<InstanceMirrorClass> for Class {
    fn from(cls: InstanceMirrorClass) -> Class {
        Class::InstanceMirrorClass(Arc::new(cls))
    }
}
