use crate::runtime::frame::operand_stack::Operand;

#[derive(Debug)]
pub struct LocalVariableArray {
    local_variables: Vec<Operand>,
}

impl LocalVariableArray {
    pub fn new(size: usize) -> Self {
        LocalVariableArray {
            local_variables: vec![Operand::Null; size],
        }
    }

    pub fn new_with_args(size: usize, args: Vec<Operand>) -> Self {
        let mut local_variables = Vec::with_capacity(size);
        for arg in args {
            match arg {
                v @ Operand::Long(_) | v @ Operand::Double(_) => {
                    local_variables.push(v);
                    local_variables.push(Operand::Null);
                }
                v => {
                    local_variables.push(v);
                }
            }
        }

        local_variables.resize(size, Operand::Null);
        LocalVariableArray { local_variables }
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

    pub fn get_long(&mut self, index: u16) -> i64 {
        match self.local_variables[index as usize] {
            Operand::Long(num) => num,
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

    pub fn set_object_ref_addr(&mut self, index: u16, value: u32) {
        self.local_variables[index as usize] = Operand::ObjectRef(value);
    }

    pub fn set(&mut self, index: u16, value: Operand) {
        self.local_variables[index as usize] = value;
    }

    pub fn get_object(&mut self, index: u16) -> Operand {
        match &self.local_variables[index as usize] {
            v @ Operand::ObjectRef(_) | v @ Operand::Null | v @ Operand::ArrayRef(_) => v.clone(),
            v => unreachable!("{:?}", v),
        }
    }
}
