pub mod local_variable_array;
pub mod operand_stack;

use crate::runtime::frame::local_variable_array::LocalVariableArray;
use crate::runtime::frame::operand_stack::{Operand, OperandStack};
use crate::runtime::method::Method;

#[derive(Debug)]
pub struct JvmFrame {
    pub local_variable_array: LocalVariableArray,
    pub operand_stack: OperandStack,
    pub method: Method,
}

impl JvmFrame {
    pub fn new(method: &Method) -> Self {
        JvmFrame {
            local_variable_array: LocalVariableArray::new(method.max_locals()),
            operand_stack: OperandStack::with_capacity(method.max_stack()),
            method: method.clone(),
        }
    }

    pub fn new_with_args(method: &Method, args: Vec<Operand>) -> Self {
        JvmFrame {
            local_variable_array: LocalVariableArray::new_with_args(method.max_locals(), args),
            operand_stack: OperandStack::with_capacity(method.max_stack()),
            method: method.clone(),
        }
    }
}
