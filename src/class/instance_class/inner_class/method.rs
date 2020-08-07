use super::cp_cache::CpCache;
use crate::class::Class;
use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::descriptor::method_descriptor;
use crate::class_parser::method_info::MethodInfo;
use crate::class_parser::{
    is_bit_set, JVM_ACC_ABSTRACT, JVM_ACC_FINAL, JVM_ACC_NATIVE, JVM_ACC_PRIVATE,
    JVM_ACC_PROTECTED, JVM_ACC_PUBLIC, JVM_ACC_STATIC, JVM_ACC_VARARGS,
};
use crate::gc::global_definition::{BasicType, JObject};

use crate::class_parser::attribute_info::predefined_attribute::{
    ExceptionHandler, LineNumberTable,
};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Method {
    inner: Arc<InnerMethod>,
}

impl Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Method", 4).unwrap();
        state.serialize_field("class", self.class_name()).unwrap();
        state.serialize_field("name", self.name()).unwrap();
        state
            .serialize_field("descriptor", self.descriptor())
            .unwrap();
        state
            .serialize_field("is_native", &self.is_native())
            .unwrap();
        state.end()
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: String,
    access_flags: u16,
}

#[derive(Debug)]
struct InnerMethod {
    access_flags: u16,
    name: String,
    descriptor: String,
    max_locals: usize,
    max_stack: usize,
    n_args: usize,
    code: Arc<Vec<u8>>,
    exception_table: Vec<ExceptionHandler>,
    parameters: Vec<Parameter>,
    line_number_tables: Vec<LineNumberTable>,
    class_name: String,
    param_descriptors: Vec<String>,
    return_descriptor: String,
    cp_cache: Mutex<CpCache>,
    loader: JObject,
}

impl Method {
    pub fn new(
        const_pool: &ConstPool,
        method_info: MethodInfo,
        class_name: String,
        loader: JObject,
    ) -> Self {
        let name = const_pool.get_utf8_string_at(method_info.name_index);
        let descriptor = const_pool.get_utf8_string_at(method_info.descriptor_index);
        let (_, (params, return_descriptor)) =
            method_descriptor(descriptor).expect("parse descriptor");
        let n_args = params.len();
        let access_flags = method_info.access_flags;
        let parameters = if let Some(params) = method_info.parameters() {
            params
                .iter()
                .map(|p| Parameter {
                    name: const_pool.get_utf8_string_at(p.name_index).to_string(),
                    access_flags: p.access_flags,
                })
                .collect()
        } else {
            vec![]
        };

        if is_bit_set(access_flags, JVM_ACC_NATIVE) || is_bit_set(access_flags, JVM_ACC_ABSTRACT) {
            Method {
                inner: Arc::new(InnerMethod {
                    access_flags,
                    name: name.to_string(),
                    descriptor: descriptor.to_string(),
                    max_locals: 0,
                    max_stack: 0,
                    exception_table: vec![],
                    line_number_tables: vec![],
                    code: Arc::new(vec![]),
                    n_args,
                    parameters,
                    class_name,
                    param_descriptors: params,
                    return_descriptor,
                    cp_cache: Mutex::new(CpCache::new(0)),
                    loader,
                }),
            }
        } else {
            let code_attr = method_info
                .code_attr()
                .unwrap_or_else(|| panic!("get method code attr: {}", name));
            let line_number_table = code_attr.line_number_tables();
            let exception_table = code_attr.exception_table;

            Method {
                inner: Arc::new(InnerMethod {
                    access_flags,
                    name: name.to_string(),
                    descriptor: descriptor.to_string(),
                    cp_cache: Mutex::new(CpCache::new(code_attr.code.len())),
                    max_locals: code_attr.max_locals as usize,
                    max_stack: code_attr.max_stack as usize,
                    exception_table,
                    line_number_tables: line_number_table,
                    code: Arc::new(code_attr.code),
                    n_args,
                    parameters,
                    class_name,
                    param_descriptors: params,
                    return_descriptor,
                    loader,
                }),
            }
        }
    }

