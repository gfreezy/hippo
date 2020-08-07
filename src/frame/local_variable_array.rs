use crate::gc::global_definition::{JInt, JLong, JValue};
use nom::lib::std::fmt::Formatter;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
#[serde(transparent)]
pub struct LocalVariableArray {
    local_variables: Vec<JValue>,
}

impl fmt::Debug for LocalVariableArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LocalVariableArray {{ ")?;
        for (i, local_variable) in self.local_variables.iter().enumerate() {
            write!(f, "{}: {:?}", i, local_variable)?;
            if i != self.local_variables.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
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

    pub fn set_jint(&mut self, index: u16, value: JInt) {
        self.local_variables[index as usize] = JValue::Int(value);
    }

    pub fn get_jint(&mut self, index: u16) -> i32 {
        match self.local_variables[index as usize] {
            JValue::Int(num) => num,
            _ => unreachable!(),
        }
    }
    pub fn set_jlong(&mut self, index: u16, value: JLong) {
        self.local_variables[index as usize] = JValue::Long(value);
    }

    pub fn get_jlong(&mut self, index: u16) -> i64 {
        match self.local_variables[index as usize] {
            JValue::Long(num) => num,
            _ => unreachable!(),
        }
    }

    pub fn set_jfloat(&mut self, index: u16, value: f32) {
        self.local_variables[index as usize] = JValue::Float(value);
    }

    pub fn get_jfloat(&mut self, index: u16) -> f32 {
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
