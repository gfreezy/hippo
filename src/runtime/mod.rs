mod class;
mod class_loader;
mod code_reader;
mod cp_cache;
mod field;
mod frame;
mod heap;
mod instruction;
mod jvm_env;
mod method;
mod native;
mod opcode;

use crate::runtime::class::{Class, InstanceClass};
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::frame::JvmFrame;
use crate::runtime::instruction::*;
use crate::runtime::jvm_env::JvmEnv;
use crate::runtime::method::Method;
use crate::runtime::native::*;
use crate::runtime::opcode::show_opcode;
use std::panic;
use tracing::debug;

#[derive(Debug)]
pub struct Jvm {
    jenv: JvmEnv,
    main_class: String,
}

impl Drop for Jvm {
    fn drop(&mut self) {
        eprintln!("heap: {:?}", self.jenv.heap);
        eprintln!("backtraces:");
        for frame in &self.jenv.thread.stack.frames {
            eprintln!(
                "{}.{}:{}, pc={}",
                frame.method.class_name(),
                frame.method.name(),
                frame.method.descriptor(),
                frame.pc() - 1
            );
            eprintln!(
                "locals: {:?}\noperand_stack:{:?}",
                frame.local_variable_array, frame.operand_stack,
            );
        }
    }
}
impl Jvm {
    pub fn new(class_name: &str, jre_opt: Option<String>, cp_opt: Option<String>) -> Self {
        let mut jvm = Jvm {
            jenv: JvmEnv::new(jre_opt, cp_opt),
            main_class: class_name.to_string(),
        };
        let system_class = jvm.jenv.load_and_init_class("java/lang/System");
        let system_class_initialize = system_class
            .get_method("initializeSystemClass", "()V", true)
            .expect("system init");
        execute_method(&mut jvm.jenv, system_class_initialize, vec![]);
        jvm
    }

    pub fn run(&mut self) {
        let class = self
            .jenv
            .load_and_init_class(&self.main_class)
            .instance_class();
        let main_method = class.main_method().expect("find main method");
        execute_method(&mut self.jenv, main_method, vec![]);
    }
}

