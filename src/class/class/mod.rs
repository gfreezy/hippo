pub mod cp_cache;
pub mod field;
pub mod method;

use crate::class::class::field::descriptor_size_in_bytes;
use crate::class::class::method::Method;
use crate::class::InstanceMirrorClass;
use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::field_info::FieldInfo;
use crate::class_parser::{
    ClassFile, ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC,
    ACC_SUPER,
};
use crate::gc::mem::align_usize;
use crate::gc::oop::InstanceOop;
use crate::gc::oop_desc::ClassId;
use crate::operand::Operand;
use field::Field;
use nom::lib::std::collections::HashMap;
use nom::lib::std::fmt::{Debug, Formatter};
use parking_lot::{Mutex, RwLock};
use std::cell::{Cell, RefCell};
use std::fmt;
use std::sync::Arc;
use tracing::trace;

#[derive(Clone)]
pub struct Class {
    inner: Arc<InnerClass>,
}

#[repr(C)]
struct InnerClass {
    id: Mutex<ClassId>,
    name: String,
    layout_helper: usize,
    constant_pool: ConstPool,
    access_flags: u16,
    super_class: Option<Class>,
    interfaces: Vec<Class>,
    static_fields: HashMap<String, Field>,
    instance_fields: HashMap<String, Field>,
    methods: Vec<Method>,
    mirror_class: RwLock<InstanceOop>,
    instance_size: usize,
    static_size: usize,
    loader: InstanceOop,
}

impl Class {
    pub fn new(
        name: String,
        class_file: ClassFile,
        super_class: Option<Class>,
        interfaces: Vec<Class>,
        loader: InstanceOop,
    ) -> Self {
        let ClassFile {
            constant_pool,
            access_flags,
            fields: field_infos,
            methods: method_infos,
            ..
        } = class_file;
        let mut fields = field_infos
            .into_iter()
            .map(|field_info| {
                let name = constant_pool
                    .get_utf8_string_at(field_info.name_index)
                    .to_string();
                let descriptor = constant_pool
                    .get_utf8_string_at(field_info.descriptor_index)
                    .to_string();
                let field_size = descriptor_size_in_bytes(&descriptor);
                (name, descriptor, field_size, field_info)
            })
            .collect::<Vec<_>>();
        fields.sort_by_key(|f| f.2);
        let mut instance_offset = super_class.clone().map(|c| c.instance_size()).unwrap_or(0);
        let mut static_offset = 0;
        let mut instance_fields = HashMap::new();
        let mut static_fields = HashMap::new();
        for (name, descriptor, size, field_info) in fields {
            if field_info.is_static() {
                static_offset = align_usize(static_offset, size);
                let mut f = Field::new(
                    name,
                    descriptor,
                    field_info.access_flags,
                    size,
                    static_offset,
                    loader,
                );
                static_fields.insert(f.name(), f.clone());
                static_offset += f.size();
            } else {
                instance_offset = align_usize(instance_offset, size);
                let mut f = Field::new(
                    name,
                    descriptor,
                    field_info.access_flags,
                    size,
                    instance_offset,
                    loader,
                );
                instance_fields.insert(f.name(), f.clone());
                instance_offset += f.size();
            }
        }

        let methods = method_infos
            .into_iter()
            .map(|method| Method::new(&constant_pool, method, name.clone(), loader))
            .collect();

        let loader_class = loader.0.class;

        let inner_class = InnerClass {
            id: Mutex::new(0),
            name,
            layout_helper: 0,
            constant_pool,
            access_flags,
            super_class,
            instance_fields,
            static_fields,
            methods,
            interfaces,
            mirror_class: RwLock::new(unsafe { InstanceOop::empty() }),
            instance_size: instance_offset,
            static_size: static_offset,
            loader,
        };
        Class {
            inner: Arc::new(inner_class),
        }
    }

    pub fn set_mirror_class(&self, oop: InstanceOop) {
        *self.inner.mirror_class.write() = oop;
    }

    pub fn set_id(&self, id: ClassId) {
        *self.inner.id.lock() = id;
    }

    pub fn id(&self) -> ClassId {
        *self.inner.id.lock()
    }

    pub fn instance_size(&self) -> usize {
        self.inner.instance_size
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags() & ACC_INTERFACE != 0
    }

    pub fn is_class(&self) -> bool {
        self.access_flags() & ACC_INTERFACE == 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags() & ACC_STATIC != 0
    }

    pub fn is_super(&self) -> bool {
        self.access_flags() & ACC_SUPER != 0
    }

    pub fn is_public(&self) -> bool {
        self.access_flags() & ACC_PUBLIC != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags() & ACC_PRIVATE != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags() & ACC_PROTECTED != 0
    }
    pub fn is_final(&self) -> bool {
        self.access_flags() & ACC_FINAL != 0
    }

    pub fn constant_pool(&self) -> &ConstPool {
        &self.inner.constant_pool
    }

    pub fn access_flags(&self) -> u16 {
        self.inner.access_flags
    }

    pub fn super_class(&self) -> Option<Class> {
        self.inner.super_class.clone()
    }

    pub fn instance_fields(&self) -> &HashMap<String, Field> {
        &self.inner.instance_fields
    }

