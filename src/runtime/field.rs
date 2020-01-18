use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::field_info::FieldInfo;
use crate::class_parser::{ACC_FINAL, ACC_STATIC};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Field {
    inner: Arc<InnerField>,
}

#[derive(Debug)]
struct InnerField {
    access_flags: u16,
    name: String,
    descriptor: String,
    constant_value_index: Option<usize>,
}

impl Field {
    pub fn new(const_pool: &ConstPool, filed_info: FieldInfo) -> Field {
        let constant_value_index = filed_info
            .constant_value_attribute()
            .map(|attr| attr.constant_value_index as usize);
        let name = const_pool
            .get_utf8_string_at(filed_info.name_index)
            .to_string();
        let descriptor = const_pool
            .get_utf8_string_at(filed_info.descriptor_index)
            .to_string();
        Field {
            inner: Arc::new(InnerField {
                access_flags: filed_info.access_flags,
                name,
                descriptor,
                constant_value_index,
            }),
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

    pub fn constant_value_index(&self) -> Option<usize> {
        self.inner.constant_value_index
    }

    pub fn is_long_or_double(&self) -> bool {
        let descriptor = self.descriptor();
        descriptor == "J" || descriptor == "D"
    }

    pub fn is_static(&self) -> bool {
        self.access_flags() & ACC_STATIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags() & ACC_FINAL != 0
    }
}
