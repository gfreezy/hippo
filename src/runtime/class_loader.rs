use crate::class_parser::parse_class_file;
use crate::class_path::ClassPath;
use crate::runtime::class::{Class, InstanceClass, ObjArrayClass, TypeArrayClass};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug)]
pub struct BootstrapClassLoader {
    class_path: ClassPath,
    classes: HashMap<String, Class>,
}

impl BootstrapClassLoader {
    pub fn new(class_path: ClassPath) -> Self {
        BootstrapClassLoader {
            class_path,
            classes: Default::default(),
        }
    }

    pub fn contains_class(&self, class: &Class) -> bool {
        self.classes.contains_key(class.name())
    }

    pub(super) fn load_class(&mut self, name: &str) -> Class {
        if self.classes.contains_key(name) {
            self.classes
                .get(name)
                .unwrap_or_else(|| panic!("get class: {}", name))
                .clone()
        } else {
            debug!(%name, "load_class");
            let name_bytes = name.as_bytes();
            let class = match name_bytes {
                [b'[', .., b'L'] => {
                    let class_name =
                        std::str::from_utf8(&name_bytes[2..name_bytes.len() - 1]).unwrap();
                    Class::ObjArrayClass(ObjArrayClass {
                        class: class_name.to_string(),
                    })
                }
                [b'[', ty, ..] => Class::TypeArrayClass(TypeArrayClass { ty: *ty }),
                [b'L', name_slice @ .., b';'] | name_slice => {
                    let name = std::str::from_utf8(name_slice).unwrap();
                    let data = self
                        .class_path
                        .read_class(name)
                        .unwrap_or_else(|_| panic!("read class file: {}", name));
                    self.define_class(name.to_string(), data).into()
                }
            };
            self.classes.insert(name.to_string(), class.clone());
            class
        }
    }

    fn define_class(&mut self, name: String, data: Vec<u8>) -> InstanceClass {
        debug!(%name, data_len = data.len(), "define_class");
        let (_, class_file) = parse_class_file(&data).expect("parse class");
        let super_class_index = class_file.super_class;
        let super_class = if super_class_index == 0 {
            None
        } else {
            let super_class_name = class_file
                .constant_pool
                .get_class_name_at(super_class_index);
            Some(self.load_class(super_class_name).instance_class())
        };

        let mut interfaces = Vec::with_capacity(class_file.interfaces.len());
        for interface_index in &class_file.interfaces {
            let interface_name = class_file.constant_pool.get_class_name_at(*interface_index);
            interfaces.push(self.load_class(interface_name).instance_class());
        }

        InstanceClass::new(name, class_file, super_class, interfaces)
    }
}
