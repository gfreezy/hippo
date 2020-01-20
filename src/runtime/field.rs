use crate::class_parser::constant_pool::ConstPool;
use crate::class_parser::field_info::FieldInfo;
use crate::class_parser::{ACC_FINAL, ACC_STATIC};
use crate::runtime::frame::operand_stack::Operand;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Field {
    inner: Arc<Mutex<InnerField>>,
}

#[derive(Debug)]
struct InnerField {
    access_flags: u16,
    name: String,
    descriptor: String,
    value: Option<Operand>,
}

impl Field {
    pub fn new(const_pool: &ConstPool, filed_info: FieldInfo) -> Field {
        let constant_value_index = filed_info
            .constant_value_attribute()
            .map(|attr| attr.constant_value_index);
        let name = const_pool
            .get_utf8_string_at(filed_info.name_index)
            .to_string();
        let descriptor = const_pool
            .get_utf8_string_at(filed_info.descriptor_index)
            .to_string();
        let value = if filed_info.access_flags & ACC_STATIC != 0
            && filed_info.access_flags & ACC_FINAL != 0
        {
            let constant_value_index = constant_value_index.unwrap();
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
        };
        Field {
            inner: Arc::new(Mutex::new(InnerField {
                access_flags: filed_info.access_flags,
                name,
                descriptor,
                value,
            })),
        }
    }

    pub fn access_flags(&self) -> u16 {
        self.inner.lock().unwrap().access_flags
    }

    pub fn descriptor(&self) -> String {
        self.inner.lock().unwrap().descriptor.clone()
    }

    pub fn name(&self) -> String {
        self.inner.lock().unwrap().name.clone()
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
