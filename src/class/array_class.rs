use crate::class::{Class, ClassId, InnerClass};
use crate::gc::global_definition::BasicType;

pub struct TypeArrayClass {
    class: InnerClass,
    pub dimension: usize,
    pub ty: BasicType,
}

pub struct ObjArrayClass {
    class: InnerClass,
    pub dimension: usize,
    pub element_class: Class,
}
