pub mod local_variable_array;
pub mod operand_stack;

use crate::runtime::frame::local_variable_array::LocalVariableArray;
use crate::runtime::frame::operand_stack::OperandStack;
use crate::runtime::method::Method;

#[derive(Debug)]
pub struct JvmFrame {
    pub local_variable_array: LocalVariableArray,
    pub operand_stack: OperandStack,
}

impl JvmFrame {
    pub fn new(method: &Method) -> Self {
        JvmFrame {
            local_variable_array: LocalVariableArray::new(method.max_locals()),
            operand_stack: OperandStack::with_capacity(method.max_stack()),
        }
    }
}
