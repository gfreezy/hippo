use crate::class::Class;
use crate::gc::global_definition::BasicType;
use std::sync::Arc;

#[derive(Clone)]
pub struct TypeArrayClass {
    inner: Arc<InnerTypeArrayClass>,
}

struct InnerTypeArrayClass {
    dimension: usize,
    ty: BasicType,
}

impl TypeArrayClass {
    pub fn ty(&self) -> BasicType {
        self.inner.ty
    }
}

#[derive(Clone)]
pub struct ObjArrayClass {
    inner: Arc<InnerObjArrayClass>,
}

struct InnerObjArrayClass {
    dimension: usize,
    element_class: Class,
}
impl ObjArrayClass {
    pub fn element_class(&self) -> Class {
        self.inner.element_class.clone()
    }
}
