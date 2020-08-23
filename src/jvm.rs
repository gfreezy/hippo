use crate::class::{Class, Method};
use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;
use crate::class_loader::class_path::ClassPath;
use crate::class_loader::{init_class, load_class, BOOTSTRAP_LOADER};
use crate::frame::JvmFrame;
use crate::gc::global_definition::{JObject, JValue};

use crate::instruction::opcode::show_opcode;
use crate::instruction::*;
use crate::jenv::{JTHREAD, OPCODE_ID};
use crate::jni::execute_native_method;
use crate::jthread::JvmThread;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::debug;

pub static SPACE_SIZE: AtomicUsize = AtomicUsize::new(1024 * 1024 * 100);

#[derive(Debug)]
pub struct Jvm;

pub fn backtraces(thread: &JvmThread) {
    for frame in &thread.stack.frames {
        eprintln!(
            "{}.{}:{}, pc={}, line={:?}",
            frame.method.class_name(),
            frame.method.name(),
            frame.method.descriptor(),
            frame.pc() - 1,
            frame.method.line_for_pc(frame.pc() - 1),
        );
        eprintln!(
            "\tlocals: {:?}\n\toperand_stack:{:?}",
            frame.local_variable_array, frame.operand_stack,
        );
    }
}

impl Drop for Jvm {
    fn drop(&mut self) {
        eprintln!("backtraces:\n");
        JTHREAD.with(|thread| {
            backtraces(&*thread.borrow());
        });

        // dump_space();
    }
}

impl Default for Jvm {
    fn default() -> Self {
        Jvm::new(None, None)
    }
}

impl Jvm {
    pub fn new(jre_opt: Option<String>, cp_opt: Option<String>) -> Self {
        let classpath = ClassPath::new(jre_opt, cp_opt);
        let _ = BOOTSTRAP_LOADER.set(BootstrapClassLoader::new(classpath));

        Jvm
    }

    pub fn run(&mut self, main_class: &str) {
        JTHREAD.with(|thread| {
            let mut thread = thread.borrow_mut();
            let system_class = load_class(JObject::null(), "java/lang/System");
            init_class(&mut thread, &system_class);
            let system_class_initialize = system_class
                .get_method("initializeSystemClass", "()V", true)
                .expect("system init");

            execute_class_method(&mut thread, system_class, system_class_initialize, vec![]);
        });

        JTHREAD.with(|thread| {
            let mut thread = thread.borrow_mut();
            let class = load_class(JObject::null(), main_class);
            init_class(&mut thread, &class);
            let main_method = class
                .get_method("main", "([Ljava/lang/String;)V", true)
                .unwrap();

            execute_class_method(&mut thread, class, main_method, vec![]);
        });
    }
}

pub fn execute_method_by_name(
    thread: &mut JvmThread,
    class: &Class,
    name: &str,
    descriptor: &str,
    is_static: bool,
    args: Vec<JValue>,
) {
    let method = class
        .get_method(name, descriptor, is_static)
        .expect("no method found");
    execute_method(thread, method, args);
}

pub fn execute_method(thread: &mut JvmThread, method: Method, args: Vec<JValue>) {
    let class = load_class(method.class_loader(), method.class_name());
    init_class(thread, &class);
    execute_class_method(thread, class, method, args)
}

