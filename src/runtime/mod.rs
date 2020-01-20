mod class;
mod class_loader;
mod field;
mod method;
mod opcode;

use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::method::Method;
use nom::lib::std::collections::VecDeque;
use std::sync::Arc;

pub type JvmPC = usize;

#[derive(Debug, Clone)]
enum Operand {
    Int(i32),
    Float(f32),
    Double(f64),
    Long(i64),
    Str(u16),
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

    fn push(&mut self, val: Operand) {
        self.stack.push(val)
    }

    fn push_integer(&mut self, num: i32) {
        self.push(Operand::Int(num))
    }

    fn push_float(&mut self, num: f32) {
        self.push(Operand::Float(num))
    }

    fn push_object_ref(&mut self, reference: u16) {
        self.push(Operand::ObjectRef(reference))
    }

    fn push_class_ref(&mut self, class: String) {
        self.push(Operand::ClassRef(class))
    }

    fn pop(&mut self) -> Operand {
        self.stack.pop().unwrap()
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
    class_loader: ClassLoader,
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
            class_loader: ClassLoader::new(ClassPath::new(None, None)),
            main_class: class_name.to_string(),
        };
        jvm
    }

    pub fn run(&mut self) {
        let class = load_and_init_class(
            &mut self.thread,
            &mut self.class_loader,
            self.main_class.clone(),
        );
        let main_method = class.main_method().expect("find main method");
        execute_method(
            &mut self.thread,
            &mut self.class_loader,
            class,
            main_method,
            None,
        );
    }
}

fn load_and_init_class(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    class_name: String,
) -> Class {
    let class = class_loader.load_class(class_name);
    if !class.is_inited() {
        let cinit_method = class.cinit_method();
        execute_method(thread, class_loader, class.clone(), cinit_method, None);
    }
    class
}

//noinspection ALL
fn execute_method(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    class: Class,
    method: Method,
    args: Option<Vec<Operand>>,
) {
    let frame = JvmFrame::new(&method);
    thread.stack.frames.push_back(frame);
    {
        let frame = thread.stack.frames.back_mut().unwrap();
        if let Some(args) = args {
            for arg in args {
                frame.operand_stack.push(arg)
            }
        }
    }

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

        match code {
            opcode::ICONST_0 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                frame.operand_stack.push_integer(0);
            }
            opcode::LDC => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let index = code_reader.read_u8().unwrap();
                let const_pool_info = class.constant_pool().get_const_pool_info_at(index as u16);
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
                        let _class = class_loader.load_class(name.clone());
                        frame.operand_stack.push_class_ref(name.clone());
                    }
                    ConstPoolInfo::ConstantMethodHandleInfo { .. } => unimplemented!(),
                    ConstPoolInfo::ConstantMethodTypeInfo { .. } => unimplemented!(),
                    _ => unreachable!(),
                }
            }
            opcode::ISTORE_0 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.operand_stack.pop_integer();
                frame.local_variable_array.set_integer(0, val);
            }
            opcode::ISTORE_1 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.operand_stack.pop_integer();
                frame.local_variable_array.set_integer(1, val);
            }
            opcode::ISTORE_2 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.operand_stack.pop_integer();
                frame.local_variable_array.set_integer(2, val);
            }
            opcode::ISTORE_3 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.operand_stack.pop_integer();
                frame.local_variable_array.set_integer(3, val);
            }
            opcode::ISTORE => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let index = code_reader.read_u8().unwrap();
                let val = frame.operand_stack.pop_integer();
                frame.local_variable_array.set_integer(index as u16, val);
            }
            opcode::ASTORE_2 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let object_ref = frame.operand_stack.pop_object_reference();
                frame.local_variable_array.set_object_ref(2, object_ref);
            }
            opcode::BIPUSH => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let byte = code_reader.read_u8().unwrap();
                frame.operand_stack.push_integer(byte as i32);
            }
            opcode::ILOAD_0 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.local_variable_array.get_integer(0);
                frame.operand_stack.push_integer(val);
            }
            opcode::ILOAD_1 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.local_variable_array.get_integer(1);
                frame.operand_stack.push_integer(val);
            }
            opcode::ILOAD_2 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.local_variable_array.get_integer(2);
                frame.operand_stack.push_integer(val);
            }
            opcode::ILOAD_3 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.local_variable_array.get_integer(3);
                frame.operand_stack.push_integer(val);
            }
            opcode::IADD => {
                let frame = thread.stack.frames.back_mut().unwrap();
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
                        let class_name = class.constant_pool().get_class_name_at(*class_index);
                        let class = load_and_init_class(thread, class_loader, class_name.clone());
                        let (method_name, method_type) = class
                            .constant_pool()
                            .get_name_and_type_at(*name_and_type_index);

                        let method = class
                            .get_user_method(method_name, method_type, true)
                            .expect("get method");

                        let frame = thread.stack.frames.back_mut().unwrap();
                        let n_args = method.parameters().len();
                        let mut args = Vec::with_capacity(n_args);
                        for _ in 0..n_args {
                            args.push(frame.operand_stack.pop());
                        }
                        execute_method(thread, class_loader, class, method, Some(args));
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
                let frame = thread.stack.frames.back_mut().unwrap();
                let val = frame.operand_stack.pop_integer();
                let _ = thread.stack.frames.pop_back();
                let last_frame = thread.stack.frames.back_mut().unwrap();
                last_frame.operand_stack.push_integer(val);
                break;
            }
            opcode::RETURN => {
                let _ = thread.stack.frames.pop_back();
                break;
            }
            opcode::PUTSTATIC => {}
            op @ _ => unimplemented!("{:#x}", op),
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
