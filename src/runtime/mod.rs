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

use crate::runtime::class::InstanceClass;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::frame::JvmFrame;
use crate::runtime::instruction::*;
use crate::runtime::jvm_env::JvmEnv;
use crate::runtime::method::Method;
use crate::runtime::native::*;
use crate::runtime::opcode::show_opcode;
use tracing::debug;

#[derive(Debug)]
pub struct Jvm {
    jenv: JvmEnv,
    main_class: String,
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
        let class = self.jenv.load_and_init_class(&self.main_class);
        let main_method = class.main_method().expect("find main method");
        execute_method(&mut self.jenv, main_method, vec![]);
    }
}

fn execute_method(jenv: &mut JvmEnv, method: Method, args: Vec<Operand>) {
    let is_native = method.is_native();
    let class = jenv.load_and_init_class(method.class_name());

    let span = tracing::debug_span!("execute_method", %class, %method, is_native);
    let _span = span.enter();

    if is_native {
        execute_native_method(jenv, &class, method, args);

        return;
    }

    let frame = JvmFrame::new_with_args(&method, args);
    jenv.thread.stack.frames.push_back(frame);

    let mut code_reader = CodeReader::new(method);
    while let Some(code) = code_reader.read_u8() {
        let frame = jenv.thread.stack.frames.back().unwrap();
        debug!(
            pc = code_reader.pc(),
            opcode = opcode::show_opcode(code),
            ?frame,
            ?jenv.heap,
            "will execute"
        );
        match code {
            opcode::ICONST_0 => {
                iconst_n(jenv, &mut code_reader, &class, 0);
            }
            opcode::ICONST_1 => {
                iconst_n(jenv, &mut code_reader, &class, 1);
            }
            opcode::ICONST_2 => {
                iconst_n(jenv, &mut code_reader, &class, 2);
            }
            opcode::ICONST_3 => {
                iconst_n(jenv, &mut code_reader, &class, 3);
            }
            opcode::ICONST_4 => {
                iconst_n(jenv, &mut code_reader, &class, 4);
            }
            opcode::ICONST_5 => {
                iconst_n(jenv, &mut code_reader, &class, 5);
            }
            opcode::FCONST_0 => {
                fconst_n(jenv, &mut code_reader, &class, 0.0);
            }
            opcode::FCONST_1 => {
                fconst_n(jenv, &mut code_reader, &class, 1.0);
            }
            opcode::FCONST_2 => {
                fconst_n(jenv, &mut code_reader, &class, 2.0);
            }
            opcode::LDC => ldc(jenv, &mut code_reader, &class),
            opcode::ISTORE_0 | opcode::ASTORE_0 => {
                store_n(jenv, &mut code_reader, &class, 0);
            }
            opcode::ISTORE_1 | opcode::ASTORE_1 => {
                store_n(jenv, &mut code_reader, &class, 1);
            }
            opcode::ISTORE_2 | opcode::ASTORE_2 => {
                store_n(jenv, &mut code_reader, &class, 2);
            }
            opcode::ISTORE_3 | opcode::ASTORE_3 => {
                store_n(jenv, &mut code_reader, &class, 3);
            }
            opcode::ISTORE | opcode::ASTORE => {
                store(jenv, &mut code_reader, &class);
            }
            opcode::AASTORE => {
                aastore(jenv, &mut code_reader, &class);
            }
            opcode::BIPUSH => {
                let frame = jenv.thread.stack.frames.back_mut().unwrap();
                let byte = code_reader.read_u8().unwrap();
                frame.operand_stack.push_integer(byte as i32);
            }
            opcode::ILOAD_0 => {
                iload_n(jenv, &mut code_reader, &class, 0);
            }
            opcode::ILOAD_1 => {
                iload_n(jenv, &mut code_reader, &class, 1);
            }
            opcode::ILOAD_2 => {
                iload_n(jenv, &mut code_reader, &class, 2);
            }
            opcode::ILOAD_3 => {
                iload_n(jenv, &mut code_reader, &class, 3);
            }
            opcode::ALOAD_0 => {
                aload_n(jenv, &mut code_reader, &class, 0);
            }
            opcode::ALOAD_1 => {
                aload_n(jenv, &mut code_reader, &class, 1);
            }
            opcode::ALOAD_2 => {
                aload_n(jenv, &mut code_reader, &class, 2);
            }
            opcode::ALOAD_3 => {
                aload_n(jenv, &mut code_reader, &class, 3);
            }
            opcode::ALOAD => {
                aload(jenv, &mut code_reader, &class);
            }
            opcode::AALOAD => {
                aaload(jenv, &mut code_reader, &class);
            }
            opcode::FLOAD_0 => {
                fload_n(jenv, &mut code_reader, &class, 0);
            }
            opcode::FLOAD_1 => {
                fload_n(jenv, &mut code_reader, &class, 1);
            }
            opcode::FLOAD_2 => {
                fload_n(jenv, &mut code_reader, &class, 2);
            }
            opcode::FLOAD_3 => {
                fload_n(jenv, &mut code_reader, &class, 3);
            }
            opcode::IADD => {
                iadd(jenv, &mut code_reader, &class);
            }
            opcode::IREM => {
                irem(jenv, &mut code_reader, &class);
            }
            opcode::INVOKESTATIC => {
                invokestatic(jenv, &mut code_reader, &class);
            }
            opcode::IRETURN => {
                ireturn(jenv, &mut code_reader, &class);
                break;
            }
            opcode::DRETURN => {
                dreturn(jenv, &mut code_reader, &class);
                break;
            }
            opcode::FRETURN => {
                freturn(jenv, &mut code_reader, &class);
                break;
            }
            opcode::ARETURN => {
                areturn(jenv, &mut code_reader, &class);
                break;
            }
            opcode::RETURN => {
                return_(jenv, &mut code_reader, &class);
                break;
            }
            opcode::NOP => {}
            opcode::GETSTATIC => {
                getstatic(jenv, &mut code_reader, &class);
            }
            opcode::ACONST_NULL => {
                aconst_null(jenv, &mut code_reader, &class);
            }
            opcode::PUTSTATIC => {
                putstatic(jenv, &mut code_reader, &class);
            }
            opcode::INVOKEVIRTUAL => {
                invokevirtual(jenv, &mut code_reader, &class);
            }
            opcode::INVOKEINTERFACE => {
                invokeinterface(jenv, &mut code_reader, &class);
            }
            opcode::NEW => {
                new(jenv, &mut code_reader, &class);
            }
            opcode::NEWARRAY => {
                newarray(jenv, &mut code_reader, &class);
            }
            opcode::DUP => {
                dup(jenv, &mut code_reader, &class);
            }
            opcode::CASTORE => {
                castore(jenv, &mut code_reader, &class);
            }
            opcode::INVOKESPECIAL => {
                invokespecial(jenv, &mut code_reader, &class);
            }
            opcode::PUTFIELD => {
                putfield(jenv, &mut code_reader, &class);
            }
            opcode::GETFIELD => {
                getfield(jenv, &mut code_reader, &class);
            }
            opcode::IFGE => {
                ifge(jenv, &mut code_reader, &class);
            }
            opcode::IFGT => {
                ifgt(jenv, &mut code_reader, &class);
            }
            opcode::IFLE => {
                ifle(jenv, &mut code_reader, &class);
            }
            opcode::IFEQ => {
                ifeq(jenv, &mut code_reader, &class);
            }
            opcode::IFNE => {
                ifne(jenv, &mut code_reader, &class);
            }
            opcode::IFNONNULL => {
                ifnonnull(jenv, &mut code_reader, &class);
            }
            opcode::IFNULL => {
                ifnull(jenv, &mut code_reader, &class);
            }
            opcode::IF_ICMPEQ => {
                if_icmpeq(jenv, &mut code_reader, &class);
            }
            opcode::IF_ICMPGE => {
                if_icmpge(jenv, &mut code_reader, &class);
            }
            opcode::IF_ICMPGT => {
                if_icmpgt(jenv, &mut code_reader, &class);
            }
            opcode::IF_ICMPLE => {
                if_icmple(jenv, &mut code_reader, &class);
            }
            opcode::IF_ICMPLT => {
                if_icmplt(jenv, &mut code_reader, &class);
            }
            opcode::IF_ICMPNE => {
                if_icmpne(jenv, &mut code_reader, &class);
            }
            opcode::I2F => {
                i2f(jenv, &mut code_reader, &class);
            }
            opcode::F2I => {
                f2i(jenv, &mut code_reader, &class);
            }
            opcode::I2L => {
                i2l(jenv, &mut code_reader, &class);
            }
            opcode::FMUL => {
                fmul(jenv, &mut code_reader, &class);
            }
            opcode::FCMPG | opcode::FCMPL => {
                fcmpg(jenv, &mut code_reader, &class);
            }
            opcode::ANEWARRAY => {
                anewarray(jenv, &mut code_reader, &class);
            }
            opcode::GOTO => {
                goto(jenv, &mut code_reader, &class);
            }
            opcode::LDC2_W => {
                ldc2_w(jenv, &mut code_reader, &class);
            }
            opcode::SIPUSH => {
                sipush(jenv, &mut code_reader, &class);
            }
            opcode::LADD => {
                ladd(jenv, &mut code_reader, &class);
            }
            opcode::LSHL => {
                lshl(jenv, &mut code_reader, &class);
            }
            opcode::ISHL => {
                ishl(jenv, &mut code_reader, &class);
            }
            opcode::LAND => {
                land(jenv, &mut code_reader, &class);
            }
            opcode::IAND => {
                iand(jenv, &mut code_reader, &class);
            }
            opcode::ISUB => {
                isub(jenv, &mut code_reader, &class);
            }
            opcode::ILOAD => {
                iload(jenv, &mut code_reader, &class);
            }
            opcode::IINC => {
                iinc(jenv, &mut code_reader, &class);
            }
            opcode::ARRAYLENGTH => {
                arraylength(jenv, &mut code_reader, &class);
            }
            opcode::POP => {
                pop(jenv, &mut code_reader, &class);
            }
            opcode::MONITORENTER | opcode::MONITOREXIT => {}
            op => unimplemented!("{}", show_opcode(op)),
        }
    }
}