pub fn execute_class_method(
    thread: &mut JvmThread,
    class: Class,
    method: Method,
    args: Vec<JValue>,
) {
    assert_eq!(class.name(), method.class_name());
    if method.is_static() {
        assert_eq!(method.n_args(), args.len());
    } else {
        assert_eq!(method.n_args() + 1, args.len());
    }

    let is_native = method.is_native();

    let callstack = thread.callstack();
    // println!("----------------------");
    // println!("execute_method: {:?}", &callstack);
    // backtraces(thread);
    let span = tracing::debug_span!(
        "execute_method",
        callstack = %serde_json::to_string(&callstack).unwrap(),
        is_native
    );
    let _span = span.enter();

    if is_native {
        execute_native_method(thread, &class, method, args);
        return;
    }

    let frame = JvmFrame::new_with_args(class.clone(), method.clone(), args);
    thread.push_frame(frame);

    while let Some(code) = thread.current_frame_mut().read_u8() {
        let (parent_frame_id, parent_opcode_id) = thread
            .caller_frame()
            .map(|f| (f.id, f.opcode_id))
            .unwrap_or((0, 0));
        let frame = thread.current_frame_mut();
        let opcode_id = OPCODE_ID.fetch_add(1, Ordering::SeqCst);
        frame.set_opcode_id(opcode_id);
        debug!(
            frame_id = frame.id,
            pc = frame.pc() - 1,
            opcode_id,
            opcode = opcode::show_opcode(code),
            frame = %serde_json::to_string(&frame).unwrap(),
            parent_frame_id,
            parent_opcode_id,
            "will execute"
        );
        match code {
            opcode::ICONST_0 => {
                iconst_n(thread, &class, 0);
            }
            opcode::ICONST_1 => {
                iconst_n(thread, &class, 1);
            }
            opcode::ICONST_2 => {
                iconst_n(thread, &class, 2);
            }
            opcode::ICONST_3 => {
                iconst_n(thread, &class, 3);
            }
            opcode::ICONST_4 => {
                iconst_n(thread, &class, 4);
            }
            opcode::ICONST_5 => {
                iconst_n(thread, &class, 5);
            }
            opcode::ICONST_M1 => {
                iconst_n(thread, &class, -1);
            }
            opcode::LCONST_0 => {
                lconst_n(thread, &class, 0);
            }
            opcode::LCONST_1 => {
                lconst_n(thread, &class, 1);
            }
            opcode::FCONST_0 => {
                fconst_n(thread, &class, 0.0);
            }
            opcode::FCONST_1 => {
                fconst_n(thread, &class, 1.0);
            }
            opcode::FCONST_2 => {
                fconst_n(thread, &class, 2.0);
            }
            opcode::LDC => ldc(thread, &class),
            opcode::ISTORE_0 => {
                istore_n(thread, &class, 0);
            }
            opcode::ISTORE_1 => {
                istore_n(thread, &class, 1);
            }
            opcode::ISTORE_2 => {
                istore_n(thread, &class, 2);
            }
            opcode::ISTORE_3 => {
                istore_n(thread, &class, 3);
            }
            opcode::ISTORE => {
                istore(thread, &class);
            }
            opcode::ASTORE_0 => {
                astore_n(thread, &class, 0);
            }
            opcode::ASTORE_1 => {
                astore_n(thread, &class, 1);
            }
            opcode::ASTORE_2 => {
                astore_n(thread, &class, 2);
            }
            opcode::ASTORE_3 => {
                astore_n(thread, &class, 3);
            }
            opcode::ASTORE => {
                astore(thread, &class);
            }
            opcode::AASTORE => {
                aastore(thread, &class);
            }
            opcode::IASTORE => {
                iastore(thread, &class);
            }
            opcode::IALOAD => {
                iaload(thread, &class);
            }
            opcode::BIPUSH => {
                bipush(thread, &class);
            }
            opcode::ILOAD_0 => {
                iload_n(thread, &class, 0);
            }
            opcode::ILOAD_1 => {
                iload_n(thread, &class, 1);
            }
            opcode::ILOAD_2 => {
                iload_n(thread, &class, 2);
            }
            opcode::ILOAD_3 => {
                iload_n(thread, &class, 3);
            }
            opcode::LLOAD_0 => {
                lload_n(thread, &class, 0);
            }
            opcode::LLOAD_1 => {
                lload_n(thread, &class, 1);
            }
            opcode::LLOAD_2 => {
                lload_n(thread, &class, 2);
            }
            opcode::LLOAD_3 => {
                lload_n(thread, &class, 3);
            }
            opcode::LLOAD => {
                lload(thread, &class);
            }
            opcode::ALOAD_0 => {
                aload_n(thread, &class, 0);
            }
            opcode::ALOAD_1 => {
                aload_n(thread, &class, 1);
            }
            opcode::ALOAD_2 => {
                aload_n(thread, &class, 2);
            }
            opcode::ALOAD_3 => {
                aload_n(thread, &class, 3);
            }
            opcode::ALOAD => {
                aload(thread, &class);
            }
            opcode::AALOAD => {
                aaload(thread, &class);
            }
            opcode::FLOAD_0 => {
                fload_n(thread, &class, 0);
            }
            opcode::FLOAD_1 => {
                fload_n(thread, &class, 1);
            }
            opcode::FLOAD_2 => {
                fload_n(thread, &class, 2);
            }
            opcode::FLOAD_3 => {
                fload_n(thread, &class, 3);
            }
            opcode::CALOAD => {
                caload(thread, &class);
            }
            opcode::IADD => {
                iadd(thread, &class);
            }
            opcode::INEG => {
                ineg(thread, &class);
            }
            opcode::IMUL => {
                imul(thread, &class);
            }
            opcode::IREM => {
                irem(thread, &class);
            }
            opcode::INVOKESTATIC => {
                invokestatic(thread, &class);
            }
            opcode::IRETURN => {
                ireturn(thread, &class);
                break;
            }
            opcode::DRETURN => {
                dreturn(thread, &class);
                break;
            }
            opcode::FRETURN => {
                freturn(thread, &class);
                break;
            }
            opcode::ARETURN => {
                areturn(thread, &class);
                break;
            }
            opcode::LRETURN => {
                lreturn(thread, &class);
                break;
            }
            opcode::RETURN => {
                return_(thread, &class);
                break;
            }
            opcode::NOP => {}
            opcode::GETSTATIC => {
                getstatic(thread, &class);
            }
            opcode::ACONST_NULL => {
                aconst_null(thread, &class);
            }
            opcode::PUTSTATIC => {
                putstatic(thread, &class);
            }
            opcode::INVOKEVIRTUAL => {
                invokevirtual(thread, &class);
            }
            opcode::INVOKEINTERFACE => {
                invokeinterface(thread, &class);
            }
            opcode::NEW => {
                new(thread, &class);
            }
            opcode::NEWARRAY => {
                newarray(thread, &class);
            }
            opcode::DUP => {
                dup(thread, &class);
            }
            opcode::DUP2 => {
                dup2(thread, &class);
            }
            opcode::CASTORE => {
                castore(thread, &class);
            }
            opcode::INVOKESPECIAL => {
                invokespecial(thread, &class);
            }
            opcode::PUTFIELD => {
                putfield(thread, &class);
            }
            opcode::GETFIELD => {
                getfield(thread, &class);
            }
            opcode::IFGE => {
                ifge(thread, &class);
            }
            opcode::IFGT => {
                ifgt(thread, &class);
            }
            opcode::IFLT => {
                iflt(thread, &class);
            }
            opcode::IFLE => {
                ifle(thread, &class);
            }
            opcode::IFEQ => {
                ifeq(thread, &class);
            }
            opcode::IFNE => {
                ifne(thread, &class);
            }
            opcode::IFNONNULL => {
                ifnonnull(thread, &class);
            }
            opcode::IFNULL => {
                ifnull(thread, &class);
            }
            opcode::IF_ICMPEQ => {
                if_icmpeq(thread, &class);
            }
            opcode::IF_ICMPGE => {
                if_icmpge(thread, &class);
            }
            opcode::IF_ICMPGT => {
                if_icmpgt(thread, &class);
            }
            opcode::IF_ICMPLE => {
                if_icmple(thread, &class);
            }
            opcode::IF_ICMPLT => {
                if_icmplt(thread, &class);
            }
            opcode::IF_ICMPNE => {
                if_icmpne(thread, &class);
            }
            opcode::IF_ACMPNE => {
                if_acmpne(thread, &class);
            }
            opcode::IF_ACMPEQ => {
                if_acmpeq(thread, &class);
            }
            opcode::I2F => {
                i2f(thread, &class);
            }
            opcode::I2C => {
                i2c(thread, &class);
            }
            opcode::F2I => {
                f2i(thread, &class);
            }
            opcode::I2L => {
                i2l(thread, &class);
            }
            opcode::FMUL => {
                fmul(thread, &class);
            }
            opcode::FCMPG | opcode::FCMPL => {
                fcmpg(thread, &class);
            }
            opcode::ANEWARRAY => {
                anewarray(thread, &class);
            }
            opcode::GOTO => {
                goto(thread, &class);
            }
            opcode::LDC2_W => {
                ldc2_w(thread, &class);
            }
            opcode::LDC_W => {
                ldc_w(thread, &class);
            }
            opcode::SIPUSH => {
                sipush(thread, &class);
            }
            opcode::LADD => {
                ladd(thread, &class);
            }
            opcode::LSHL => {
                lshl(thread, &class);
            }
            opcode::ISHL => {
                ishl(thread, &class);
            }
            opcode::ISHR => {
                ishr(thread, &class);
            }
            opcode::IUSHR => {
                iushr(thread, &class);
            }
            opcode::IXOR => {
                ixor(thread, &class);
            }
            opcode::LAND => {
                land(thread, &class);
            }
            opcode::IAND => {
                iand(thread, &class);
            }
            opcode::IOR => {
                ior(thread, &class);
            }
            opcode::ISUB => {
                isub(thread, &class);
            }
            opcode::ILOAD => {
                iload(thread, &class);
            }
            opcode::IINC => {
                iinc(thread, &class);
            }
            opcode::ARRAYLENGTH => {
                arraylength(thread, &class);
            }
            opcode::POP => {
                pop(thread, &class);
            }
            opcode::CHECKCAST => {
                checkcast(thread, &class);
            }
            opcode::DUP_X1 => {
                dup_x1(thread, &class);
            }
            opcode::INSTANCEOF => {
                instanceof(thread, &class);
            }
            opcode::ATHROW => {
                athrow(thread, &class);
            }
            opcode::TABLESWITCH => {
                tableswitch(thread, &class);
            }
            opcode::LOOKUPSWITCH => {
                lookupswitch(thread, &class);
            }
            opcode::MONITORENTER => {
                monitorenter(thread, &class);
            }
            opcode::MONITOREXIT => {
                monitorexit(thread, &class);
            }
            op => unimplemented!("{}", show_opcode(op)),
        }
    }
}
