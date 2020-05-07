mod instance_class;
mod obj_array_class;
mod type_array_class;

use crate::class_parser::constant_pool::ConstPool;
use crate::runtime::class::instance_class::SuperClassesIter;
use crate::runtime::field::Field;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::method::Method;
pub use instance_class::InstanceClass;
pub use obj_array_class::ObjArrayClass;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
pub use type_array_class::TypeArrayClass;

#[derive(Clone, Debug)]
pub enum Class {
    InstanceClass(InstanceClass),
    ObjArrayClass(ObjArrayClass),
    TypeArrayClass(TypeArrayClass),
}

impl Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Class {
    pub fn instance_class(&self) -> InstanceClass {
        match self {
            Class::InstanceClass(i) => i.clone(),
            _ => unreachable!(),
        }
    }

    pub fn instance_class_ref(&self) -> &InstanceClass {
        match self {
            Class::InstanceClass(i) => &i,
            _ => unreachable!(),
        }
    }

    pub fn obj_array_class(&self) -> ObjArrayClass {
        match self {
            Class::ObjArrayClass(i) => i.clone(),
            _ => unreachable!(),
        }
    }
    pub fn type_array_class(&self) -> TypeArrayClass {
        match self {
            Class::TypeArrayClass(i) => i.clone(),
            _ => unreachable!(),
        }
    }

    pub fn is_interface(&self) -> bool {
        self.instance_class_ref().is_interface()
    }

    pub fn is_class(&self) -> bool {
        self.instance_class_ref().is_class()
    }

    pub fn is_static(&self) -> bool {
        self.instance_class_ref().is_static()
    }

    pub fn is_super(&self) -> bool {
        self.instance_class_ref().is_super()
    }

    pub fn is_public(&self) -> bool {
        self.instance_class_ref().is_public()
    }

    pub fn is_private(&self) -> bool {
        self.instance_class_ref().is_private()
    }

    pub fn is_protected(&self) -> bool {
        self.instance_class_ref().is_protected()
    }
    pub fn is_final(&self) -> bool {
        self.instance_class_ref().is_final()
    }

    pub fn constant_pool(&self) -> &ConstPool {
        match self {
            Class::InstanceClass(i) => i.constant_pool(),
            _ => unreachable!(),
        }
    }

    pub fn access_flags(&self) -> u16 {
        self.instance_class_ref().access_flags()
    }

    pub fn super_class(&self) -> Option<Class> {
        self.instance_class_ref().super_class().map(|c| c.into())
    }

    pub fn iter_super_classes(&self) -> SuperClassesIter {
        self.instance_class_ref().iter_super_classes()
    }

    pub fn instance_fields(&self) -> &HashMap<String, Field> {
        self.instance_class_ref().instance_fields()
    }

    pub fn all_instance_fields(&self) -> Vec<Field> {
        self.instance_class_ref().all_instance_fields()
    }

    pub fn static_fields(&self) -> &HashMap<String, Field> {
        self.instance_class_ref().static_fields()
    }

    pub fn total_instance_fields(&self) -> usize {
        self.instance_class_ref().total_instance_fields()
    }

    pub fn total_self_instance_fields(&self) -> usize {
        self.instance_class_ref().total_self_instance_fields()
    }

    pub fn methods(&self) -> &[Method] {
        self.instance_class_ref().methods()
    }

    pub fn interfaces(&self) -> &[InstanceClass] {
        self.instance_class_ref().interfaces()
    }

    pub fn did_implement_interface(&self, interface: InstanceClass) -> bool {
        self.instance_class_ref().did_implement_interface(interface)
    }

    pub fn name(&self) -> &str {
        self.instance_class_ref().name()
    }

    pub fn main_method(&self) -> Option<Method> {
        self.instance_class_ref().main_method()
    }

    pub fn clinit_method(&self) -> Option<Method> {
        self.instance_class_ref().clinit_method()
    }

    pub fn get_self_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        self.instance_class_ref()
            .get_self_method(name, descriptor, is_static)
    }

    pub fn get_class_method(
        &self,
        name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Option<Method> {
        self.instance_class_ref()
            .get_class_method(name, descriptor, is_static)
    }

    pub fn get_interface_method(&self, name: &str, descriptor: &str) -> Option<Method> {
        self.instance_class_ref()
            .get_interface_method(name, descriptor)
    }

    pub fn get_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        self.instance_class_ref()
            .get_method(name, descriptor, is_static)
    }

    pub fn get_static_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        self.instance_class_ref().get_static_field(name, descriptor)
    }

    pub fn get_static_field_value(&self, index: usize) -> Operand {
        self.instance_class_ref().get_static_field_value(index)
    }

    pub fn set_static_field_value(&self, index: usize, value: Operand) {
        self.instance_class_ref()
            .set_static_field_value(index, value)
    }

    pub fn get_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        self.instance_class_ref().get_field(name, descriptor)
    }

    pub fn is_subclass_of(&self, class: Class) -> bool {
        self.instance_class_ref()
            .is_subclass_of(class.instance_class())
    }
}

impl From<InstanceClass> for Class {
    fn from(c: InstanceClass) -> Self {
        Class::InstanceClass(c)
    }
}

impl From<ObjArrayClass> for Class {
    fn from(c: ObjArrayClass) -> Self {
        Class::ObjArrayClass(c)
    }
}
impl From<TypeArrayClass> for Class {
    fn from(c: TypeArrayClass) -> Self {
        Class::TypeArrayClass(c)
    }
}
