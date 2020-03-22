use crate::class_parser::parse_class_file;
use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug)]
pub struct ClassLoader {
    class_path: ClassPath,
    classes: HashMap<String, Class>,
}

impl ClassLoader {
    pub fn new(class_path: ClassPath) -> Self {
        ClassLoader {
            class_path,
            classes: Default::default(),
        }
    }

    pub(super) fn load_class(&mut self, name: &str) -> Class {
        if self.classes.contains_key(name) {
            return self
                .classes
                .get(name)
                .expect(&format!("get class: {}", name))
                .clone();
        } else {
            debug!(%name, "load_class");
            let data = self
                .class_path
                .read_class(name)
                .expect(&format!("read class file: {}", name));
            let class = self.define_class(name.to_string(), data);
            self.classes.insert(name.to_string(), class.clone());
            class
        }
    }

    fn define_class(&mut self, name: String, data: Vec<u8>) -> Class {
        debug!(%name, data_len = data.len(), "define_class");
        let (_, class_file) = parse_class_file(&data).expect("parse class");
        let super_class_index = class_file.super_class;
        let super_class = if super_class_index == 0 {
            None
        } else {
            let super_class_name = class_file
                .constant_pool
                .get_class_name_at(super_class_index);
            Some(self.load_class(super_class_name))
        };

        let mut interfaces = Vec::with_capacity(class_file.interfaces.len());
        for interface_index in &class_file.interfaces {
            let interface_name = class_file.constant_pool.get_class_name_at(*interface_index);
            interfaces.push(self.load_class(interface_name));
        }

        const OBJECT_CLASS: &str = "java/lang/Object;";
        let object_class = if name == OBJECT_CLASS {
            Some(self.load_class("java/lang/Object"))
        } else {
            None
        };
        Class::new(name, class_file, super_class, interfaces, object_class)
    }
}
