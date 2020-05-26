use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::field_info::FieldInfo;
use crate::class_parser::{ACC_FINAL, ACC_STATIC};
use crate::runtime::frame::operand_stack::Operand;
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
    index: usize,
}

impl Field {
    pub fn new(const_pool: &ConstPool, field: &FieldInfo, index: usize) -> Field {
        let name = const_pool.get_utf8_string_at(field.name_index).to_string();
        let descriptor = const_pool
            .get_utf8_string_at(field.descriptor_index)
            .to_string();

        Field {
            inner: Arc::new(InnerField {
                access_flags: field.access_flags,
                name,
                descriptor,
                index,
            }),
        }
    }

    pub fn access_flags(&self) -> u16 {
        self.inner.access_flags
    }

    pub fn descriptor(&self) -> String {
        self.inner.descriptor.clone()
    }

    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    pub fn index(&self) -> usize {
        self.inner.index
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

    pub fn default_value(&self) -> Operand {
        let descriptor = self.descriptor();
        match descriptor.as_bytes()[0] {
            b'B' => Operand::Byte(0),
            b'C' => Operand::Char(0),
            b'D' => Operand::Double(0.0),
            b'F' => Operand::Float(0.0),
            b'I' => Operand::Int(0),
            b'J' => Operand::Long(0),
            b'S' => Operand::Short(0),
            b'Z' => Operand::Int(0),
            b'L' | b'[' => Operand::Null,
            _ => unreachable!("{}", descriptor),
        }
    }
}