fn execute_method(jenv: &mut JvmEnv, method: Method, args: Vec<Operand>) {
    let is_native = method.is_native();
    let class = jenv.load_and_init_class(method.class_name());

    let span = tracing::debug_span!("execute_method", %class, %method, method_descriptor = %method.descriptor(), is_native);
    let _span = span.enter();

    if is_native {
        execute_native_method(jenv, &class, method, args);
        return;
    }

    let frame = JvmFrame::new_with_args(&method, args);
    jenv.thread.stack.frames.push_back(frame);

    while let Some(code) = jenv.thread.stack.frames.back_mut().unwrap().read_u8() {
        let frame = jenv.thread.stack.frames.back().unwrap();
        debug!(
            pc = frame.pc() - 1,
            opcode = opcode::show_opcode(code),
            ?frame,
            "will execute"
        );
        match code {
            opcode::ICONST_0 => {
                iconst_n(jenv, &class, 0);
            }
            opcode::ICONST_1 => {
                iconst_n(jenv, &class, 1);
            }
            opcode::ICONST_2 => {
                iconst_n(jenv, &class, 2);
            }
            opcode::ICONST_3 => {
                iconst_n(jenv, &class, 3);
            }
            opcode::ICONST_4 => {
                iconst_n(jenv, &class, 4);
            }
            opcode::ICONST_5 => {
                iconst_n(jenv, &class, 5);
            }
            opcode::FCONST_0 => {
                fconst_n(jenv, &class, 0.0);
            }
            opcode::FCONST_1 => {
                fconst_n(jenv, &class, 1.0);
            }
            opcode::FCONST_2 => {
                fconst_n(jenv, &class, 2.0);
            }
            opcode::LDC => ldc(jenv, &class),
            opcode::ISTORE_0 => {
                istore_n(jenv, &class, 0);
            }
            opcode::ISTORE_1 => {
                istore_n(jenv, &class, 1);
            }
            opcode::ISTORE_2 => {
                istore_n(jenv, &class, 2);
            }
            opcode::ISTORE_3 => {
                istore_n(jenv, &class, 3);
            }
            opcode::ISTORE => {
                istore(jenv, &class);
            }
            opcode::ASTORE_0 => {
                astore_n(jenv, &class, 0);
            }
            opcode::ASTORE_1 => {
                astore_n(jenv, &class, 1);
            }
            opcode::ASTORE_2 => {
                astore_n(jenv, &class, 2);
            }
            opcode::ASTORE_3 => {
                astore_n(jenv, &class, 3);
            }
            opcode::ASTORE => {
                astore(jenv, &class);
            }
            opcode::AASTORE => {
                aastore(jenv, &class);
            }
            opcode::BIPUSH => {
                let frame = jenv.thread.stack.frames.back_mut().unwrap();
                let byte = frame.read_u8().unwrap();
                frame.operand_stack.push_integer(byte as i32);
            }
            opcode::ILOAD_0 => {
                iload_n(jenv, &class, 0);
            }
            opcode::ILOAD_1 => {
                iload_n(jenv, &class, 1);
            }
            opcode::ILOAD_2 => {
                iload_n(jenv, &class, 2);
            }
            opcode::ILOAD_3 => {
                iload_n(jenv, &class, 3);
            }
            opcode::ALOAD_0 => {
                aload_n(jenv, &class, 0);
            }
            opcode::ALOAD_1 => {
                aload_n(jenv, &class, 1);
            }
            opcode::ALOAD_2 => {
                aload_n(jenv, &class, 2);
            }
            opcode::ALOAD_3 => {
                aload_n(jenv, &class, 3);
            }
            opcode::ALOAD => {
                aload(jenv, &class);
            }
            opcode::AALOAD => {
                aaload(jenv, &class);
            }
            opcode::FLOAD_0 => {
                fload_n(jenv, &class, 0);
            }
            opcode::FLOAD_1 => {
                fload_n(jenv, &class, 1);
            }
            opcode::FLOAD_2 => {
                fload_n(jenv, &class, 2);
            }
            opcode::FLOAD_3 => {
                fload_n(jenv, &class, 3);
            }
            opcode::CALOAD => {
                caload(jenv, &class);
            }
            opcode::IADD => {
                iadd(jenv, &class);
            }
            opcode::IREM => {
                irem(jenv, &class);
            }
            opcode::INVOKESTATIC => {
                invokestatic(jenv, &class);
            }
            opcode::IRETURN => {
                ireturn(jenv, &class);
                break;
            }
            opcode::DRETURN => {
                dreturn(jenv, &class);
                break;
            }
            opcode::FRETURN => {
                freturn(jenv, &class);
                break;
            }
            opcode::ARETURN => {
                areturn(jenv, &class);
                break;
            }
            opcode::RETURN => {
                return_(jenv, &class);
                break;
            }
            opcode::NOP => {}
            opcode::GETSTATIC => {
                getstatic(jenv, &class);
            }
            opcode::ACONST_NULL => {
                aconst_null(jenv, &class);
            }
            opcode::PUTSTATIC => {
                putstatic(jenv, &class);
            }
            opcode::INVOKEVIRTUAL => {
                invokevirtual(jenv, &class);
            }
            opcode::INVOKEINTERFACE => {
                invokeinterface(jenv, &class);
            }
            opcode::NEW => {
                new(jenv, &class);
            }
            opcode::NEWARRAY => {
                newarray(jenv, &class);
            }
            opcode::DUP => {
                dup(jenv, &class);
            }
            opcode::CASTORE => {
                castore(jenv, &class);
            }
            opcode::INVOKESPECIAL => {
                invokespecial(jenv, &class);
            }
            opcode::PUTFIELD => {
                putfield(jenv, &class);
            }
            opcode::GETFIELD => {
                getfield(jenv, &class);
            }
            opcode::IFGE => {
                ifge(jenv, &class);
            }
            opcode::IFGT => {
                ifgt(jenv, &class);
            }
            opcode::IFLT => {
                iflt(jenv, &class);
            }
            opcode::IFLE => {
                ifle(jenv, &class);
            }
            opcode::IFEQ => {
                ifeq(jenv, &class);
            }
            opcode::IFNE => {
                ifne(jenv, &class);
            }
            opcode::IFNONNULL => {
                ifnonnull(jenv, &class);
            }
            opcode::IFNULL => {
                ifnull(jenv, &class);
            }
            opcode::IF_ICMPEQ => {
                if_icmpeq(jenv, &class);
            }
            opcode::IF_ICMPGE => {
                if_icmpge(jenv, &class);
            }
            opcode::IF_ICMPGT => {
                if_icmpgt(jenv, &class);
            }
            opcode::IF_ICMPLE => {
                if_icmple(jenv, &class);
            }
            opcode::IF_ICMPLT => {
                if_icmplt(jenv, &class);
            }
            opcode::IF_ICMPNE => {
                if_icmpne(jenv, &class);
            }
            opcode::IF_ACMPNE => {
                if_acmpne(jenv, &class);
            }
            opcode::I2F => {
                i2f(jenv, &class);
            }
            opcode::F2I => {
                f2i(jenv, &class);
            }
            opcode::I2L => {
                i2l(jenv, &class);
            }
            opcode::FMUL => {
                fmul(jenv, &class);
            }
            opcode::FCMPG | opcode::FCMPL => {
                fcmpg(jenv, &class);
            }
            opcode::ANEWARRAY => {
                anewarray(jenv, &class);
            }
            opcode::GOTO => {
                goto(jenv, &class);
            }
            opcode::LDC2_W => {
                ldc2_w(jenv, &class);
            }
            opcode::SIPUSH => {
                sipush(jenv, &class);
            }
            opcode::LADD => {
                ladd(jenv, &class);
            }
            opcode::LSHL => {
                lshl(jenv, &class);
            }
            opcode::ISHL => {
                ishl(jenv, &class);
            }
            opcode::IUSHR => {
                iushr(jenv, &class);
            }
            opcode::IXOR => {
                ixor(jenv, &class);
            }
            opcode::LAND => {
                land(jenv, &class);
            }
            opcode::IAND => {
                iand(jenv, &class);
            }
            opcode::ISUB => {
                isub(jenv, &class);
            }
            opcode::ILOAD => {
                iload(jenv, &class);
            }
            opcode::IINC => {
                iinc(jenv, &class);
            }
            opcode::ARRAYLENGTH => {
                arraylength(jenv, &class);
            }
            opcode::POP => {
                pop(jenv, &class);
            }
            opcode::CHECKCAST => {
                checkcast(jenv, &class);
            }
            opcode::DUP_X1 => {
                dup_x1(jenv, &class);
            }
            opcode::INSTANCEOF => {
                instanceof(jenv, &class);
            }
            opcode::ATHROW => {
                athrow(jenv, &class);
            }
            opcode::MONITORENTER | opcode::MONITOREXIT => {}
            op => unimplemented!("{}", show_opcode(op)),
        }
    }
}

