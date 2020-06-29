use crate::class::{Class, ClassId};
use crate::gc::global_definition::{BasicType, JObject};
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
}

impl TypeArrayClass {
    pub fn new(name: String, ty: BasicType, dimension: usize, loader: JObject) -> Self {
        TypeArrayClass {
            inner: Arc::new(InnerTypeArrayClass {
                dimension,
                ty,
                name,
                class_loader: loader,
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
}
impl ObjArrayClass {
    pub fn new(name: String, class: Class, dimension: usize, loader: JObject) -> Self {
        ObjArrayClass {
            inner: Arc::new(InnerObjArrayClass {
                dimension,
                name,
                class_loader: loader,
                element_class: class,
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
}
