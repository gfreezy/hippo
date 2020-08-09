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
pub use self::instance_mirror_class::{is_primitive_class, InstanceMirrorClass};
pub use crate::class::array_class::{ObjArrayClass, TypeArrayClass};
use crate::class_loader::load_class;
use crate::class_parser::constant_pool::ConstPool;

use crate::gc::global_definition::{BasicType, JObject};

use crate::java_const::JAVA_LANG_OBJECT;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;

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

impl Serialize for Class {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.name())
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Class::InstanceClass(_) => write!(f, "InstanceClass{{ name: {}}}", self.name()),
            Class::InstanceClassLoaderClass(_) => {
                write!(f, "InstanceClassLoaderClass{{ name: {}}}", self.name())
            }
            Class::InstanceMirrorClass(_) => {
                write!(f, "InstanceMirrorClass{{ name: {}}}", self.name())
            }
            Class::TypeArrayClass(_) => write!(f, "TypeArrayClass{{ name: {}}}", self.name()),
            Class::ObjArrayClass(_) => write!(f, "ObjArrayClass{{ name: {}}}", self.name()),
        }
    }
}

impl Class {
    pub fn as_instance_class(&self) -> Option<InstanceClass> {
        match self {
            Class::InstanceClass(c) => Some(c.clone()),
            _ => None,
        }
    }

    pub fn as_type_array_class(&self) -> Option<TypeArrayClass> {
        match self {
            Class::TypeArrayClass(c) => Some(c.clone()),
            _ => None,
        }
    }

    pub fn is_array_class(&self) -> bool {
        self.is_type_array_class() || self.is_obj_array_class()
    }

    pub fn is_type_array_class(&self) -> bool {
        matches!(self, Class::TypeArrayClass(_))
    }

    pub fn is_obj_array_class(&self) -> bool {
        matches!(self, Class::ObjArrayClass(_))
    }

    pub fn as_obj_array_class(&self) -> Option<ObjArrayClass> {
        match self {
            Class::ObjArrayClass(c) => Some(c.clone()),
            _ => None,
        }
    }

    pub fn as_instance_mirror_class(&self) -> Option<InstanceMirrorClass> {
        match self {
            Class::InstanceMirrorClass(c) => Some(c.clone()),
            _ => None,
        }
    }

    pub fn element_type(&self) -> BasicType {
        match self {
            Class::TypeArrayClass(c) => c.ty(),
            Class::ObjArrayClass(_) => BasicType::Object,
            _ => unreachable!(),
        }
    }