fn execute_native_method(jenv: &mut JvmEnv, class: &Class, method: Method, args: Vec<Operand>) {
    let frame = jenv.thread.stack.frames.back().unwrap();
    debug!(
        ?frame,
        ?args,
        %method,
        descriptor = method.descriptor(),
        "execute_native_method"
    );

    match (class.name(), method.name(), method.descriptor()) {
        ("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;") => {
            java_lang_Class_getPrimitiveClass(jenv, class, args);
        }
        (_, "desiredAssertionStatus0", "(Ljava/lang/Class;)Z") => {
            jvm_desiredAssertionStatus0(jenv, class, args);
        }
        ("java/lang/Float", "floatToRawIntBits", "(F)I") => {
            java_lang_Float_floatToRawIntBits(jenv, class, args);
        }
        ("java/lang/Double", "doubleToRawLongBits", "(D)J") => {
            java_lang_Double_doubleToRawLongBits(jenv, class, args);
        }
        ("java/lang/Double", "longBitsToDouble", "(J)D") => {
            java_lang_Double_longBitsToDouble(jenv, class, args);
        }
        (
            "java/lang/System",
            "initProperties",
            "(Ljava/util/Properties;)Ljava/util/Properties;",
        ) => {
            java_lang_System_initProperties(jenv, class, args);
        }
        ("java/lang/Object", "hashCode", "()I") => {
            java_lang_Object_hashCode(jenv, class, args);
        }
        ("java/lang/System", "registerNatives", "()V") => {
            java_lang_System_registerNatives(jenv, class, args);
        }
        ("java/lang/Object", "registerNatives", "()V") => {
            java_lang_Object_registerNatives(jenv, class, args);
        }
        ("java/lang/Class", "registerNatives", "()V") => {
            registerNatives(jenv, class, args);
        }
        ("java/lang/Thread", "registerNatives", "()V") => {
            registerNatives(jenv, class, args);
        }
        ("sun/misc/VM", "initialize", "()V") => {
            sun_misc_VM_initalize(jenv, class, args);
        }
        ("sun/misc/Unsafe", "registerNatives", "()V") => {
            sun_misc_Unsafe_registerNatives(jenv, class, args);
        }
        ("sun/misc/Unsafe", "arrayBaseOffset", "(Ljava/lang/Class;)I") => {
            sun_misc_Unsafe_arrayBaseOffset(jenv, class, args);
        }
        ("sun/misc/Unsafe", "arrayIndexScale", "(Ljava/lang/Class;)I") => {
            sun_misc_Unsafe_arrayIndexScale(jenv, class, args);
        }
        ("sun/misc/Unsafe", "addressSize", "()I") => {
            sun_misc_Unsafe_addressSize(jenv, class, args);
        }
        ("sun/reflect/Reflection", "getCallerClass", "()Ljava/lang/Class;") => {
            sun_reflect_Reflection_getCallerClass(jenv, class, args);
        }
        ("java/io/FileInputStream", "initIDs", "()V") => {
            java_io_FileInputStream_initIDs(jenv, class, args);
        }
        ("java/io/FileDescriptor", "initIDs", "()V") => {
            java_io_FileDescriptor_initIDs(jenv, class, args);
        }
        ("java/lang/Throwable", "fillInStackTrace", "(I)Ljava/lang/Throwable;") => {
            java_lang_Throwable_fillInStackTrace(jenv, class, args);
        }
        ("java/io/FileOutputStream", "initIDs", "()V") => {
            java_io_FileOutputStream_initIDs(jenv, class, args);
        }
        (
            "java/security/AccessController",
            "doPrivileged",
            "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
        ) => {
            java_security_AccessController_doPrivileged(jenv, class, args);
        }
        (
            "java/security/AccessController",
            "doPrivileged",
            "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
        ) => {
            java_security_AccessController_doPrivileged(jenv, class, args);
        }
        ("java/lang/Thread", "currentThread", "()Ljava/lang/Thread;") => {
            java_lang_Thread_currentThread(jenv, class, args);
        }
        ("java/lang/Class", "getName0", "()Ljava/lang/String;") => {
            java_lang_Class_getName0(jenv, class, args);
        }
        (
            "java/lang/Class",
            "forName0",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;",
        ) => {
            // todo:
        }
        (class_name, name, descriptor) => {
            panic!(
                r#"native method: ("{}", "{}", "{}")"#,
                class_name, name, descriptor
            );
        }
    };
}

pub fn execute_java_method(
    jenv: &mut JvmEnv,
    class: &InstanceClass,
    method_name: &str,
    descriptor: &str,
    is_static: bool,
    args: Vec<Operand>,
) {
    let method = class
        .get_method(method_name, descriptor, is_static)
        .unwrap();
    execute_method(jenv, method, args)
}
