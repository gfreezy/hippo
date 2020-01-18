use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::method_info::MethodInfo;
use crate::class_parser::{ACC_FINAL, ACC_NATIVE, ACC_STATIC};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Method {
    inner: Arc<InnerMethod>,
}

#[derive(Debug)]
struct InnerMethod {
    access_flags: u16,
    name: String,
    descriptor: String,
    max_locals: usize,
    max_stack: usize,
    code: Arc<Vec<u8>>,
}

impl Method {
    pub fn new(const_pool: &ConstPool, method_info: MethodInfo) -> Self {
        let name = const_pool.get_utf8_string_at(method_info.name_index);
        let descriptor = const_pool.get_utf8_string_at(method_info.descriptor_index);
        let access_flags = method_info.access_flags;
        if access_flags & ACC_NATIVE == 0 {
            let code_attr = method_info.code_attribute();

            Method {
                inner: Arc::new(InnerMethod {
                    access_flags,
                    name: name.to_string(),
                    descriptor: descriptor.to_string(),
                    max_locals: code_attr.max_locals as usize,
                    max_stack: code_attr.max_stack as usize,
                    code: Arc::new(code_attr.code),
                }),
            }
        } else {
            Method {
                inner: Arc::new(InnerMethod {
                    access_flags,
                    name: name.to_string(),
                    descriptor: descriptor.to_string(),
                    max_locals: 0,
                    max_stack: 0,
                    code: Arc::new(vec![]),
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

    pub fn is_final(&self) -> bool {
        self.access_flags() & ACC_FINAL != 0
    }
}
