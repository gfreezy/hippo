use crate::class::{alloc_jobject, InstanceClass, InstanceMirrorClass, ObjArrayClass};
use crate::class::{Class, TypeArrayClass};
use crate::class_loader::class_path::ClassPath;
use crate::class_parser::parse_class_file;
use crate::gc::global_definition::JObject;

use crate::class_loader::{load_class, register_class};
use crate::java_const::{JAVA_LANG_CLASS, JAVA_LANG_OBJECT};
use crate::jthread::JvmThread;
use tracing::debug;

#[derive(Debug)]
pub struct BootstrapClassLoader {
    class_path: ClassPath,
}

impl BootstrapClassLoader {
    pub fn new(class_path: ClassPath) -> Self {
        BootstrapClassLoader { class_path }
    }

    pub fn load_class(&self, name: &str) -> Class {
        debug!(%name, "load_class");
        let name_bytes = name.as_bytes();
        match name_bytes {
            [b'[', ty] => {
                let dimension = name_bytes.len() - 1;
                Class::TypeArrayClass(TypeArrayClass::new(
                    name.to_string(),
                    (*ty).into(),
                    dimension,
                    JObject::null(),
                ))
            }
            [b'[', b'L', class_name @ .., b';'] => {
                let dimension = 1;
                Class::ObjArrayClass(ObjArrayClass::new(
                    name.to_string(),
                    load_class(JObject::null(), std::str::from_utf8(class_name).unwrap()),
                    dimension,
                    JObject::null(),
                ))
            }
            [b'L', name_slice @ .., b';'] | name_slice => {
                let class_name = std::str::from_utf8(name_slice).unwrap();
                let data = self
                    .class_path
                    .read_class(class_name)
                    .unwrap_or_else(|_| panic!("read class file: {}", name));
                self.define_class(class_name.to_string(), data).into()
            }
        }
    }

    fn define_class(&self, name: String, data: Vec<u8>) -> InstanceClass {
        debug!(%name, data_len = data.len(), "define_class");
        let loader = JObject::null();
        let (_, class_file) = parse_class_file(&data).expect("parse class");
        let super_class_index = class_file.super_class;
        let super_class = if super_class_index == 0 {
            None
        } else {
            let super_class_name = class_file
                .constant_pool
                .get_class_name_at(super_class_index);
            Some(load_class(JObject::null(), super_class_name))
        };

        let mut interfaces = Vec::with_capacity(class_file.interfaces.len());
        for interface_index in &class_file.interfaces {
            let interface_name = class_file.constant_pool.get_class_name_at(*interface_index);
            interfaces.push(load_class(JObject::null(), interface_name));
        }

        InstanceClass::new(name, class_file, super_class, interfaces, loader)
    }
}