    pub fn class_loader(&self) -> JObject {
        self.inner.loader
    }

    pub fn resolve_static_field(&self, pc: usize) -> Option<(Class, BasicType, usize)> {
        self.inner.cp_cache.lock().unwrap().resolve_static_field(pc)
    }

    pub fn resolve_field(&self, pc: usize) -> Option<(BasicType, usize)> {
        self.inner.cp_cache.lock().unwrap().resolve_field(pc)
    }

    pub fn line_for_pc(&self, pc: usize) -> Option<u16> {
        let tables = &self.inner.line_number_tables;
        if tables.is_empty() {
            return None;
        }
        let found = tables.binary_search_by_key(&pc, |t| t.start_pc as usize);
        Some(match found {
            Ok(index) => tables[index].line_number,
            Err(0) => unreachable!(),
            Err(index) => tables[index - 1].line_number,
        })
    }

    pub fn exception_handlers_for_pc(&self, pc: usize) -> Vec<ExceptionHandler> {
        let table = &self.inner.exception_table;
        if table.is_empty() {
            return vec![];
        }
        let pc = pc as u16;
        table
            .iter()
            .filter(|t| t.start_pc <= pc && t.end_pc > pc)
            .cloned()
            .collect()
    }

    pub fn set_field(&self, pc: usize, ty: BasicType, field_index: usize) {
        self.inner
            .cp_cache
            .lock()
            .unwrap()
            .set_field(pc, ty, field_index)
    }

    pub fn set_static_field(&self, pc: usize, class: Class, ty: BasicType, field_index: usize) {
        self.inner
            .cp_cache
            .lock()
            .unwrap()
            .set_static_field(pc, class, ty, field_index)
    }

    pub fn n_args(&self) -> usize {
        self.inner.n_args
    }

    pub fn return_descriptor(&self) -> &str {
        &self.inner.return_descriptor
    }

    pub fn access_flags(&self) -> u16 {
        self.inner.access_flags
    }

    pub fn descriptor(&self) -> &str {
        &self.inner.descriptor
    }

    pub fn is_initialization_method(&self) -> bool {
        self.name() == "<init>"
    }

    pub fn parameters(&self) -> &[Parameter] {
        &self.inner.parameters
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn class_name(&self) -> &str {
        &self.inner.class_name
    }

    pub fn max_locals(&self) -> usize {
        self.inner.max_locals
    }

    pub fn max_stack(&self) -> usize {
        self.inner.max_stack
    }

    pub fn code(&self) -> Arc<Vec<u8>> {
        self.inner.code.clone()
    }

    pub fn is_static(&self) -> bool {
        self.access_flags() & JVM_ACC_STATIC != 0
    }

    pub fn is_native(&self) -> bool {
        self.access_flags() & JVM_ACC_NATIVE != 0
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
    pub fn is_abstract(&self) -> bool {
        self.access_flags() & JVM_ACC_ABSTRACT != 0
    }
    pub fn is_signature_polymorphic(&self) -> bool {
        self.inner.class_name == "java/lang/invoke/MethodHandle"
            && self
                .parameters()
                .iter()
                .map(|p| &p.name)
                .collect::<Vec<_>>()
                == vec!["[java/lang/Object;"]
            && self.descriptor().split(')').last() == Some("java/lang/Object")
            && is_bit_set(self.access_flags(), JVM_ACC_VARARGS)
            && is_bit_set(self.access_flags(), JVM_ACC_NATIVE)
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name(), self.descriptor())?;
        if self.is_static() {
            write!(f, "-static")?;
        }
        Ok(())
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
            && self.descriptor() == other.descriptor()
            && self.access_flags() == other.access_flags()
            && self.class_name() == other.class_name()
    }
}
