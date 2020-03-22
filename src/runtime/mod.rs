mod class;
mod class_loader;
mod code_reader;
mod field;
mod frame;
mod heap;
mod instruction;
mod method;
mod opcode;

use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::frame::JvmFrame;
use crate::runtime::heap::JvmHeap;
use crate::runtime::instruction::*;
use crate::runtime::method::Method;
use crate::runtime::opcode::show_opcode;
use std::collections::VecDeque;
use tracing::{debug, debug_span};

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
pub struct Jvm {
    heap: JvmHeap,
    thread: JvmThread,
    class_loader: ClassLoader,
    main_class: String,
}

impl Jvm {
    pub fn new(class_name: &str) -> Self {
        let mut jvm = Jvm {
            heap: JvmHeap::new(),
            thread: JvmThread {
                stack: JvmStack {
                    frames: Default::default(),
                },
                pc: 0,
            },
            class_loader: ClassLoader::new(ClassPath::new(None, None)),
            main_class: class_name.to_string(),
        };
        let system_class = load_and_init_class(
            &mut jvm.heap,
            &mut jvm.thread,
            &mut jvm.class_loader,
            "java/lang/System",
        );
        let system_class_initialize = system_class
            .get_method("initializeSystemClass", "()V", true)
            .expect("system init");
        execute_method(
            &mut jvm.heap,
            &mut jvm.thread,
            &mut jvm.class_loader,
            system_class_initialize,
            vec![],
        );
        jvm
    }

    pub fn run(&mut self) {
        let class = load_and_init_class(
            &mut self.heap,
            &mut self.thread,
            &mut self.class_loader,
            &self.main_class,
        );
        let main_method = class.main_method().expect("find main method");
        execute_method(
            &mut self.heap,
            &mut self.thread,
            &mut self.class_loader,
            main_method,
            vec![],
        );
    }
}

fn load_and_init_class(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    class_name: &str,
) -> Class {
    let class = class_loader.load_class(class_name);
    if !class.is_inited() {
        let span = debug_span!("init_class", %class_name);
        let _s = span.enter();
        class.set_inited();
        debug!("init successfully.");
        let clinit_method = class.clinit_method();
        if let Some(clinit_method) = clinit_method {
            execute_method(heap, thread, class_loader, clinit_method, vec![]);
        }
    }
    class
}

fn did_override_method(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    method: &Method,
    other: &Method,
) -> bool {
    if method == other {
        return true;
    }
    let this_class = load_and_init_class(heap, thread, class_loader, method.name());
    let other_class = load_and_init_class(heap, thread, class_loader, other.name());
    if !this_class.is_subclass_of(other_class) {
        return false;
    }
    if method.name() != other.name() {
        return false;
    }
    if method.descriptor() != other.descriptor() {
        return false;
    }
    if method.is_private() {
        return false;
    }
    if (other.is_protected() || other.is_public())
        || (!other.is_public() && !other.is_private() && !other.is_protected())
    {
        return true;
    }

    false
}

//noinspection ALL
fn execute_method(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    method: Method,
    args: Vec<Operand>,
) {
    let is_native = method.is_native();
    let class = load_and_init_class(heap, thread, class_loader, method.class_name());

    let span = tracing::debug_span!("execute_method", %class, %method, is_native);
    let _span = span.enter();

    if is_native {
        debug!("skip native method");
        if method.return_descriptor() != "V" {
            panic!("native method returns {}", method.return_descriptor());
        }
        return;
    }

    let frame = JvmFrame::new_with_args(&method, args);
    thread.stack.frames.push_back(frame);

    let mut code_reader = CodeReader::new(method.code());
    while let Some(code) = code_reader.read_u8() {
        let frame = thread.stack.frames.back().unwrap();
        debug!(
            pc = code_reader.pc(),
            opcode = opcode::show_opcode(code),
            ?frame
        );
        match code {
            opcode::ICONST_0 => {
                iconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    0,
                );
            }
            opcode::ICONST_1 => {
                iconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    1,
                );
            }
            opcode::ICONST_2 => {
                iconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    2,
                );
            }
            opcode::ICONST_3 => {
                iconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    3,
                );
            }
            opcode::ICONST_4 => {
                iconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    4,
                );
            }
            opcode::ICONST_5 => {
                iconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    5,
                );
            }
            opcode::FCONST_0 => {
                fconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    0.0,
                );
            }
            opcode::FCONST_1 => {
                fconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    1.0,
                );
            }
            opcode::FCONST_2 => {
                fconst_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    2.0,
                );
            }
            opcode::LDC => ldc(heap, thread, class_loader, &mut code_reader, class.clone()),
            opcode::ISTORE_0 => {
                istore_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    0,
                );
            }
            opcode::ISTORE_1 => {
                istore_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    1,
                );
            }
            opcode::ISTORE_2 => {
                istore_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    2,
                );
            }
            opcode::ISTORE_3 => {
                istore_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    3,
                );
            }
            opcode::ISTORE => {
                istore(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::ASTORE_2 => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let object_ref = frame.operand_stack.pop_object_ref();
                frame
                    .local_variable_array
                    .set_object_ref_addr(2, object_ref);
            }
            opcode::BIPUSH => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let byte = code_reader.read_u8().unwrap();
                frame.operand_stack.push_integer(byte as i32);
            }
            opcode::ILOAD_0 => {
                iload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    0,
                );
            }
            opcode::ILOAD_1 => {
                iload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    1,
                );
            }
            opcode::ILOAD_2 => {
                iload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    2,
                );
            }
            opcode::ILOAD_3 => {
                iload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    3,
                );
            }
            opcode::ALOAD_0 => {
                aload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    0,
                );
            }
            opcode::ALOAD_1 => {
                aload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    1,
                );
            }
            opcode::ALOAD_2 => {
                aload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    2,
                );
            }
            opcode::ALOAD_3 => {
                aload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    3,
                );
            }
            opcode::FLOAD_0 => {
                fload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    0,
                );
            }
            opcode::FLOAD_1 => {
                fload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    1,
                );
            }
            opcode::FLOAD_2 => {
                fload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    2,
                );
            }
            opcode::FLOAD_3 => {
                fload_n(
                    heap,
                    thread,
                    class_loader,
                    &mut code_reader,
                    class.clone(),
                    3,
                );
            }
            opcode::IADD => {
                iadd(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::INVOKESTATIC => {
                invokestatic(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::IRETURN => {
                ireturn(heap, thread, class_loader, &mut code_reader, class);
                break;
            }
            opcode::RETURN => {
                return_(heap, thread, class_loader, &mut code_reader, class);
                break;
            }
            opcode::NOP => {}
            opcode::GETSTATIC => {
                getstatic(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::ACONST_NULL => {
                aconst_null(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::PUTSTATIC => {
                putstatic(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::INVOKEVIRTUAL => {
                invokevirtual(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::NEW => {
                new(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::NEWARRAY => {
                newarray(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::DUP => {
                dup(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::CASTORE => {
                castore(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::INVOKESPECIAL => {
                invokespecial(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::PUTFIELD => {
                putfield(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::IFGE => {
                ifge(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::IFLE => {
                ifle(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::FCMPG | opcode::FCMPL => {
                fcmpg(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            opcode::ANEWARRAY => {
                anewarray(heap, thread, class_loader, &mut code_reader, class.clone());
            }
            op => unimplemented!("{}", show_opcode(op)),
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
