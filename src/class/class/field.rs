use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::field_info::FieldInfo;
use crate::class_parser::{ACC_FINAL, ACC_STATIC};
use crate::gc::oop::InstanceOop;
use crate::operand::Operand;

#[derive(Debug, Clone)]
pub struct Field {
    access_flags: u16,
    name: String,
    descriptor: String,
    offset: usize,
    size: usize,
    loader: InstanceOop,
}

impl Field {
    pub fn new(
        name: String,
        descriptor: String,
        access_flags: u16,
        size: usize,
        offset: usize,
        loader: InstanceOop,
    ) -> Field {
        Field {
            access_flags,
            name,
            descriptor,
            offset,
            size,
            loader,
        }
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset
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

pub fn descriptor_size_in_bytes(descriptor: &str) -> usize {
    match descriptor.as_bytes()[0] {
        b'B' => 1,
        b'C' => 2,
        b'D' => 8,
        b'F' => 4,
        b'I' => 4,
        b'J' => 8,
        b'S' => 2,
        b'Z' => 4,
        b'L' | b'[' => 8,
        _ => unreachable!("{}", descriptor),
    }
}
