mod class;
mod class_loader;
mod field;
mod method;
mod opcode;

use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::class_loader::Classloader;
use crate::runtime::method::Method;
use nom::lib::std::collections::VecDeque;
use std::sync::Arc;

pub type JvmPC = usize;

#[derive(Debug, Clone)]
enum Operand {
    Int(i32),
    Float(f32),
    ObjectRef(u16),
    ClassRef(String),
}

#[derive(Debug)]
struct OperandStack {
    stack: Vec<Operand>,
}

impl OperandStack {
    fn new() -> Self {
        OperandStack { stack: Vec::new() }
    }

    fn with_capacity(cap: usize) -> Self {
        OperandStack {
            stack: Vec::with_capacity(cap),
        }
    }

    fn push_integer(&mut self, num: i32) {
        self.stack.push(Operand::Int(num))
    }

    fn push_float(&mut self, num: f32) {
        self.stack.push(Operand::Float(num))
    }

    fn push_object_ref(&mut self, reference: u16) {
        self.stack.push(Operand::ObjectRef(reference))
    }

    fn push_class_ref(&mut self, class: String) {
        self.stack.push(Operand::ClassRef(class))
    }

    fn pop_integer(&mut self) -> i32 {
        match self.stack.pop() {
            Some(Operand::Int(num)) => num,
            _ => unreachable!(),
        }
    }

    fn pop_float(&mut self) -> f32 {
        match self.stack.pop() {
            Some(Operand::Float(num)) => num,
            _ => unreachable!(),
        }
    }

    fn pop_object_reference(&mut self) -> u16 {
        match self.stack.pop() {
            Some(Operand::ObjectRef(num)) => num,
            _ => unreachable!(),
        }
    }

    fn clear(&mut self) {
        self.stack.clear();
    }
}

#[derive(Debug)]
struct LocalVariableArray {
    local_variables: Vec<Operand>,
}

impl LocalVariableArray {
    fn set_integer(&mut self, index: u16, value: i32) {
        self.local_variables[index as usize] = Operand::Int(value);
    }

    fn get_integer(&mut self, index: u16) -> i32 {
        match self.local_variables[index as usize] {
            Operand::Int(num) => num,
            _ => unreachable!(),
        }
    }

    fn set_float(&mut self, index: u16, value: f32) {
        self.local_variables[index as usize] = Operand::Float(value);
    }

    fn get_float(&mut self, index: u16) -> f32 {
        match self.local_variables[index as usize] {
            Operand::Float(num) => num,
            _ => unreachable!(),
        }
    }

    fn set_object_ref(&mut self, index: u16, value: u16) {
        self.local_variables[index as usize] = Operand::ObjectRef(value);
    }

