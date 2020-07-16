pub mod cp_cache;
pub mod field;
pub mod method;

use crate::class::{Class, InstanceMirrorClass};
use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::field_info::FieldInfo;
use crate::class_parser::{
    ClassFile, JVM_ACC_FINAL, JVM_ACC_INTERFACE, JVM_ACC_PRIVATE, JVM_ACC_PROTECTED,
    JVM_ACC_PUBLIC, JVM_ACC_STATIC, JVM_ACC_SUPER,
};
use crate::gc::global_definition::{JObject, JValue};
use crate::gc::mem::align_usize;
use field::descriptor_size_in_bytes;
use method::Method;

use crate::jenv::{alloc_jobject, new_java_lang_string, new_jclass};
use field::Field;
use nom::lib::std::collections::HashMap;

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::class_loader::load_mirror_class;
use crossbeam::atomic::AtomicCell;
use std::sync::atomic::Ordering::SeqCst;
use tracing::trace;

#[repr(C)]
pub struct InnerClass {
    name: String,
    constant_pool: ConstPool,
    access_flags: u16,
    super_class: Option<Class>,
    interfaces: Vec<Class>,
    static_fields: HashMap<String, Field>,
    instance_fields: HashMap<String, Field>,
    methods: Vec<Method>,
    instance_size: AtomicU64,
    static_size: usize,
    inited: AtomicBool,
    loader: JObject,
}

impl InnerClass {
    pub fn new(
        name: String,
        class_file: ClassFile,
        super_class: Option<Class>,
        interfaces: Vec<Class>,
        loader: JObject,
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
        for (field_name, descriptor, size, field_info) in fields {
            if field_info.is_static() {
                static_offset = align_usize(static_offset, size);
                let f = Field::new(
                    field_name,
                    descriptor,
                    field_info.access_flags,
                    size,
                    static_offset,
                    loader,
                );
                static_fields.insert(f.name().to_string(), f.clone());
                static_offset += f.size();
            } else {
                instance_offset = align_usize(instance_offset, size);
                let f = Field::new(
                    field_name,
                    descriptor,
                    field_info.access_flags,
                    size,
                    instance_offset,
                    loader,
                );
                instance_fields.insert(f.name().to_string(), f.clone());
                instance_offset += f.size();
            }
        }

        let methods = method_infos
            .into_iter()
            .map(|method| Method::new(&constant_pool, method, name.clone(), loader))
            .collect();

        let inner_class = InnerClass {
            name,
            constant_pool,
            access_flags,
            super_class,
            instance_fields,
            static_fields,
            methods,
            interfaces,
            instance_size: AtomicU64::new(instance_offset as u64),
            static_size: static_offset,
            inited: AtomicBool::new(false),
            loader,
        };
        inner_class
    }

    pub fn mirror_class(&self) -> JObject {
        let mirror_class = load_mirror_class(self.class_loader(), self.name());
        mirror_class.mirror_class()
    }

    pub fn instance_size(&self) -> usize {
        self.instance_size.load(Ordering::SeqCst) as usize
    }

    pub fn static_size(&self) -> usize {
        self.static_size
    }

    pub fn set_instance_size(&self, size: usize) {
        self.instance_size.store(size as u64, Ordering::SeqCst);
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags() & JVM_ACC_INTERFACE != 0
    }

    pub fn is_class(&self) -> bool {
        self.access_flags() & JVM_ACC_INTERFACE == 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags() & JVM_ACC_STATIC != 0
    }

    pub fn is_super(&self) -> bool {
        self.access_flags() & JVM_ACC_SUPER != 0
    }

    pub fn is_public(&self) -> bool {
        self.access_flags() & JVM_ACC_PUBLIC != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags() & JVM_ACC_PRIVATE != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags() & JVM_ACC_PROTECTED != 0
    }
    pub fn is_final(&self) -> bool {
        self.access_flags() & JVM_ACC_FINAL != 0
    }

    pub fn constant_pool(&self) -> &ConstPool {
        &self.constant_pool
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn super_class(&self) -> Option<Class> {
        self.super_class.clone()
    }

    pub fn class_loader(&self) -> JObject {
        self.loader
    }

    pub fn instance_fields(&self) -> &HashMap<String, Field> {
        &self.instance_fields
    }

    pub fn static_fields(&self) -> &HashMap<String, Field> {
        &self.static_fields
    }

    pub fn iter_fields(&self) -> impl Iterator<Item = &Field> {
        self.instance_fields
            .values()
            .chain(self.static_fields.values())
    }

    pub fn total_self_instance_fields(&self) -> usize {
        self.instance_fields.len()
    }

    pub fn methods(&self) -> &[Method] {
        &self.methods
    }

    pub fn interfaces(&self) -> &[Class] {
        &self.interfaces
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_inited(&self) -> bool {
        self.inited.load(SeqCst)
    }
    pub fn set_inited(&self) {
        self.inited.store(true, SeqCst)
    }

    pub fn iter_super_classes(&self) -> SuperClassesIter {
        SuperClassesIter(self.super_class.clone())
    }

    pub fn total_instance_fields(&self) -> usize {
        self.total_self_instance_fields()
            + self
                .super_class()
                .map(|class| class.total_instance_fields())
                .unwrap_or(0)
    }

    pub fn did_implement_interface(&self, interface: Class) -> bool {
        self.interfaces.contains(&interface)
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

    pub fn get_self_field(&self, name: &str, descriptor: &str) -> Option<Field> {
        self.instance_fields()
            .get(name)
            .filter(|f| f.descriptor() == descriptor)
            .cloned()
    }

    pub fn get_interface_field(&self, name: &str, descriptor: &str) -> Option<Field> {
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
        let mut super_class = self.super_class();
        while let Some(klass) = super_class {
            if klass == class {
                return true;
            }
            super_class = klass.super_class();
        }
        false
    }
}

pub struct SuperClassesIter(Option<Class>);

impl SuperClassesIter {
    pub fn new(class: Option<Class>) -> Self {
        SuperClassesIter(class)
    }
}

impl Iterator for SuperClassesIter {
    type Item = Class;

    fn next(&mut self) -> Option<Self::Item> {
        let super_class = self.0.take();
        if let Some(class) = &super_class {
            self.0 = class.super_class()
        }
        super_class
    }
}

fn get_default_value_from_field_info(
    field_info: &FieldInfo,
    const_pool: &ConstPool,
) -> Option<JValue> {
    let constant_value_index = field_info.constant_value_attribute()?.constant_value_index;
    if field_info.is_static() && field_info.is_final() {
        let descriptor = const_pool
            .get_utf8_string_at(field_info.descriptor_index)
            .to_string();

        Some(match descriptor.as_str() {
            "B" | "C" | "I" | "S" | "Z" => {
                JValue::Int(const_pool.get_constant_integer_at(constant_value_index))
            }
            "D" => JValue::Double(const_pool.get_constant_double_at(constant_value_index)),
            "F" => JValue::Float(const_pool.get_constant_float_at(constant_value_index)),
            "J" => JValue::Long(const_pool.get_constant_long_at(constant_value_index)),
            "Ljava/lang/String;" => JValue::Object(new_java_lang_string(
                const_pool.get_constant_string_at(constant_value_index),
            )),
            _ => unreachable!(),
        })
    } else {
        None
    }
}
