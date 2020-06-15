use crate::class::class::Class;

#[repr(C)]
pub struct InstanceClass {
    class: Class,
}

macro_rules! impl_instance_class {
    ($ty: tt) => {
        impl $ty {
            pub fn set_id(&self, id: crate::gc::oop_desc::ClassId) {
                self.class.set_id(id)
            }

            pub fn instance_size(&self) -> usize {
                self.class.instance_size()
            }

            pub fn id(&self) -> crate::gc::oop_desc::ClassId {
                self.class.id()
            }

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
                self.is_super()
            }

            pub fn is_public(&self) -> bool {
                self.is_public()
            }

            pub fn is_private(&self) -> bool {
                self.is_private()
            }

            pub fn is_protected(&self) -> bool {
                self.is_protected()
            }
            pub fn is_final(&self) -> bool {
                self.is_final()
            }
            pub fn set_mirror_class(&self, oop: crate::gc::oop::InstanceOop) {
                self.class.set_mirror_class(oop)
            }

            pub fn constant_pool(&self) -> &crate::class_parser::constant_pool::ConstPool {
                self.class.constant_pool()
            }

            pub fn access_flags(&self) -> u16 {
                self.class.access_flags()
            }

            pub fn super_class(&self) -> Option<$ty> {
                Some($ty {
                    class: self.class.super_class()?,
                })
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

        impl From<$ty> for crate::class::Class {
            fn from(c: $ty) -> Self {
                c.class
            }
        }
    };
}

impl_instance_class!(InstanceClass);

impl From<Class> for InstanceClass {
    fn from(c: Class) -> Self {
        InstanceClass { class: c }
    }
}
