pub mod local_variable_array;
pub mod operand_stack;

use crate::runtime::class::Class;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::frame::local_variable_array::LocalVariableArray;
use crate::runtime::frame::operand_stack::{Operand, OperandStack};
use crate::runtime::method::Method;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct JvmFrame {
    pub local_variable_array: LocalVariableArray,
    pub operand_stack: OperandStack,
    pub method: Method,
    pub class: Class,
    #[derivative(Debug = "ignore")]
    pub code_reader: CodeReader,
}

impl JvmFrame {
    pub fn new_with_args(class: Class, method: Method, args: Vec<Operand>) -> Self {
        JvmFrame {
            local_variable_array: LocalVariableArray::new_with_args(method.max_locals(), args),
            operand_stack: OperandStack::with_capacity(method.max_stack()),
            method: method.clone(),
            class,
            code_reader: CodeReader::new(method),
        }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        self.code_reader.read_u8()
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        self.code_reader.read_u16()
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        self.code_reader.read_i16()
    }

    pub fn pc(&self) -> usize {
        self.code_reader.pc()
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.code_reader.set_pc(pc)
    }
}
