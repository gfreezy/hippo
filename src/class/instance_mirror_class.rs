use crate::class::{Class, InstanceClass};
use crate::class_loader::{init_class, load_class};
use crate::gc::global_definition::JObject;
use crate::gc::mem::align_usize;

use crate::java_const::JAVA_LANG_CLASS;
use nom::lib::std::fmt::Formatter;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub struct InstanceMirrorClass {
    class: InstanceClass,
    base_static_offset: usize,
    mirror_class_name: String,
}

impl_instance_class!(InstanceMirrorClass);

impl InstanceMirrorClass {
    pub fn new(name: &str, loader: JObject) -> Self {
        let java_class = load_class(loader, &name);
        let class = load_class(loader, JAVA_LANG_CLASS);
        let java_class_static_size = java_class.static_size();
        let self_instance_size = class.instance_size();
        let offset = align_usize(self_instance_size, 8);
        class.set_instance_size(offset + java_class_static_size);
        let mirror_class_name = Self::convert_to_mirror_class_name(name);
        InstanceMirrorClass {
            class: class.as_instance_class().unwrap(),
            base_static_offset: offset,
            mirror_class_name: mirror_class_name,
        }
    }

    pub fn instance_size(&self) -> usize {
        self.class.instance_size()
    }

    pub fn name(&self) -> &str {
        &self.mirror_class_name
    }

    pub fn mirror_name(&self) -> &str {
        let &[_, name]: &[&str; 2] = self
            .mirror_class_name
            .split(":")
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
        name
    }

    pub fn convert_to_mirror_class_name(name: &str) -> String {
        format!("mirror:{}", name)
    }
}

impl From<InstanceMirrorClass> for Class {
    fn from(cls: InstanceMirrorClass) -> Class {
        Class::InstanceMirrorClass(cls)
    }
}

impl Debug for InstanceMirrorClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InstanceMirrorClass {{ name: {}, base_static_offset: {} }}",
            self.name(),
            self.base_static_offset
        )
    }
}
