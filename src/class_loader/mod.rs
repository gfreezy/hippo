use crate::class::{alloc_object, Class, InstanceMirrorClass};
use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;
use crate::class_loader::class_path::ClassPath;
use crate::gc::oop::InstanceOop;
use crate::gc::oop_desc::ClassId;
use nom::lib::std::collections::HashMap;
use parking_lot::RwLock;
use std::collections::hash_map::Entry;

struct GlobalClasses {
    inner: RwLock<Inner>,
}

struct Inner {
    classes: Vec<Class>,
    map: HashMap<String, ClassId>,
}

impl GlobalClasses {
    pub fn new() -> Self {
        let inner = Inner {
            classes: Vec::new(),
            map: HashMap::new(),
        };
        GlobalClasses {
            inner: RwLock::new(inner),
        }
    }
}

lazy_static::lazy_static! {
    static ref GLOBAL_CLASSES: GlobalClasses = GlobalClasses::new();
}

pub fn get_class_by_id(id: ClassId) -> Class {
    let g = GLOBAL_CLASSES.inner.read();
    g.classes
        .get(id as usize)
        .unwrap_or_else(|| panic!("get class by id: {}", id))
        .clone()
}

pub fn get_class_by_name(name: &str) -> Class {
    let g = GLOBAL_CLASSES.inner.read();
    let id = g
        .map
        .get(name)
        .unwrap_or_else(|| panic!("get class by name: {}", name));
    g.classes
        .get(*id)
        .unwrap_or_else(|| panic!("get class by id: {}", id))
        .clone()
}

pub fn register_class(class: Class, loader: InstanceOop) -> ClassId {
    let mut g = GLOBAL_CLASSES.inner.write();
    let Inner { classes, map, .. } = &mut *g;
    let entry = map.entry(class.name().to_string());
    if let Entry::Occupied(occupied) = entry {
        return *occupied.get();
    }
    let class_id = classes.len();
    class.set_id(class_id);
    let mirror_class = InstanceMirrorClass::new(loader);
    let mirror_class_oop = alloc_object(mirror_class.into());
    class.set_mirror_class(mirror_class_oop.into());
    classes.push(class);
    entry.or_insert(class_id);
    class_id
}

pub fn load_class(loader: InstanceOop, name: &str) -> Class {
    if loader.is_empty() {
        unimplemented!()
    }
    unimplemented!()
}

mod bootstrap_class_loader;
mod class_path;
