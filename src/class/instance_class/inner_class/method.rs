use super::cp_cache::CpCache;
use crate::class::Class;
use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::descriptor::method_descriptor;
use crate::class_parser::method_info::MethodInfo;
use crate::class_parser::{
    is_bit_set, ACC_ABSTRACT, ACC_FINAL, ACC_NATIVE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC,
    ACC_STATIC, ACC_VARARGS,
};
use crate::gc::global_definition::{BasicType, JObject};

use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Method {
    inner: Arc<InnerMethod>,
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
    parameters: Vec<Parameter>,
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

        if is_bit_set(access_flags, ACC_NATIVE) || is_bit_set(access_flags, ACC_ABSTRACT) {
            Method {
                inner: Arc::new(InnerMethod {
                    access_flags,
                    name: name.to_string(),
                    descriptor: descriptor.to_string(),
                    max_locals: 0,
                    max_stack: 0,
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

            Method {
                inner: Arc::new(InnerMethod {
                    access_flags,
                    name: name.to_string(),
                    descriptor: descriptor.to_string(),
                    cp_cache: Mutex::new(CpCache::new(code_attr.code.len())),
                    max_locals: code_attr.max_locals as usize,
                    max_stack: code_attr.max_stack as usize,
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
        self.access_flags() & ACC_STATIC != 0
    }

    pub fn is_native(&self) -> bool {
        self.access_flags() & ACC_NATIVE != 0
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
    pub fn is_abstract(&self) -> bool {
        self.access_flags() & ACC_ABSTRACT != 0
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
            && is_bit_set(self.access_flags(), ACC_VARARGS)
            && is_bit_set(self.access_flags(), ACC_NATIVE)
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
