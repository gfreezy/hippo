#[macro_use]
mod instance_class;
mod array_class;
mod instance_class_loader_class;
mod instance_mirror_class;

pub use self::instance_class::Field;
pub use self::instance_class::InstanceClass;
pub use self::instance_class::Method;
pub use self::instance_class::SuperClassesIter;
pub use self::instance_class_loader_class::InstanceClassLoaderClass;
pub use self::instance_mirror_class::InstanceMirrorClass;
use crate::class::array_class::{ObjArrayClass, TypeArrayClass};
use crate::class_loader::get_class_id_by_name;
use crate::class_parser::constant_pool::ConstPool;

use crate::gc::global_definition::{
    BasicType, JArray, JBoolean, JByte, JChar, JDouble, JFloat, JInt, JLong, JObject, JShort,
};
use crate::gc::oop::Oop;
use crate::gc::oop_desc::{ArrayOopDesc, InstanceOopDesc};
use crate::gc::tlab::alloc_tlab;

use nom::lib::std::collections::hash_map::RandomState;
use nom::lib::std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub enum Class {
    InstanceClass(InstanceClass),
    InstanceClassLoaderClass(InstanceClassLoaderClass),
    InstanceMirrorClass(InstanceMirrorClass),
    TypeArrayClass(TypeArrayClass),
    ObjArrayClass(ObjArrayClass),
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class{{ name: {}}}", self.name())
    }
}

impl Class {
    pub fn as_instance_class(&self) -> InstanceClass {
        match self {
            Class::InstanceClass(c) => c.clone(),
            _ => unreachable!(),
        }
    }

    pub fn ty(&self) -> ClassType {
        unimplemented!()
    }

