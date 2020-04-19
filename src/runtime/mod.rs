mod class;
mod class_loader;
mod code_reader;
mod field;
mod frame;
mod heap;
mod instruction;
mod method;
mod native;
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
use crate::runtime::native::{
    java_lang_Class_getPrimitiveClass, java_lang_Double_doubleToRawLongBits,
    java_lang_Double_longBitsToDouble, java_lang_Float_floatToRawIntBits,
    java_lang_Object_hashCode, java_lang_System_initProperties, jvm_desiredAssertionStatus0,
};
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
    pub fn new(class_name: &str, jre_opt: Option<String>, cp_opt: Option<String>) -> Self {
        let mut jvm = Jvm {
            heap: JvmHeap::new(),
            thread: JvmThread {
                stack: JvmStack {
                    frames: Default::default(),
                },
                pc: 0,
            },
            class_loader: ClassLoader::new(ClassPath::new(jre_opt, cp_opt)),
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
        execute_native_method(heap, thread, class_loader, &class, method, args);

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
            ?frame,
            "will execute"
        );
        match code {
            opcode::ICONST_0 => {
                iconst_n(heap, thread, class_loader, &mut code_reader, &class, 0);
            }
            opcode::ICONST_1 => {
                iconst_n(heap, thread, class_loader, &mut code_reader, &class, 1);
            }
            opcode::ICONST_2 => {
                iconst_n(heap, thread, class_loader, &mut code_reader, &class, 2);
            }
            opcode::ICONST_3 => {
                iconst_n(heap, thread, class_loader, &mut code_reader, &class, 3);
            }
            opcode::ICONST_4 => {
                iconst_n(heap, thread, class_loader, &mut code_reader, &class, 4);
            }
            opcode::ICONST_5 => {
                iconst_n(heap, thread, class_loader, &mut code_reader, &class, 5);
            }
            opcode::FCONST_0 => {
                fconst_n(heap, thread, class_loader, &mut code_reader, &class, 0.0);
            }
            opcode::FCONST_1 => {
                fconst_n(heap, thread, class_loader, &mut code_reader, &class, 1.0);
            }
            opcode::FCONST_2 => {
                fconst_n(heap, thread, class_loader, &mut code_reader, &class, 2.0);
            }
            opcode::LDC => ldc(heap, thread, class_loader, &mut code_reader, &class),
            opcode::ISTORE_0 | opcode::ASTORE_0 => {
                store_n(heap, thread, class_loader, &mut code_reader, &class, 0);
            }
            opcode::ISTORE_1 | opcode::ASTORE_1 => {
                store_n(heap, thread, class_loader, &mut code_reader, &class, 1);
            }
            opcode::ISTORE_2 | opcode::ASTORE_2 => {
                store_n(heap, thread, class_loader, &mut code_reader, &class, 2);
            }
            opcode::ISTORE_3 | opcode::ASTORE_3 => {
                store_n(heap, thread, class_loader, &mut code_reader, &class, 3);
            }
            opcode::ISTORE | opcode::ASTORE => {
                store(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::BIPUSH => {
                let frame = thread.stack.frames.back_mut().unwrap();
                let byte = code_reader.read_u8().unwrap();
                frame.operand_stack.push_integer(byte as i32);
            }
            opcode::ILOAD_0 => {
                iload_n(heap, thread, class_loader, &mut code_reader, &class, 0);
            }
            opcode::ILOAD_1 => {
                iload_n(heap, thread, class_loader, &mut code_reader, &class, 1);
            }
            opcode::ILOAD_2 => {
                iload_n(heap, thread, class_loader, &mut code_reader, &class, 2);
            }
            opcode::ILOAD_3 => {
                iload_n(heap, thread, class_loader, &mut code_reader, &class, 3);
            }
            opcode::ALOAD_0 => {
                aload_n(heap, thread, class_loader, &mut code_reader, &class, 0);
            }
            opcode::ALOAD_1 => {
                aload_n(heap, thread, class_loader, &mut code_reader, &class, 1);
            }
            opcode::ALOAD_2 => {
                aload_n(heap, thread, class_loader, &mut code_reader, &class, 2);
            }
            opcode::ALOAD_3 => {
                aload_n(heap, thread, class_loader, &mut code_reader, &class, 3);
            }
            opcode::ALOAD => {
                aload(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::AALOAD => {
                aaload(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::FLOAD_0 => {
                fload_n(heap, thread, class_loader, &mut code_reader, &class, 0);
            }
            opcode::FLOAD_1 => {
                fload_n(heap, thread, class_loader, &mut code_reader, &class, 1);
            }
            opcode::FLOAD_2 => {
                fload_n(heap, thread, class_loader, &mut code_reader, &class, 2);
            }
            opcode::FLOAD_3 => {
                fload_n(heap, thread, class_loader, &mut code_reader, &class, 3);
            }
            opcode::IADD => {
                iadd(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IREM => {
                irem(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::INVOKESTATIC => {
                invokestatic(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IRETURN => {
                ireturn(heap, thread, class_loader, &mut code_reader, &class);
                break;
            }
            opcode::DRETURN => {
                dreturn(heap, thread, class_loader, &mut code_reader, &class);
                break;
            }
            opcode::FRETURN => {
                freturn(heap, thread, class_loader, &mut code_reader, &class);
                break;
            }
            opcode::ARETURN => {
                areturn(heap, thread, class_loader, &mut code_reader, &class);
                break;
            }
            opcode::RETURN => {
                return_(heap, thread, class_loader, &mut code_reader, &class);
                break;
            }
            opcode::NOP => {}
            opcode::GETSTATIC => {
                getstatic(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::ACONST_NULL => {
                aconst_null(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::PUTSTATIC => {
                putstatic(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::INVOKEVIRTUAL => {
                invokevirtual(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::NEW => {
                new(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::NEWARRAY => {
                newarray(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::DUP => {
                dup(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::CASTORE => {
                castore(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::INVOKESPECIAL => {
                invokespecial(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::PUTFIELD => {
                putfield(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::GETFIELD => {
                getfield(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IFGE => {
                ifge(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IFGT => {
                ifgt(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IFLE => {
                ifle(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IFEQ => {
                ifeq(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IFNE => {
                ifne(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IFNONNULL => {
                ifnonnull(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IFNULL => {
                ifnull(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::I2F => {
                i2f(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::F2I => {
                f2i(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::I2L => {
                i2l(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::FMUL => {
                fmul(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::FCMPG | opcode::FCMPL => {
                fcmpg(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::ANEWARRAY => {
                anewarray(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::GOTO => {
                goto(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::LDC2_W => {
                ldc2_w(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::SIPUSH => {
                sipush(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::LADD => {
                ladd(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::LSHL => {
                lshl(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::LAND => {
                land(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::IAND => {
                iand(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::ILOAD => {
                iload(heap, thread, class_loader, &mut code_reader, &class);
            }
            opcode::ARRAYLENGTH => {
                arraylength(heap, thread, class_loader, &mut code_reader, &class);
            }
            op => unimplemented!("{}", show_opcode(op)),
        }
    }
}

fn execute_native_method(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    class: &Class,
    method: Method,
    args: Vec<Operand>,
) {
    let frame = thread.stack.frames.back().unwrap();
    debug!(
        ?frame,
        ?args,
        %method,
        descriptor = method.descriptor(),
        "execute_native_method"
    );

    match (
        class.name(),
        method.name(),
        method.descriptor(),
        method.return_descriptor(),
    ) {
        ("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", _) => {
            java_lang_Class_getPrimitiveClass(heap, thread, class_loader, class, args);
        }
        (_, "desiredAssertionStatus0", "(Ljava/lang/Class;)Z", _) => {
            jvm_desiredAssertionStatus0(heap, thread, class_loader, class, args);
        }
        ("java/lang/Float", "floatToRawIntBits", "(F)I", _) => {
            java_lang_Float_floatToRawIntBits(heap, thread, class_loader, class, args);
        }
        ("java/lang/Double", "doubleToRawLongBits", "(D)J", _) => {
            java_lang_Double_doubleToRawLongBits(heap, thread, class_loader, class, args);
        }
        ("java/lang/Double", "longBitsToDouble", "(J)D", _) => {
            java_lang_Double_longBitsToDouble(heap, thread, class_loader, class, args);
        }
        (
            "java/lang/System",
            "initProperties",
            "(Ljava/util/Properties;)Ljava/util/Properties;",
            _,
        ) => {
            java_lang_System_initProperties(heap, thread, class_loader, class, args);
        }
        ("java/lang/Object", "hashCode", "()I", _) => {
            java_lang_Object_hashCode(heap, thread, class_loader, class, args);
        }
        (_, _, _, "V") => {
            debug!("skip native method");
        }
        (class_name, name, descriptor, _) => {
            panic!("native method: {}:{}, {}", class_name, name, descriptor);
        }
    };
}

pub fn execute_java_method(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    class: &Class,
    method_name: &str,
    descriptor: &str,
    is_static: bool,
    args: Vec<Operand>,
) {
    let method = class
        .get_method(method_name, descriptor, is_static)
        .unwrap();
    execute_method(heap, thread, class_loader, method, args)
}
