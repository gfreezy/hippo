mod class;
mod class_loader;
mod code_reader;
mod field;
mod frame;
mod instruction;
mod method;
mod opcode;

use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::frame::JvmFrame;
use crate::runtime::instruction::*;
use crate::runtime::method::Method;
use std::collections::VecDeque;
use tracing::debug;

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
    tracing::debug!(%class_name, "load_and_init_class");
    let class = class_loader.load_class(class_name);
    if !dbg!(class.is_inited()) {
        class.set_inited();
        tracing::debug!(%class, "init class");
        let clinit_method = class.cinit_method();
        if let Some(clinit_method) = clinit_method {
            execute_method(thread, class_loader, class.clone(), clinit_method, None);
        }
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
    let span = tracing::debug_span!("execute_method", %method, %class);
    let _ = span.enter();
    let frame = JvmFrame::new(&method);
    thread.stack.frames.push_back(frame);
    let frame = thread.stack.frames.back_mut().unwrap();
    if let Some(args) = args {
        for arg in args {
            frame.operand_stack.push(arg)
        }
    }

    let mut code_reader = CodeReader::new(method.code());
    while let Some(code) = code_reader.read_u8() {
        debug!(?code);
        match code {
            opcode::ICONST_0 => {
                iconst_n(thread, class_loader, &mut code_reader, class.clone(), 0);
            }
            opcode::ICONST_1 => {
                iconst_n(thread, class_loader, &mut code_reader, class.clone(), 1);
            }
            opcode::ICONST_2 => {
                iconst_n(thread, class_loader, &mut code_reader, class.clone(), 2);
            }
            opcode::ICONST_3 => {
                iconst_n(thread, class_loader, &mut code_reader, class.clone(), 3);
            }
            opcode::ICONST_4 => {
                iconst_n(thread, class_loader, &mut code_reader, class.clone(), 4);
            }
            opcode::ICONST_5 => {
                iconst_n(thread, class_loader, &mut code_reader, class.clone(), 5);
            }
            opcode::LDC => ldc(thread, class_loader, &mut code_reader, class.clone()),
            opcode::ISTORE_0 => {
                istore_n(thread, class_loader, &mut code_reader, class.clone(), 0);
            }
            opcode::ISTORE_1 => {
                istore_n(thread, class_loader, &mut code_reader, class.clone(), 1);
            }
            opcode::ISTORE_2 => {
                istore_n(thread, class_loader, &mut code_reader, class.clone(), 2);
            }
            opcode::ISTORE_3 => {
                istore_n(thread, class_loader, &mut code_reader, class.clone(), 3);
            }
            opcode::ISTORE => {
                istore(thread, class_loader, &mut code_reader, class.clone());
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
                iload_n(thread, class_loader, &mut code_reader, class.clone(), 0);
            }
            opcode::ILOAD_1 => {
                iload_n(thread, class_loader, &mut code_reader, class.clone(), 1);
            }
            opcode::ILOAD_2 => {
                iload_n(thread, class_loader, &mut code_reader, class.clone(), 2);
            }
            opcode::ILOAD_3 => {
                iload_n(thread, class_loader, &mut code_reader, class.clone(), 3);
            }
            opcode::IADD => {
                iadd(thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::INVOKESTATIC => {
                invokestatic(thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::IRETURN => {
                ireturn(thread, class_loader, &mut code_reader, class.clone());
                break;
            }
            opcode::RETURN => {
                return_(thread, class_loader, &mut code_reader, class.clone());
                break;
            }
            opcode::NOP => {}
            opcode::GETSTATIC => {
                getstatic(thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::ACONST_NULL => {
                aconst_null(thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::PUTSTATIC => {
                putstatic(thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::INVOKEVIRTUAL => {
                invokevirtual(thread, class_loader, &mut code_reader, class.clone());
            }
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
