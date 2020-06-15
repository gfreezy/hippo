mod class;
#[macro_use]
mod instance_class;
mod instance_class_loader_class;
mod instance_mirror_class;

pub use self::class::field::Field;
pub use self::class::method::Method;
pub use self::class::{Class, SuperClassesIter};
pub use self::instance_class::InstanceClass;
pub use self::instance_mirror_class::InstanceMirrorClass;
use crate::gc::oop::Oop;
use crate::gc::tlab::alloc_tlab;

struct InstanceClassLoaderClass;

struct ArrayClass;

struct ObjArrayClass;

struct TypeArrayClass;

pub fn alloc_object(class: Class) -> Oop {
    alloc_tlab(class.instance_size())
}