    pub fn static_fields(&self) -> &HashMap<String, Field> {
        &self.inner.static_fields
    }

    pub fn total_self_instance_fields(&self) -> usize {
        self.inner.instance_fields.len()
    }

    pub fn methods(&self) -> &[Method] {
        &self.inner.methods
    }

    pub fn interfaces(&self) -> &[Class] {
        &self.inner.interfaces
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn iter_super_classes(&self) -> SuperClassesIter {
        SuperClassesIter(self.clone())
    }

    pub fn total_instance_fields(&self) -> usize {
        self.total_self_instance_fields()
            + self
                .super_class()
                .map(|class| class.total_instance_fields())
                .unwrap_or(0)
    }

    pub fn did_implement_interface(&self, interface: Class) -> bool {
        self.inner.interfaces.contains(&interface)
            || self
                .super_class()
                .map(|c| c.did_implement_interface(interface))
                .unwrap_or(false)
    }

    pub fn clinit_method(&self) -> Option<Method> {
        self.get_self_method("<clinit>", "()V", true)
    }

    pub fn get_self_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        let method = self
            .methods()
            .iter()
            .find(|x| {
                x.is_static() == is_static && x.name() == name && x.descriptor() == descriptor
            })
            .cloned();
        trace!(name, descriptor, is_static, ?method, "get_self_method");
        method
    }

    pub fn get_class_method(
        &self,
        name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Option<Method> {
        // todo: polymorphic method
        if let Some(method) = self.get_self_method(name, descriptor, is_static) {
            return Some(method);
        }

        self.super_class()
            .and_then(|super_class| super_class.get_class_method(name, descriptor, is_static))
    }

    fn get_interface_method_inner(&self, name: &str, descriptor: &str) -> Option<Method> {
        if let Some(method) = self
            .interfaces()
            .iter()
            .filter_map(|interface| interface.get_interface_method(name, descriptor))
            .filter(|method| !method.is_abstract() && !method.is_private() && !method.is_static())
            .take(1)
            .collect::<Vec<_>>()
            .first()
            .cloned()
        {
            return Some(method);
        }

        self.super_class()
            .and_then(|c| c.get_interface_method(name, descriptor))
    }

    pub fn get_interface_method(&self, name: &str, descriptor: &str) -> Option<Method> {
        if let Some(method) = self.get_self_method(name, descriptor, false) {
            return Some(method);
        }
        self.get_interface_method_inner(name, descriptor)
    }

    pub fn get_method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Method> {
        if let Some(method) = self.get_class_method(name, descriptor, is_static) {
            return Some(method);
        }
        self.get_interface_method_inner(name, descriptor)
    }

    fn get_self_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        self.instance_fields()
            .get(name)
            .filter(|f| f.descriptor() == descriptor)
            .cloned()
    }

    fn get_interface_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        assert!(self.is_interface());
        if let Some(field) = self.get_self_field(name, descriptor) {
            return Some(field);
        }

        if let Some(field) = self
            .interfaces()
            .iter()
            .find_map(|interface| interface.get_self_field(name, descriptor))
        {
            return Some(field);
        }

        None
    }

    pub fn get_static_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        self.static_fields()
            .get(name)
            .filter(|f| f.descriptor() == descriptor)
            .cloned()
    }

    pub fn get_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        if let Some(field) = self.get_self_field(name, descriptor) {
            return Some(field);
        }

        if let Some(field) = self
            .interfaces()
            .iter()
            .find_map(|interface| interface.get_interface_field(name, descriptor))
        {
            return Some(field);
        }

        if let Some(field) = self
            .super_class()
            .and_then(|class| class.get_field(name, descriptor))
        {
            return Some(field);
        }
        None
    }

    pub fn is_subclass_of(&self, class: Class) -> bool {
        let mut current = self.clone();
        while let Some(klass) = current.super_class() {
            if klass == class {
                return true;
            }
            current = klass;
        }
        false
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

pub struct SuperClassesIter(Class);

impl Iterator for SuperClassesIter {
    type Item = Class;

    fn next(&mut self) -> Option<Self::Item> {
        let super_class = self.0.super_class();
        if let Some(class) = super_class.clone() {
            self.0 = class;
        }
        super_class
    }
}

fn get_default_value_from_field_info(
    field_info: &FieldInfo,
    const_pool: &ConstPool,
) -> Option<Operand> {
    let constant_value_index = field_info.constant_value_attribute()?.constant_value_index;
    if field_info.is_static() && field_info.is_final() {
        let descriptor = const_pool
            .get_utf8_string_at(field_info.descriptor_index)
            .to_string();

        Some(match descriptor.as_str() {
            "B" | "C" | "I" | "S" | "Z" => {
                Operand::Int(const_pool.get_constant_integer_at(constant_value_index))
            }
            "D" => Operand::Double(const_pool.get_constant_double_at(constant_value_index)),
            "F" => Operand::Float(const_pool.get_constant_float_at(constant_value_index)),
            "J" => Operand::Long(const_pool.get_constant_long_at(constant_value_index)),
            "Ljava/lang/String;" => {
                Operand::Str(const_pool.get_constant_string_at(constant_value_index))
            }
            _ => unreachable!(),
        })
    } else {
        None
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class{{ name: {}}}", self.name())
    }
}
