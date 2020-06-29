use crate::class::{alloc_jobject, Class, ClassId, InstanceMirrorClass};
use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;

use crate::gc::global_definition::JObject;

use crate::java_const::{JAVA_LANG_CLASS, JAVA_LANG_OBJECT};
use crate::jthread::JvmThread;
use crate::jvm::{execute_class_method, execute_method};
use nom::lib::std::collections::HashMap;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::collections::hash_map::Entry;

struct GlobalClasses {
    inner: RwLock<Inner>,
}

struct Inner {
    classes: Vec<Class>,
    defining_classes: Vec<ClassId>,
    initiating_classes: Vec<ClassId>,
    map: HashMap<String, ClassId>,
}

impl GlobalClasses {
    pub fn new() -> Self {
        let inner = Inner {
            classes: Vec::new(),
            map: HashMap::new(),
            defining_classes: Vec::new(),
            initiating_classes: Vec::new(),
        };
        GlobalClasses {
            inner: RwLock::new(inner),
        }
    }
}

lazy_static::lazy_static! {
    static ref GLOBAL_CLASSES: GlobalClasses = GlobalClasses::new();
}
pub static BOOTSTRAP_LOADER: OnceCell<BootstrapClassLoader> = OnceCell::new();

pub fn get_class_by_id(id: ClassId) -> Class {
    let g = GLOBAL_CLASSES.inner.read();
    g.classes
        .get(id as usize)
        .unwrap_or_else(|| panic!("get class by id: {}", id))
        .clone()
}

pub fn get_class_id_by_name(name: &str) -> ClassId {
    let g = GLOBAL_CLASSES.inner.read();
    *g.map.get(name).unwrap()
}

fn get_class_by_name(name: &str) -> Option<Class> {
    let g = GLOBAL_CLASSES.inner.read();
    let id = g.map.get(name)?;
    Some(g.classes.get(*id)?.clone())
}

fn register_class(class: Class, loader: JObject) -> ClassId {
    println!("register class {}", class.name());

    let class_id = {
        let mut g = GLOBAL_CLASSES.inner.write();
        let Inner { classes, map, .. } = &mut *g;
        let entry = map.entry(class.name().to_string());
        if let Entry::Occupied(occupied) = entry {
            return *occupied.get();
        }
        let class_id = classes.len();
        classes.push(class.clone());
        entry.or_insert(class_id);
        class_id
    };

    class_id
}

pub fn init_class(thread: &mut JvmThread, class: &Class) {
    if class.is_inited() {
        return;
    }

    for c in class.iter_super_classes() {
        init_class(thread, &c);
    }

    for i in class.interfaces() {
        init_class(thread, i);
    }

    class.set_inited();
    let clinit_method = class.clinit_method();
    if let Some(clinit_method) = clinit_method {
        execute_class_method(thread, class.clone(), clinit_method, vec![]);
    }
}

pub fn load_class(loader: JObject, name: &str) -> Class {
    if let Some(class) = get_class_by_name(name) {
        assert_eq!(class.class_loader(), loader);
        return class;
    }

    let class = if loader.is_null() {
        println!("load class {}", name);
        let boot_loader = BOOTSTRAP_LOADER.get().expect("get bootstarap_loader");
        boot_loader.load_class(name)
    } else {
        unreachable!()
    };
    let _class_id = register_class(class.clone(), loader.clone());
    class
}

pub mod bootstrap_class_loader;
pub mod class_path;
