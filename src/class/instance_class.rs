
use crate::class::{Class, InnerClass};
use crate::class_parser::ClassFile;
use crate::gc::global_definition::JObject;

use std::sync::Arc;

#[repr(C)]
pub struct InstanceClass {
    class: InnerClass,
}

macro_rules! impl_instance_class {
    ($ty: tt) => {
        impl $ty {
            pub fn is_interface(&self) -> bool {
                self.class.is_interface()
            }

            pub fn is_class(&self) -> bool {
                self.class.is_class()
            }

            pub fn is_static(&self) -> bool {
                self.class.is_static()
            }

            pub fn is_super(&self) -> bool {
                self.class.is_super()
            }

            pub fn is_public(&self) -> bool {
                self.class.is_public()
            }

            pub fn is_private(&self) -> bool {
                self.class.is_private()
            }

            pub fn is_protected(&self) -> bool {
                self.class.is_protected()
            }

            pub fn is_final(&self) -> bool {
                self.class.is_final()
            }

            pub fn class_loader(&self) -> crate::gc::global_definition::JObject {
                self.class.class_loader()
            }

            pub fn set_instance_size(&self, size: usize) {
                self.class.set_instance_size(size)
            }

            pub fn static_size(&self) -> usize {
                self.class.static_size()
            }

            pub fn constant_pool(&self) -> &crate::class_parser::constant_pool::ConstPool {
                self.class.constant_pool()
            }

            pub fn access_flags(&self) -> u16 {
                self.class.access_flags()
            }

            pub fn super_class(&self) -> Option<$crate::class::Class> {
                self.class.super_class()
            }

            pub fn instance_fields(
                &self,
            ) -> &std::collections::HashMap<String, crate::class::Field> {
                self.class.instance_fields()
            }

            pub fn static_fields(&self) -> &std::collections::HashMap<String, crate::class::Field> {
                self.class.static_fields()
            }

            pub fn methods(&self) -> &[crate::class::Method] {
                self.class.methods()
            }

            pub fn interfaces(&self) -> &[crate::class::Class] {
                self.class.interfaces()
            }

            pub fn name(&self) -> &str {
                self.class.name()
            }

            pub fn iter_super_classes(&self) -> crate::class::SuperClassesIter {
                self.class.iter_super_classes()
            }

            pub fn did_implement_interface(&self, interface: crate::class::Class) -> bool {
                self.class.did_implement_interface(interface)
            }

            pub fn clinit_method(&self) -> Option<crate::class::Method> {
                self.class.clinit_method()
            }

            pub fn get_self_method(
                &self,
                name: &str,
                descriptor: &str,
                is_static: bool,
            ) -> Option<crate::class::Method> {
                self.class.get_self_method(name, descriptor, is_static)
            }

            pub fn get_class_method(
                &self,
                name: &str,
                descriptor: &str,
                is_static: bool,
            ) -> Option<crate::class::Method> {
                self.class.get_class_method(name, descriptor, is_static)
            }

            pub fn get_interface_method(
                &self,
                name: &str,
                descriptor: &str,
            ) -> Option<crate::class::Method> {
                self.class.get_interface_method(name, descriptor)
            }

            pub fn get_method(
                &self,
                name: &str,
                descriptor: &str,
                is_static: bool,
            ) -> Option<crate::class::Method> {
                self.class.get_method(name, descriptor, is_static)
            }

            pub fn get_static_field(
                &self,
                name: &str,
                descriptor: &str,
            ) -> Option<crate::class::Field> {
                self.class.get_static_field(name, descriptor)
            }

            pub fn get_field(&self, name: &str, descriptor: &str) -> Option<crate::class::Field> {
                self.class.get_field(name, descriptor)
            }

            pub fn is_subclass_of(&self, class: crate::class::Class) -> bool {
                self.class.is_subclass_of(class)
            }
        }
    };
}

impl_instance_class!(InstanceClass);

impl InstanceClass {
    pub fn new(
        name: String,
        class_file: ClassFile,
        super_class: Option<Class>,
        interfaces: Vec<Class>,
        mirror_class: JObject,
        loader: JObject,
    ) -> Self {
        InstanceClass {
            class: InnerClass::new(
                name,
                class_file,
                super_class,
                interfaces,
                mirror_class,
                loader,
            ),
        }
    }
    pub fn instance_size(&self) -> usize {
        self.class.instance_size()
    }
}

impl From<InstanceClass> for Class {
    fn from(cls: InstanceClass) -> Class {
        Class::InstanceClass(Arc::new(cls))
    }
}

impl PartialEq for InstanceClass {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
