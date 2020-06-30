use crate::class::{Class, Method};
use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;
use crate::class_loader::class_path::ClassPath;
use crate::class_loader::{init_class, load_class, BOOTSTRAP_LOADER};
use crate::frame::JvmFrame;
use crate::gc::global_definition::{JObject, JValue};

use crate::gc::allocator_local::AllocatorLocal;
use crate::gc::space::Space;
use crate::gc::tlab::initialize_tlab;
use crate::instruction::opcode::show_opcode;
use crate::instruction::*;
use crate::jenv::JTHREAD;
use crate::jthread::JvmThread;
use crate::native::*;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug)]
pub struct Jvm {
    main_class: String,
}

impl Drop for Jvm {
    fn drop(&mut self) {
        eprintln!("backtraces:\n");
        JTHREAD.with(|thread| {
            for frame in &thread.borrow().stack.frames {
                eprintln!(
                    "{}.{}:{}, pc={}",
                    frame.method.class_name(),
                    frame.method.name(),
                    frame.method.descriptor(),
                    frame.pc() - 1
                );
                eprintln!(
                    "\tlocals: {:?}\n\toperand_stack:{:?}",
                    frame.local_variable_array, frame.operand_stack,
                );
            }
        });
    }
}

impl Jvm {
    pub fn new(class_name: &str, jre_opt: Option<String>, cp_opt: Option<String>) -> Self {
        let jvm = Jvm {
            main_class: class_name.to_string(),
        };
        BOOTSTRAP_LOADER
            .set(BootstrapClassLoader::new(ClassPath::new(jre_opt, cp_opt)))
            .unwrap();
        initialize_tlab(AllocatorLocal::new(Arc::new(Space::new(1024 * 1024 * 100))));

        JTHREAD.with(|thread| {
            let mut thread = thread.borrow_mut();
            let system_class = load_class(JObject::null(), "java/lang/System");
            init_class(&mut thread, &system_class);
            let system_class_initialize = system_class
                .get_method("initializeSystemClass", "()V", true)
                .expect("system init");

            execute_class_method(&mut thread, system_class, system_class_initialize, vec![]);
        });
        jvm
    }

    pub fn run(&mut self) {
        JTHREAD.with(|thread| {
            let mut thread = thread.borrow_mut();
            let class = load_class(JObject::null(), &self.main_class);
            init_class(&mut thread, &class);
            let main_method = class
                .get_method("main", "([Ljava/lang/String;)V", true)
                .unwrap();

            execute_class_method(&mut thread, class, main_method, vec![]);
        });
    }
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

    let span = tracing::debug_span!("execute_method", %class, %method, method_descriptor = %method.descriptor(), is_native);
    let _span = span.enter();

    if is_native {
        execute_native_method(thread, &class, method, args);
        return;
    }

    let frame = JvmFrame::new_with_args(class.clone(), method.clone(), args);
    thread.push_frame(frame);

    while let Some(code) = thread.current_frame_mut().read_u8() {
        let frame = thread.current_frame_mut();
        debug!(
            pc = frame.pc() - 1,
            opcode = opcode::show_opcode(code),
            ?frame,
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
            opcode::MONITORENTER | opcode::MONITOREXIT => {}
            op => unimplemented!("{}", show_opcode(op)),
        }
    }
}

