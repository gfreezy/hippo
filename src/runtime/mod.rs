mod class;
mod class_loader;
mod code_reader;
mod field;
mod frame;
mod method;
mod opcode;

use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::frame::JvmFrame;
use crate::runtime::method::Method;
use std::collections::VecDeque;

pub type JvmPC = usize;

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
pub struct Jvm {
    heap: JvmHeap,
    thread: JvmThread,
    class_loader: ClassLoader,
    main_class: String,
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

    let mut code_reader = CodeReader::new(method.code());
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
            opcode::ICONST_1 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                frame.operand_stack.push_integer(1);
            }
            opcode::ICONST_2 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                frame.operand_stack.push_integer(2);
            }
            opcode::ICONST_3 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                frame.operand_stack.push_integer(3);
            }
            opcode::ICONST_4 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                frame.operand_stack.push_integer(4);
            }
            opcode::ICONST_5 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                frame.operand_stack.push_integer(5);
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
                    }
                    | ConstPoolInfo::ConstantInterfaceMethodRefInfo {
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
            opcode::NOP => {}
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