    pub fn mirror_class(&self) -> JObject {
        match self {
            Class::InstanceClass(c) => c.mirror_class(),
            Class::InstanceClassLoaderClass(c) => c.mirror_class(),
            Class::InstanceMirrorClass(c) => c.mirror_class(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn instance_size(&self) -> usize {
        match self {
            Class::InstanceClass(c) => c.instance_size(),
            Class::InstanceClassLoaderClass(c) => c.instance_size(),
            Class::InstanceMirrorClass(c) => c.instance_size(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn set_ty(&self, _ty: ClassType) {
        unimplemented!()
    }

    pub fn is_interface(&self) -> bool {
        unimplemented!()
    }

    pub fn is_class(&self) -> bool {
        unimplemented!()
    }

    pub fn is_static(&self) -> bool {
        unimplemented!()
    }

    pub fn is_super(&self) -> bool {
        unimplemented!()
    }

    pub fn is_public(&self) -> bool {
        unimplemented!()
    }

    pub fn is_private(&self) -> bool {
        unimplemented!()
    }

    pub fn is_protected(&self) -> bool {
        unimplemented!()
    }

    pub fn is_final(&self) -> bool {
        unimplemented!()
    }

    pub fn class_loader(&self) -> JObject {
        match self {
            Class::InstanceClass(c) => c.class_loader(),
            Class::InstanceClassLoaderClass(c) => c.class_loader(),
            Class::InstanceMirrorClass(c) => c.class_loader(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn set_mirror_class(&self, mirror: JObject) {
        match self {
            Class::InstanceClass(c) => c.set_mirror_class(mirror),
            Class::InstanceClassLoaderClass(c) => c.set_mirror_class(mirror),
            Class::InstanceMirrorClass(c) => c.set_mirror_class(mirror),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn set_instance_size(&self, _size: usize) {
        unimplemented!()
    }

    pub fn static_size(&self) -> usize {
        unimplemented!()
    }

    pub fn constant_pool(&self) -> &ConstPool {
        match self {
            Class::InstanceClass(c) => c.constant_pool(),
            Class::InstanceClassLoaderClass(c) => c.constant_pool(),
            Class::InstanceMirrorClass(c) => c.constant_pool(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn access_flags(&self) -> u16 {
        unimplemented!()
    }

    pub fn super_class(&self) -> Option<Class> {
        unimplemented!()
    }

    pub fn instance_fields(&self) -> &HashMap<String, Field, RandomState> {
        unimplemented!()
    }

    pub fn static_fields(&self) -> &HashMap<String, Field, RandomState> {
        unimplemented!()
    }

    pub fn methods(&self) -> &[Method] {
        unimplemented!()
    }

    pub fn interfaces(&self) -> &[Class] {
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        match self {
            Class::InstanceClass(c) => c.name(),
            Class::InstanceClassLoaderClass(c) => c.name(),
            Class::InstanceMirrorClass(c) => c.name(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn iter_super_classes(&self) -> SuperClassesIter {
        unimplemented!()
    }

    pub fn did_implement_interface(&self, _interface: Class) -> bool {
        unimplemented!()
    }

    pub fn clinit_method(&self) -> Option<Method> {
        match self {
            Class::InstanceClass(c) => c.clinit_method(),
            Class::InstanceClassLoaderClass(c) => c.clinit_method(),
            Class::InstanceMirrorClass(c) => c.clinit_method(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn total_instance_fields(&self) -> usize {
        unimplemented!()
    }

    pub fn get_self_field(&self, _name: &str, _descriptor: &str) -> Option<Field> {
        unimplemented!()
    }

    pub fn get_interface_field(&self, _name: &str, _descriptor: &str) -> Option<Field> {
        unimplemented!()
    }

    pub fn get_self_method(
        &self,
        _name: &str,
        _descriptor: &str,
        _is_static: bool,
    ) -> Option<Method> {
        unimplemented!()
    }

    pub fn get_class_method(
        &self,
        _name: &str,
        _descriptor: &str,
        _is_static: bool,
    ) -> Option<Method> {
        unimplemented!()
    }

    pub fn get_interface_method(&self, _name: &str, _descriptor: &str) -> Option<Method> {
        unimplemented!()
    }

    pub fn get_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        match self {
            Class::InstanceClass(c) => c.get_method(name, descriptor, is_static),
            Class::InstanceClassLoaderClass(c) => c.get_method(name, descriptor, is_static),
            Class::InstanceMirrorClass(c) => c.get_method(name, descriptor, is_static),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn get_static_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        match self {
            Class::InstanceClass(c) => c.get_static_field(name, descriptor),
            Class::InstanceClassLoaderClass(c) => c.get_static_field(name, descriptor),
            Class::InstanceMirrorClass(c) => c.get_static_field(name, descriptor),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn get_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        match self {
            Class::InstanceClass(c) => c.get_field(name, descriptor),
            Class::InstanceClassLoaderClass(c) => c.get_field(name, descriptor),
            Class::InstanceMirrorClass(c) => c.get_field(name, descriptor),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn is_subclass_of(&self, _class: Class) -> bool {
        unimplemented!()
    }

    pub fn as_instance_mirror_class(&self) -> InstanceMirrorClass {
        match self {
            Class::InstanceMirrorClass(c) => c.clone(),
            _ => unreachable!(),
        }
    }
}

pub type ClassId = usize;

pub enum ClassType {
    InstanceClass,
    InstanceRefClass,
    InstanceMirrorClass,
    InstanceClassLoaderClass,
    TypeArrayClass,
    ObjArrayClass,
    None,
}

pub fn alloc_jobject(class: &Class) -> JObject {
    let size = class.instance_size() + InstanceOopDesc::header_size_in_bytes();
    let _oop = alloc_tlab(size);

    JObject::new(alloc_tlab(size), get_class_id_by_name(class.name()))
}

pub fn alloc_jarray(ty: BasicType, class_id: ClassId, len: usize) -> JArray {
    let size = match ty {
        BasicType::Boolean => ArrayOopDesc::array_size_in_bytes::<JBoolean>(len),
        BasicType::Char => ArrayOopDesc::array_size_in_bytes::<JChar>(len),
        BasicType::Float => ArrayOopDesc::array_size_in_bytes::<JFloat>(len),
        BasicType::Double => ArrayOopDesc::array_size_in_bytes::<JDouble>(len),
        BasicType::Byte => ArrayOopDesc::array_size_in_bytes::<JByte>(len),
        BasicType::Short => ArrayOopDesc::array_size_in_bytes::<JShort>(len),
        BasicType::Int => ArrayOopDesc::array_size_in_bytes::<JInt>(len),
        BasicType::Long => ArrayOopDesc::array_size_in_bytes::<JLong>(len),
        BasicType::Object => ArrayOopDesc::array_size_in_bytes::<JObject>(len),
        BasicType::Array => unreachable!(),
    };
    JArray::new(alloc_tlab(size), class_id, len)
}
