use crate::class_parser::parse_class_file;
use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Classloader {
    class_path: ClassPath,
    classes: HashMap<String, Class>,
}

impl Classloader {
    pub fn new(class_path: ClassPath) -> Self {
        Classloader {
            class_path,
            classes: Default::default(),
        }
    }

    pub fn load_class(&mut self, name: String) -> Class {
        if self.classes.contains_key(&name) {
            return self.classes.get(&name).unwrap().clone();
        } else {
            let data = self.class_path.read_class(&name).expect("read class file");
            let class = self.define_class(data);
            self.classes.insert(name, class.clone());
            class
        }
    }

    fn define_class(&mut self, data: Vec<u8>) -> Class {
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

        Class::new(class_file, super_class)
    }
}