    fn get_object_ref(&mut self, index: u16) -> u16 {
        match self.local_variables[index as usize] {
            Operand::ObjectRef(val) => val,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct JvmFrame {
    local_variable_array: LocalVariableArray,
    operand_stack: OperandStack,
}

impl JvmFrame {
    fn new(method: &Method) -> Self {
        JvmFrame {
            local_variable_array: LocalVariableArray {
                local_variables: vec![Operand::Int(0); method.max_locals()],
            },
            operand_stack: OperandStack::with_capacity(method.max_stack()),
        }
    }
}

#[derive(Debug)]
pub struct JvmStack {
    frames: VecDeque<JvmFrame>,
}

#[derive(Debug)]
pub struct JvmThread {
    stack: JvmStack,
    pc: JvmPC,
}

#[derive(Debug)]
pub struct JvmHeap {}

#[derive(Debug)]
struct Jvm {
    heap: JvmHeap,
    thread: JvmThread,
    class_loader: Classloader,
    main_class: String,
}

#[derive(Debug)]
struct CodeReader {
    code: Arc<Vec<u8>>,
    pc: usize,
}

impl CodeReader {
    fn read_u8(&mut self) -> Option<u8> {
        let code = self.code.get(self.pc as usize).cloned();
        self.pc += 1;
        code
    }

    fn read_u16(&mut self) -> Option<u16> {
        let byte1 = *self.code.get(self.pc as usize)? as u16;
        self.pc += 1;
        let byte2 = *self.code.get(self.pc as usize)? as u16;
        self.pc += 1;
        Some(byte1 << 8 | byte2)
    }
}

impl Jvm {
    pub fn new(class_name: &str) -> Self {
        let jvm = Jvm {
            heap: JvmHeap {},
            thread: JvmThread {
                stack: JvmStack {
                    frames: Default::default(),
                },
                pc: 0,
            },
            class_loader: Classloader::new(ClassPath::new(None, None)),
            main_class: class_name.to_string(),
        };
        jvm
    }

    pub fn run(&mut self) {
        let class = self.class_loader.load_class(self.main_class.clone());
        let main_method = class.main_method().expect("find main method");
        let frame = JvmFrame::new(&main_method);
        self.thread.stack.frames.push_back(frame);
        self.execute_method(class, main_method);
    }

    fn execute_method(&mut self, class: Class, method: Method) {
        let frame = self.thread.stack.frames.back_mut().unwrap();

        let mut code_reader = CodeReader {
            code: method.code(),
            pc: 0,
        };
        loop {
            let code = if let Some(code) = code_reader.read_u8() {
                code
            } else {
                break;
            };
            self.thread.pc = code_reader.pc;

            match code {
                opcode::ICONST_0 => {
                    frame.operand_stack.push_integer(0);
                }
                opcode::LDC => {
                    let index = code_reader.read_u8().unwrap();
                    let const_pool_info =
                        class.constant_pool().get_const_pool_info_at(index as u16);
                    match const_pool_info {
                        ConstPoolInfo::ConstantIntegerInfo(num) => {
                            frame.operand_stack.push_integer(*num);
                        }
                        ConstPoolInfo::ConstantFloatInfo(num) => {
                            frame.operand_stack.push_float(*num);
                        }
                        ConstPoolInfo::ConstantStringInfo { string_index } => {
                            frame.operand_stack.push_object_ref(*string_index)
                        }
                        ConstPoolInfo::ConstantClassInfo { name_index } => {
                            let name = class.constant_pool().get_utf8_string_at(*name_index);
                            let _class = self.class_loader.load_class(name.clone());
                            frame.operand_stack.push_class_ref(name.clone());
                        }
                        ConstPoolInfo::ConstantMethodHandleInfo { .. } => unimplemented!(),
                        ConstPoolInfo::ConstantMethodTypeInfo { .. } => unimplemented!(),
                        _ => unreachable!(),
                    }
                }
                opcode::ISTORE_0 => {
                    let val = frame.operand_stack.pop_integer();
                    frame.local_variable_array.set_integer(0, val);
                }
                opcode::ISTORE_1 => {
                    let val = frame.operand_stack.pop_integer();
                    frame.local_variable_array.set_integer(1, val);
                }
                opcode::ISTORE_2 => {
                    let val = frame.operand_stack.pop_integer();
                    frame.local_variable_array.set_integer(2, val);
                }
                opcode::ISTORE_3 => {
                    let val = frame.operand_stack.pop_integer();
                    frame.local_variable_array.set_integer(3, val);
                }
                opcode::ISTORE => {
                    let index = code_reader.read_u8().unwrap();
                    let val = frame.operand_stack.pop_integer();
                    frame.local_variable_array.set_integer(index as u16, val);
                }
                opcode::ASTORE_2 => {
                    let object_ref = frame.operand_stack.pop_object_reference();
                    frame.local_variable_array.set_object_ref(2, object_ref);
                }
                opcode::BIPUSH => {
                    let byte = code_reader.read_u8().unwrap();
                    frame.operand_stack.push_integer(byte as i32);
                }
                opcode::ILOAD_0 => {
                    let val = frame.local_variable_array.get_integer(0);
                    frame.operand_stack.push_integer(val);
                }
                opcode::ILOAD_1 => {
                    let val = frame.local_variable_array.get_integer(1);
                    frame.operand_stack.push_integer(val);
                }
                opcode::ILOAD_2 => {
                    let val = frame.local_variable_array.get_integer(2);
                    frame.operand_stack.push_integer(val);
                }
                opcode::ILOAD_3 => {
                    let val = frame.local_variable_array.get_integer(3);
                    frame.operand_stack.push_integer(val);
                }
                opcode::IADD => {
                    let val1 = frame.operand_stack.pop_integer();
                    let val2 = frame.operand_stack.pop_integer();
                    frame.operand_stack.push_integer(val1 + val2);
                }
                opcode::INVOKESTATIC => {
                    let index = code_reader.read_u16().unwrap();
                    let const_pool_info = class.constant_pool().get_const_pool_info_at(index);
                    match const_pool_info {
                        ConstPoolInfo::ConstantMethodRefInfo {
                            class_index,
                            name_and_type_index,
                        } => {
                            // todo page 486
                        }
                        ConstPoolInfo::ConstantInterfaceMethodRefInfo {
                            class_index,
                            name_and_type_index,
                        } => {
                            // todo page 486
                        }
                        _ => unreachable!(),
                    }
                }
                opcode::IRETURN => {
                    let val = frame.operand_stack.pop_integer();
                    let _ = self.thread.stack.frames.pop_back();
                    let last_frame = self.thread.stack.frames.back_mut().unwrap();
                    last_frame.operand_stack.push_integer(val);
                    break;
                }
                opcode::RETURN => {
                    let _ = self.thread.stack.frames.pop_back();
                    break;
                }
                op @ _ => unimplemented!("{:#x}", op),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_jvm() {
        let mut jvm = Jvm::new("Main");
        jvm.run();
    }
}
