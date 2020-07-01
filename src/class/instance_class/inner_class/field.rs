use crate::class::alloc_jobject;
use crate::class_loader::load_class;
use crate::class_parser::{JVM_ACC_FINAL, JVM_ACC_PUBLIC, JVM_ACC_STATIC};
use crate::gc::global_definition::{BasicType, JObject, JValue};
use crate::java_const::JAVA_LANG_REFLECT_FIELD;

#[derive(Debug, Clone)]
pub struct Field {
    access_flags: u16,
    name: String,
    descriptor: String,
    offset: usize,
    size: usize,
    loader: JObject,
}

impl Field {
    pub fn new(
        name: String,
        descriptor: String,
        access_flags: u16,
        size: usize,
        offset: usize,
        loader: JObject,
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

    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    pub fn type_class(&self) -> &str {
        &self.descriptor[1..self.descriptor.len() - 1]
    }

    pub fn basic_type(&self) -> BasicType {
        self.descriptor.as_str().into()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn class_loader(&self) -> JObject {
        self.loader
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
        self.access_flags() & JVM_ACC_STATIC != 0
    }

    pub fn is_public(&self) -> bool {
        self.access_flags() & JVM_ACC_PUBLIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags() & JVM_ACC_FINAL != 0
    }

    pub fn default_value(&self) -> JValue {
        let descriptor = self.descriptor();
        match descriptor.as_bytes()[0] {
            b'B' => JValue::Byte(0),
            b'C' => JValue::Char(0),
            b'D' => JValue::Double(0.0),
            b'F' => JValue::Float(0.0),
            b'I' => JValue::Int(0),
            b'J' => JValue::Long(0),
            b'S' => JValue::Short(0),
            b'Z' => JValue::Int(0),
            b'L' => JValue::Object(JObject::null()),
            b'[' => JValue::Object(JObject::null()),
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
