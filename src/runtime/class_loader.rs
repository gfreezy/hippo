use crate::class_parser::parse_class_file;
use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::interface::Interface;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ClassLoader {
    class_path: ClassPath,
    classes: HashMap<String, Class>,
    interfaces: HashMap<String, Interface>,
}

impl ClassLoader {
    pub fn new(class_path: ClassPath) -> Self {
        ClassLoader {
            class_path,
            classes: Default::default(),
            interfaces: Default::default(),
        }
    }

    pub fn load_class(&mut self, name: String) -> Class {
        if self.classes.contains_key(&name) {
            return self
                .classes
                .get(&name)
                .expect(&format!("get class: {}", name))
                .clone();
        } else {
            let data = self
                .class_path
                .read_class(&name)
                .expect(&format!("read class file: {}", name));
            let class = self.define_class(name.clone(), data);
            self.classes.insert(name, class.clone());
            class
        }
    }

    pub fn load_interface(&mut self, name: String) -> Interface {
        if self.interfaces.contains_key(&name) {
            return self
                .interfaces
                .get(&name)
                .expect(&format!("get interface: {}", name))
                .clone();
        } else {
            let data = self
                .class_path
                .read_class(&name)
                .expect(&format!("read class file: {}", name));
            let interface = self.define_interface(name.clone(), data);
            self.interfaces.insert(name, interface.clone());
            interface
        }
    }

    fn define_class(&mut self, name: String, data: Vec<u8>) -> Class {
        let (_, class_file) = parse_class_file(&data).expect("parse class");
        let super_class_index = class_file.super_class;
        let super_class = if super_class_index == 0 {
            None
        } else {
            let super_class_name = class_file
                .constant_pool
                .get_class_name_at(super_class_index);
            Some(self.load_class(super_class_name.to_string()))
        };

        let mut interfaces = Vec::with_capacity(class_file.interfaces.len());
        for interface_index in &class_file.interfaces {
            let interface_name = class_file.constant_pool.get_class_name_at(*interface_index);
            interfaces.push(self.load_interface(interface_name.to_string()));
        }

        Class::new(name, class_file, super_class, interfaces)
    }

    fn define_interface(&mut self, name: String, data: Vec<u8>) -> Interface {
        let (_, class_file) = parse_class_file(&data).expect("parse class");
        dbg!(class_file.super_class);
        let mut interfaces = Vec::with_capacity(class_file.interfaces.len());
        for interface_index in &class_file.interfaces {
            let interface_name = class_file.constant_pool.get_class_name_at(*interface_index);
            interfaces.push(self.load_interface(interface_name.to_string()));
        }

        Interface::new(
            name,
            class_file,
            interfaces,
            self.load_class("java/lang/Object".to_string()),
        )
    }
}