    pub fn mirror_class(&self) -> JObject {
        match self {
            Class::InstanceClass(c) => c.mirror_class(),
            Class::InstanceClassLoaderClass(c) => c.mirror_class(),
            Class::InstanceMirrorClass(c) => c.mirror_class(),
            Class::TypeArrayClass(c) => c.mirror_class(),
            Class::ObjArrayClass(c) => c.mirror_class(),
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

    pub fn is_inited(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_inited(),
            Class::InstanceClassLoaderClass(c) => c.is_inited(),
            Class::InstanceMirrorClass(c) => c.is_inited(),
            Class::TypeArrayClass(_) => true,
            Class::ObjArrayClass(c) => c.is_inited(),
        }
    }

    pub fn set_inited(&self) {
        match self {
            Class::InstanceClass(c) => c.set_inited(),
            Class::InstanceClassLoaderClass(c) => c.set_inited(),
            Class::InstanceMirrorClass(c) => c.set_inited(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => {}
        }
    }

    pub fn set_ty(&self, _ty: ClassType) {
        unimplemented!()
    }

    pub fn is_interface(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_interface(),
            Class::InstanceClassLoaderClass(c) => c.is_interface(),
            Class::InstanceMirrorClass(c) => c.is_interface(),
            Class::TypeArrayClass(_) => false,
            Class::ObjArrayClass(_) => false,
        }
    }

    pub fn is_class(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_class(),
            Class::InstanceClassLoaderClass(c) => c.is_class(),
            Class::InstanceMirrorClass(c) => c.is_class(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_static(),
            Class::InstanceClassLoaderClass(c) => c.is_static(),
            Class::InstanceMirrorClass(c) => c.is_static(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn is_super(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_super(),
            Class::InstanceClassLoaderClass(c) => c.is_super(),
            Class::InstanceMirrorClass(c) => c.is_super(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn is_public(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_public(),
            Class::InstanceClassLoaderClass(c) => c.is_public(),
            Class::InstanceMirrorClass(c) => c.is_public(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn is_private(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_private(),
            Class::InstanceClassLoaderClass(c) => c.is_private(),
            Class::InstanceMirrorClass(c) => c.is_private(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn is_protected(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_protected(),
            Class::InstanceClassLoaderClass(c) => c.is_protected(),
            Class::InstanceMirrorClass(c) => c.is_protected(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn is_final(&self) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_final(),
            Class::InstanceClassLoaderClass(c) => c.is_final(),
            Class::InstanceMirrorClass(c) => c.is_final(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn class_loader(&self) -> JObject {
        match self {
            Class::InstanceClass(c) => c.class_loader(),
            Class::InstanceClassLoaderClass(c) => c.class_loader(),
            Class::InstanceMirrorClass(c) => c.class_loader(),
            Class::TypeArrayClass(c) => c.class_loader(),
            Class::ObjArrayClass(c) => c.class_loader(),
        }
    }

    pub fn set_instance_size(&self, size: usize) {
        match self {
            Class::InstanceClass(c) => c.set_instance_size(size),
            Class::InstanceClassLoaderClass(c) => c.set_instance_size(size),
            Class::InstanceMirrorClass(c) => c.set_instance_size(size),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn static_size(&self) -> usize {
        match self {
            Class::InstanceClass(c) => c.static_size(),
            Class::InstanceClassLoaderClass(c) => c.static_size(),
            Class::InstanceMirrorClass(c) => c.static_size(),
            Class::TypeArrayClass(_) => 0,
            Class::ObjArrayClass(_) => 0,
        }
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
        match self {
            Class::InstanceClass(c) => c.super_class(),
            Class::InstanceClassLoaderClass(c) => c.super_class(),
            Class::InstanceMirrorClass(c) => c.super_class(),
            Class::TypeArrayClass(_) | Class::ObjArrayClass(_) => {
                Some(load_class(self.class_loader(), JAVA_LANG_OBJECT))
            }
        }
    }

    pub fn instance_fields(&self) -> &HashMap<String, Field> {
        match self {
            Class::InstanceClass(c) => c.instance_fields(),
            Class::InstanceClassLoaderClass(c) => c.instance_fields(),
            Class::InstanceMirrorClass(c) => c.instance_fields(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn static_fields(&self) -> &HashMap<String, Field> {
        match self {
            Class::InstanceClass(c) => c.static_fields(),
            Class::InstanceClassLoaderClass(c) => c.static_fields(),
            Class::InstanceMirrorClass(c) => c.static_fields(),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn iter_fields<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Field> + 'a> {
        match self {
            Class::InstanceClass(c) => Box::new(c.iter_fields()),
            Class::InstanceClassLoaderClass(c) => Box::new(c.iter_fields()),
            Class::InstanceMirrorClass(c) => Box::new(c.iter_fields()),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn methods(&self) -> &[Method] {
        match self {
            Class::InstanceClass(c) => c.methods(),
            Class::InstanceClassLoaderClass(c) => c.methods(),
            Class::InstanceMirrorClass(c) => c.methods(),
            Class::TypeArrayClass(_) => &[],
            Class::ObjArrayClass(_) => &[],
        }
    }

    pub fn interfaces(&self) -> &[Class] {
        match self {
            Class::InstanceClass(c) => c.interfaces(),
            Class::InstanceClassLoaderClass(c) => c.interfaces(),
            Class::InstanceMirrorClass(c) => c.interfaces(),
            Class::TypeArrayClass(_) => &[],
            Class::ObjArrayClass(_) => &[],
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Class::InstanceClass(c) => c.name(),
            Class::InstanceClassLoaderClass(c) => c.name(),
            Class::InstanceMirrorClass(c) => c.name(),
            Class::TypeArrayClass(c) => c.name(),
            Class::ObjArrayClass(c) => c.name(),
        }
    }

    pub fn iter_super_classes(&self) -> SuperClassesIter {
        SuperClassesIter::new(self.super_class())
    }

    pub fn did_implement_interface(&self, interface: Class) -> bool {
        match self {
            Class::InstanceClass(c) => c.did_implement_interface(interface),
            Class::InstanceClassLoaderClass(c) => c.did_implement_interface(interface),
            Class::InstanceMirrorClass(c) => c.did_implement_interface(interface),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn clinit_method(&self) -> Option<Method> {
        match self {
            Class::InstanceClass(c) => c.clinit_method(),
            Class::InstanceClassLoaderClass(c) => c.clinit_method(),
            Class::InstanceMirrorClass(c) => c.clinit_method(),
            Class::TypeArrayClass(_) => None,
            Class::ObjArrayClass(c) => c.element_class().clinit_method(),
        }
    }

    pub fn total_instance_fields(&self) -> usize {
        unimplemented!()
    }

    pub fn get_self_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        match self {
            Class::InstanceClass(c) => c.get_self_field(name, descriptor),
            Class::InstanceClassLoaderClass(c) => c.get_self_field(name, descriptor),
            Class::InstanceMirrorClass(c) => c.get_self_field(name, descriptor),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn get_interface_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        match self {
            Class::InstanceClass(c) => c.get_interface_field(name, descriptor),
            Class::InstanceClassLoaderClass(c) => c.get_interface_field(name, descriptor),
            Class::InstanceMirrorClass(c) => c.get_interface_field(name, descriptor),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn get_self_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        match self {
            Class::InstanceClass(c) => c.get_self_method(name, descriptor, is_static),
            Class::InstanceClassLoaderClass(c) => c.get_self_method(name, descriptor, is_static),
            Class::InstanceMirrorClass(c) => c.get_self_method(name, descriptor, is_static),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn get_class_method(
        &self,
        name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Option<Method> {
        match self {
            Class::InstanceClass(c) => c.get_class_method(name, descriptor, is_static),
            Class::InstanceClassLoaderClass(c) => c.get_class_method(name, descriptor, is_static),
            Class::InstanceMirrorClass(c) => c.get_class_method(name, descriptor, is_static),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
    }

    pub fn get_interface_method(&self, name: &str, descriptor: &str) -> Option<Method> {
        match self {
            Class::InstanceClass(c) => c.get_interface_method(name, descriptor),
            Class::InstanceClassLoaderClass(c) => c.get_interface_method(name, descriptor),
            Class::InstanceMirrorClass(c) => c.get_interface_method(name, descriptor),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
        }
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

    pub fn is_subclass_of(&self, class: Class) -> bool {
        match self {
            Class::InstanceClass(c) => c.is_subclass_of(class),
            Class::InstanceClassLoaderClass(c) => c.is_subclass_of(class),
            Class::InstanceMirrorClass(c) => c.is_subclass_of(class),
            Class::TypeArrayClass(_) => unreachable!(),
            Class::ObjArrayClass(_) => unreachable!(),
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
