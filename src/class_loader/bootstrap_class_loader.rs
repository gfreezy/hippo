use crate::class::{alloc_jobject, InnerClass, InstanceClass, InstanceMirrorClass};
use crate::class::{Class, ClassId};
use crate::class_loader::class_path::ClassPath;
use crate::class_loader::load_class;
use crate::class_parser::parse_class_file;
use crate::gc::global_definition::JObject;
use crate::gc::oop::Oop;
use nom::lib::std::collections::HashSet;
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug)]
pub struct BootstrapClassLoader {
    class_path: ClassPath,
    classes: HashSet<ClassId>,
}

impl BootstrapClassLoader {
    pub fn new(class_path: ClassPath) -> Self {
        BootstrapClassLoader {
            class_path,
            classes: HashSet::new(),
        }
    }

    pub fn contains_class(&self, class_id: ClassId) -> bool {
        self.classes.contains(&class_id)
    }

    pub fn load_class(&mut self, name: &str) -> Class {
        debug!(%name, "load_class");
        let name_bytes = name.as_bytes();
        let class: Class = match name_bytes {
            [b'[', .., b'L'] => unimplemented!(),
            [b'[', ty, ..] => unimplemented!(),
            [b'L', name_slice @ .., b';'] | name_slice => {
                let name = std::str::from_utf8(name_slice).unwrap();
                let data = self
                    .class_path
                    .read_class(name)
                    .unwrap_or_else(|_| panic!("read class file: {}", name));
                self.define_class(name.to_string(), data).into()
            }
        };
        class
    }

    fn define_class(&mut self, name: String, data: Vec<u8>) -> InstanceClass {
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
            Some(load_class(loader, super_class_name))
        };

        let mut interfaces = Vec::with_capacity(class_file.interfaces.len());
        for interface_index in &class_file.interfaces {
            let interface_name = class_file.constant_pool.get_class_name_at(*interface_index);
            interfaces.push(load_class(loader, interface_name));
        }

        let mirror_class = InstanceMirrorClass::new(&name, loader);
        let mirror_class_oop = alloc_jobject(&mirror_class.into());

        InstanceClass::new(
            name,
            class_file,
            super_class,
            interfaces,
            mirror_class_oop,
            loader,
        )
        .into()
    }
}
