use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::method_info::MethodInfo;
use crate::class_parser::{
    is_bit_set, ACC_ABSTRACT, ACC_FINAL, ACC_NATIVE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC,
    ACC_STATIC,
};
use nom::lib::std::fmt::Formatter;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
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
    code: Arc<Vec<u8>>,
    parameters: Vec<Parameter>,
}

impl Method {
    pub fn new(const_pool: &ConstPool, method_info: MethodInfo) -> Self {
        let name = const_pool.get_utf8_string_at(method_info.name_index);
        let descriptor = const_pool.get_utf8_string_at(method_info.descriptor_index);
        let access_flags = method_info.access_flags;
        if is_bit_set(access_flags, ACC_NATIVE) || is_bit_set(access_flags, ACC_ABSTRACT) {
            Method {
                inner: Arc::new(InnerMethod {
                    access_flags,
                    name: name.to_string(),
                    descriptor: descriptor.to_string(),
                    max_locals: 0,
                    max_stack: 0,
                    code: Arc::new(vec![]),
                    parameters: vec![],
                }),
            }
        } else {
            let parameters = if let Some(params) = method_info.parameters() {
                params
                    .into_iter()
                    .map(|p| Parameter {
                        name: const_pool.get_utf8_string_at(p.name_index).to_string(),
                        access_flags: p.access_flags,
                    })
                    .collect()
            } else {
                vec![]
            };
            let code_attr = method_info
                .code_attr()
                .expect(&format!("get method code attr: {}", name));

            Method {
                inner: Arc::new(InnerMethod {
                    access_flags,
                    name: name.to_string(),
                    descriptor: descriptor.to_string(),
                    max_locals: code_attr.max_locals as usize,
                    max_stack: code_attr.max_stack as usize,
                    code: Arc::new(code_attr.code),
                    parameters,
                }),
            }
        }
    }

    pub fn access_flags(&self) -> u16 {
        self.inner.access_flags
    }

    pub fn descriptor(&self) -> &str {
        &self.inner.descriptor
    }

    pub fn parameters(&self) -> &[Parameter] {
        &self.inner.parameters
    }

    pub fn name(&self) -> &str {
        &self.inner.name
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
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
