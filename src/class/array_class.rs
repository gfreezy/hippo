use crate::class::{alloc_jobject, Class, InstanceMirrorClass};
use crate::gc::global_definition::{BasicType, JObject};
use crossbeam::atomic::AtomicCell;
use std::sync::Arc;

#[derive(Clone)]
pub struct TypeArrayClass {
    inner: Arc<InnerTypeArrayClass>,
}

struct InnerTypeArrayClass {
    dimension: usize,
    ty: BasicType,
    name: String,
    class_loader: JObject,
    mirror_class: AtomicCell<JObject>,
}

impl TypeArrayClass {
    pub fn new(name: String, ty: BasicType, dimension: usize, loader: JObject) -> Self {
        TypeArrayClass {
            inner: Arc::new(InnerTypeArrayClass {
                dimension,
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
            let mirror_class = InstanceMirrorClass::new(self.name(), self.class_loader());
            let mirror = alloc_jobject(&mirror_class.into());
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
    dimension: usize,
    element_class: Class,
    class_loader: JObject,
    mirror_class: AtomicCell<JObject>,
}
impl ObjArrayClass {
    pub fn new(name: String, class: Class, dimension: usize, loader: JObject) -> Self {
        ObjArrayClass {
            inner: Arc::new(InnerObjArrayClass {
                dimension,
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
            let mirror_class = InstanceMirrorClass::new(self.name(), self.class_loader());
            let mirror = alloc_jobject(&mirror_class.into());
            self.inner.mirror_class.store(mirror);
            mirror
        } else {
            mirror
        }
    }
}
