use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::field_info::FieldInfo;
use crate::class_parser::{
    ClassFile, ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC,
    ACC_SUPER,
};
use crate::runtime::field::Field;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::method::Method;
use nom::lib::std::collections::HashMap;
use nom::lib::std::fmt::Formatter;
use std::cell::Cell;
use std::fmt;
use std::sync::{Arc, Mutex};
use tracing::trace;

#[derive(Debug, Clone)]
pub struct InstanceClass {
    inner: Arc<InnerClass>,
}

#[derive(Debug)]
struct InnerClass {
    name: String,
    constant_pool: ConstPool,
    access_flags: u16,
    super_class: Option<InstanceClass>,
    interfaces: Vec<InstanceClass>,
    static_fields: HashMap<String, Field>,
    instance_fields: HashMap<String, Field>,
    static_field_values: Mutex<Vec<Operand>>,
    methods: Vec<Method>,
    object_class: Option<InstanceClass>,
    inited: Cell<bool>,
}

impl InstanceClass {
    pub fn new(
        name: String,
        class_file: ClassFile,
        super_class: Option<InstanceClass>,
        interfaces: Vec<InstanceClass>,
        object_class: Option<InstanceClass>,
    ) -> Self {
        let ClassFile {
            constant_pool,
            access_flags,
            fields: field_infos,
            methods: method_infos,
            ..
        } = class_file;
        let base_index = super_class
            .as_ref()
            .map(|c| c.total_instance_fields())
            .unwrap_or(0);
        let mut instance_index = base_index;
        let mut static_index = 0;
        let mut static_fields = HashMap::new();
        let mut instance_fields = HashMap::new();
        let mut static_field_values = Vec::new();
        for filed_info in &field_infos {
            if filed_info.is_static() {
                let f = Field::new(&constant_pool, filed_info, static_index);
                static_fields.insert(f.name(), f);
                let v = get_default_value_from_field_info(filed_info, &constant_pool)
                    .unwrap_or(Operand::Null);
                static_field_values.push(v);
                static_index += 1;
            } else {
                let f = Field::new(&constant_pool, filed_info, instance_index);
                instance_fields.insert(f.name(), f);
                instance_index += 1;
            }
        }
        let methods = method_infos
            .into_iter()
            .map(|method| Method::new(&constant_pool, method, name.clone()))
            .collect();
        let inner_class = InnerClass {
            name,
            constant_pool,
            access_flags,
            super_class,
            instance_fields,
            static_fields,
            static_field_values: Mutex::new(static_field_values),
            methods,
            interfaces,
            object_class,
            inited: Cell::new(false),
        };
        InstanceClass {
            inner: Arc::new(inner_class),
        }
    }

    pub fn set_inited(&self) {
        self.inner.inited.replace(true);
    }

    pub fn is_inited(&self) -> bool {
        self.inner.inited.get()
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

    pub fn super_class(&self) -> Option<InstanceClass> {
        self.inner.super_class.clone()
    }

    pub fn iter_super_classes(&self) -> SuperClassesIter {
        SuperClassesIter(self.clone())
    }

    pub fn instance_fields(&self) -> &HashMap<String, Field> {
        &self.inner.instance_fields
    }

    fn all_instance_fields_unordered(&self) -> Vec<Field> {
        let mut fields: Vec<_> = self.instance_fields().values().cloned().collect();
        if let Some(class) = self.super_class() {
            for f in class.all_instance_fields_unordered() {
                fields.push(f);
            }
        }
        fields
    }

    pub fn all_instance_fields(&self) -> Vec<Field> {
        let mut fs = self.all_instance_fields_unordered();
        fs.sort_by_key(|f| f.index());
        fs
    }
    pub fn static_fields(&self) -> &HashMap<String, Field> {
        &self.inner.static_fields
    }

    pub fn total_instance_fields(&self) -> usize {
        self.total_self_instance_fields()
            + self
                .super_class()
                .map(|class| class.total_instance_fields())
                .unwrap_or(0)
    }

    pub fn total_self_instance_fields(&self) -> usize {
        self.inner.instance_fields.len()
    }

    pub fn methods(&self) -> &[Method] {
        &self.inner.methods
    }

    pub fn interfaces(&self) -> &[InstanceClass] {
        &self.inner.interfaces
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn main_method(&self) -> Option<Method> {
        self.get_self_method("main", "([Ljava/lang/String;)V", true)
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

    pub fn get_static_field_value(&self, index: usize) -> Operand {
        self.inner.static_field_values.lock().unwrap()[index].clone()
    }

    pub fn set_static_field_value(&self, index: usize, value: Operand) {
        self.inner.static_field_values.lock().unwrap()[index] = value;
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

    pub fn is_subclass_of(&self, class: InstanceClass) -> bool {
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

impl fmt::Display for InstanceClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PartialEq for InstanceClass {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

pub struct SuperClassesIter(InstanceClass);

impl Iterator for SuperClassesIter {
    type Item = InstanceClass;

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