fn execute_native_method(thread: &mut JvmThread, class: &Class, method: Method, args: Vec<JValue>) {
    let frame = thread.current_frame_mut();
    debug!(
        ?frame,
        ?args,
        %method,
        descriptor = method.descriptor(),
        "execute_native_method"
    );

    match (class.name(), method.name(), method.descriptor()) {
        ("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;") => {
            java_lang_Class_getPrimitiveClass(thread, class, args);
        }
        (_, "desiredAssertionStatus0", "(Ljava/lang/Class;)Z") => {
            jvm_desiredAssertionStatus0(thread, class, args);
        }
        ("java/lang/Float", "floatToRawIntBits", "(F)I") => {
            java_lang_Float_floatToRawIntBits(thread, class, args);
        }
        ("java/lang/Double", "doubleToRawLongBits", "(D)J") => {
            java_lang_Double_doubleToRawLongBits(thread, class, args);
        }
        ("java/lang/Double", "longBitsToDouble", "(J)D") => {
            java_lang_Double_longBitsToDouble(thread, class, args);
        }
        (
            "java/lang/System",
            "initProperties",
            "(Ljava/util/Properties;)Ljava/util/Properties;",
        ) => {
            java_lang_System_initProperties(thread, class, args);
        }
        ("java/lang/Object", "hashCode", "()I") => {
            java_lang_Object_hashCode(thread, class, args);
        }
        ("java/lang/System", "registerNatives", "()V") => {
            java_lang_System_registerNatives(thread, class, args);
        }
        ("java/lang/Object", "registerNatives", "()V") => {
            java_lang_Object_registerNatives(thread, class, args);
        }
        ("java/lang/Class", "registerNatives", "()V") => {
            registerNatives(thread, class, args);
        }
        ("java/lang/Thread", "registerNatives", "()V") => {
            registerNatives(thread, class, args);
        }
        ("sun/misc/VM", "initialize", "()V") => {
            sun_misc_VM_initalize(thread, class, args);
        }
        ("sun/misc/Unsafe", "registerNatives", "()V") => {
            sun_misc_Unsafe_registerNatives(thread, class, args);
        }
        ("sun/misc/Unsafe", "arrayBaseOffset", "(Ljava/lang/Class;)I") => {
            sun_misc_Unsafe_arrayBaseOffset(thread, class, args);
        }
        ("sun/misc/Unsafe", "arrayIndexScale", "(Ljava/lang/Class;)I") => {
            sun_misc_Unsafe_arrayIndexScale(thread, class, args);
        }
        ("sun/misc/Unsafe", "addressSize", "()I") => {
            sun_misc_Unsafe_addressSize(thread, class, args);
        }
        ("sun/reflect/Reflection", "getCallerClass", "()Ljava/lang/Class;") => {
            sun_reflect_Reflection_getCallerClass(thread, class, args);
        }
        ("java/io/FileInputStream", "initIDs", "()V") => {
            java_io_FileInputStream_initIDs(thread, class, args);
        }
        ("java/io/FileDescriptor", "initIDs", "()V") => {
            java_io_FileDescriptor_initIDs(thread, class, args);
        }
        ("java/lang/Throwable", "fillInStackTrace", "(I)Ljava/lang/Throwable;") => {
            java_lang_Throwable_fillInStackTrace(thread, class, args);
        }
        ("java/io/FileOutputStream", "initIDs", "()V") => {
            java_io_FileOutputStream_initIDs(thread, class, args);
        }
        (
            "java/security/AccessController",
            "doPrivileged",
            "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
        ) => {
            java_security_AccessController_doPrivileged(thread, class, args);
        }
        (
            "java/security/AccessController",
            "doPrivileged",
            "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
        ) => {
            java_security_AccessController_doPrivileged(thread, class, args);
        }
        ("java/lang/Thread", "currentThread", "()Ljava/lang/Thread;") => {
            java_lang_Thread_currentThread(thread, class, args);
        }
        ("java/lang/Class", "getName0", "()Ljava/lang/String;") => {
            java_lang_Class_getName0(thread, class, args);
        }
        (
            "java/lang/Class",
            "forName0",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;",
        ) => {
            java_lang_Class_for_Name0(thread, class, args);
        }
        (
            "java/security/AccessController",
            "getStackAccessControlContext",
            "()Ljava/security/AccessControlContext;",
        ) => {
            java_security_AccessController_getStackAccessControlContext(thread, class, args);
        }
        ("java/lang/Thread", "setPriority0", "(I)V") => {
            java_lang_Thread_setPriority0(thread, class, args)
        }
        ("java/lang/Thread", "isAlive", "()Z") => java_lang_Thread_isAlive(thread, class, args),
        (class_name, name, descriptor) => {
            panic!(
                r#"native method: ("{}", "{}", "{}")"#,
                class_name, name, descriptor
            );
        }
    };
}