fn execute_native_method(
    jenv: &mut JvmEnv,
    class: &InstanceClass,
    method: Method,
    args: Vec<Operand>,
) {
    let frame = jenv.thread.stack.frames.back().unwrap();
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
            java_lang_Class_getPrimitiveClass(jenv, class, args);
        }
        (_, "desiredAssertionStatus0", "(Ljava/lang/Class;)Z", _) => {
            jvm_desiredAssertionStatus0(jenv, class, args);
        }
        ("java/lang/Float", "floatToRawIntBits", "(F)I", _) => {
            java_lang_Float_floatToRawIntBits(jenv, class, args);
        }
        ("java/lang/Double", "doubleToRawLongBits", "(D)J", _) => {
            java_lang_Double_doubleToRawLongBits(jenv, class, args);
        }
        ("java/lang/Double", "longBitsToDouble", "(J)D", _) => {
            java_lang_Double_longBitsToDouble(jenv, class, args);
        }
        (
            "java/lang/System",
            "initProperties",
            "(Ljava/util/Properties;)Ljava/util/Properties;",
            _,
        ) => {
            java_lang_System_initProperties(jenv, class, args);
        }
        ("java/lang/Object", "hashCode", "()I", _) => {
            java_lang_Object_hashCode(jenv, class, args);
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
