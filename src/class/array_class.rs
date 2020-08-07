use crate::class::Class;
use crate::class_loader::load_mirror_class;
use crate::gc::global_definition::{BasicType, JObject};
use crate::jenv::new_jclass;
use crossbeam::atomic::AtomicCell;
use std::sync::Arc;

#[derive(Clone)]
pub struct TypeArrayClass {
    inner: Arc<InnerTypeArrayClass>,
}

struct InnerTypeArrayClass {
    ty: BasicType,
    name: String,
    class_loader: JObject,
    mirror_class: AtomicCell<JObject>,
}

impl TypeArrayClass {
    pub fn new(name: String, ty: BasicType, loader: JObject) -> Self {
        TypeArrayClass {
            inner: Arc::new(InnerTypeArrayClass {
                ty,
                name,
                class_loader: loader,
                mirror_class: AtomicCell::new(JObject::null()),
            }),
        }
    }

    pub fn ty(&self) -> BasicType {
        self.inner.ty
    }
    pub fn name(&self) -> &str {
        &self.inner.name
    }
    pub fn class_loader(&self) -> JObject {
        self.inner.class_loader.clone()
    }
    pub fn mirror_class(&self) -> JObject {
        let mirror = self.inner.mirror_class.load();
        if mirror.is_null() {
            let mirror_class = load_mirror_class(self.class_loader(), self.name());
            let mirror = new_jclass(&mirror_class.into());
            self.inner.mirror_class.store(mirror);
            mirror
        } else {
            mirror
        }
    }
}

#[derive(Clone)]
pub struct ObjArrayClass {
    inner: Arc<InnerObjArrayClass>,
}

struct InnerObjArrayClass {
    name: String,
    element_class: Class,
    class_loader: JObject,
    mirror_class: AtomicCell<JObject>,
}

impl ObjArrayClass {
    pub fn new(name: String, class: Class, loader: JObject) -> Self {
        ObjArrayClass {
            inner: Arc::new(InnerObjArrayClass {
                name,
                class_loader: loader,
                element_class: class,
                mirror_class: AtomicCell::new(JObject::null()),
            }),
        }
    }
    pub fn element_class(&self) -> Class {
        self.inner.element_class.clone()
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }
    pub fn class_loader(&self) -> JObject {
        self.inner.class_loader.clone()
    }
    pub fn is_inited(&self) -> bool {
        self.element_class().is_inited()
    }
    pub fn mirror_class(&self) -> JObject {
        let mirror = self.inner.mirror_class.load();
        if mirror.is_null() {
            let mirror_class = load_mirror_class(self.class_loader(), self.name());
            let mirror = new_jclass(&mirror_class.into());
            self.inner.mirror_class.store(mirror);
            mirror
        } else {
            mirror
        }
    }
}
