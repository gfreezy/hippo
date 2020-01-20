use crate::runtime::frame::operand_stack::Operand;

#[derive(Debug)]
pub struct LocalVariableArray {
    local_variables: Vec<Operand>,
}

impl LocalVariableArray {
    pub fn new(size: usize) -> Self {
        LocalVariableArray {
            local_variables: vec![Operand::Int(0); size],
        }
    }
    pub fn set_integer(&mut self, index: u16, value: i32) {
        self.local_variables[index as usize] = Operand::Int(value);
    }

    pub fn get_integer(&mut self, index: u16) -> i32 {
        match self.local_variables[index as usize] {
            Operand::Int(num) => num,
            _ => unreachable!(),
        }
    }

    pub fn set_float(&mut self, index: u16, value: f32) {
        self.local_variables[index as usize] = Operand::Float(value);
    }

    pub fn get_float(&mut self, index: u16) -> f32 {
        match self.local_variables[index as usize] {
            Operand::Float(num) => num,
            _ => unreachable!(),
        }
    }

    pub fn set_object_ref(&mut self, index: u16, value: u16) {
        self.local_variables[index as usize] = Operand::ObjectRef(value);
    }

    pub fn get_object_ref(&mut self, index: u16) -> u16 {
        match self.local_variables[index as usize] {
            Operand::ObjectRef(val) => val,
            _ => unreachable!(),
        }
    }
}
