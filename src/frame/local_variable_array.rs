use crate::gc::global_definition::{JObject, JValue};
use crate::gc::oop::Oop;

#[derive(Debug)]
pub struct LocalVariableArray {
    local_variables: Vec<JValue>,
}

impl LocalVariableArray {
    pub fn new(size: usize) -> Self {
        LocalVariableArray {
            local_variables: vec![JValue::default(); size],
        }
    }

    pub fn new_with_args(size: usize, args: Vec<JValue>) -> Self {
        let mut local_variables = Vec::with_capacity(size);
        for arg in args {
            match arg {
                v @ JValue::Long(_) | v @ JValue::Double(_) => {
                    local_variables.push(v);
                    local_variables.push(JValue::default());
                }
                v => {
                    local_variables.push(v);
                }
            }
        }

        local_variables.resize(size, JValue::default());
        LocalVariableArray { local_variables }
    }

    pub fn set_integer(&mut self, index: u16, value: i32) {
        self.local_variables[index as usize] = JValue::Int(value);
    }

    pub fn get_integer(&mut self, index: u16) -> i32 {
        match self.local_variables[index as usize] {
            JValue::Int(num) => num,
            _ => unreachable!(),
        }
    }

    pub fn get_long(&mut self, index: u16) -> i64 {
        match self.local_variables[index as usize] {
            JValue::Long(num) => num,
            _ => unreachable!(),
        }
    }

    pub fn set_float(&mut self, index: u16, value: f32) {
        self.local_variables[index as usize] = JValue::Float(value);
    }

    pub fn get_float(&mut self, index: u16) -> f32 {
        match self.local_variables[index as usize] {
            JValue::Float(num) => num,
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, index: u16, value: JValue) {
        self.local_variables[index as usize] = value;
    }

    pub fn get_jobject(&mut self, index: u16) -> JValue {
        match &self.local_variables[index as usize] {
            v @ JValue::Object(_) => v.clone(),
            v => unreachable!("{:?}", v),
        }
    }
}
