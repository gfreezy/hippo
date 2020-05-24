#![allow(non_snake_case, unused_variables)]
use crate::runtime::class::Class;
use crate::runtime::execute_method;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::jvm_env::JvmEnv;

pub fn java_lang_Class_getPrimitiveClass(
    jenv: &mut JvmEnv,
    _class: &Class,
    mut args: Vec<Operand>,
) {
    let string_ref = args.pop().unwrap();
    let class_name = jenv.get_java_string(&string_ref);
    let addr = jenv.new_java_lang_class(&class_name);
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::ObjectRef(addr));
}

pub fn jvm_desiredAssertionStatus0(jenv: &mut JvmEnv, _class: &Class, _args: Vec<Operand>) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(0);
}

pub fn java_lang_Float_floatToRawIntBits(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    let n = args[0].get_float();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(n.to_bits() as i32);
}

pub fn java_lang_Double_doubleToRawLongBits(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    let n = args[0].get_double();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_long(n.to_bits() as i64);
}

pub fn java_lang_Double_longBitsToDouble(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    let n = args[0].get_long();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame
        .operand_stack
        .push_double(f64::from_be_bytes(n.to_be_bytes()));
}

pub fn java_lang_System_initProperties(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    let props_ref = &args[0];
    let properties = jenv.heap.get_object(props_ref);
    let class_name = properties.class_name().to_string();
    let propertiesClass = jenv.load_and_init_class(&class_name);
    let method = propertiesClass
        .get_method(
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            false,
        )
        .unwrap();
    let systemProperties = vec![
        ("java.version", "1.8"),
        ("java.vendor", "hippo"),
        ("java.vendor.url", "https://github.com/gfreezy/hippo"),
        ("java.home", "/Users/feichao"),
        ("java.class.version", "1.8"),
        ("java.class.path", "/Users/feichao"),
        ("os.name", "macos"),
        ("os.arch", "x64"),
        ("os.version", "10.115.4"),
        ("file.separator", "/"),
        ("path.separator", ":"),
        ("line.separator", "\n"),
        ("user.name", "feichao"),
        ("user.home", "/Users/feichao"),
        ("user.dir", "/Users/feichao"),
    ];
    for (key, value) in systemProperties {
        let key = Operand::ObjectRef(jenv.new_java_lang_string(key));
        let value = Operand::ObjectRef(jenv.new_java_lang_string(value));
        let args = vec![props_ref.clone(), key, value];
        execute_method(jenv, method.clone(), args);
    }
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(props_ref.clone());
}

pub fn java_lang_Object_hashCode(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    let obj = &args[0];
    jenv.thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push_integer(obj.hash_code());
}

pub fn java_lang_System_registerNatives(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {}

pub fn java_lang_Object_registerNatives(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {}

pub fn registerNatives(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {}

pub fn sun_misc_VM_initalize(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {}

pub fn sun_misc_Unsafe_registerNatives(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {}

pub fn sun_misc_Unsafe_arrayBaseOffset(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    jenv.thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push_integer(0);
}

pub fn sun_misc_Unsafe_arrayIndexScale(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    jenv.thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push_integer(1);
}

pub fn sun_misc_Unsafe_addressSize(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    jenv.thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push_integer(8);
}

pub fn sun_reflect_Reflection_getCallerClass(
    jenv: &mut JvmEnv,
    _class: &Class,
    args: Vec<Operand>,
) {
    let frames = &jenv.thread.stack.frames;
    let len = frames.len();
    let caller_class = if len >= 2 {
        let frame = &frames[len - 2];
        let class_name = frame.method.class_name().to_string();
        let addr = jenv.new_java_lang_class(&class_name);
        Operand::ObjectRef(addr)
    } else {
        Operand::Null
    };
    jenv.thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push(caller_class);
}

pub fn java_io_FileInputStream_initIDs(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {}
pub fn java_io_FileDescriptor_initIDs(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {}
pub fn java_lang_Throwable_fillInStackTrace(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {
    let obj = &args[0];
    jenv.thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push(obj.clone());
}

pub fn java_io_FileOutputStream_initIDs(jenv: &mut JvmEnv, _class: &Class, args: Vec<Operand>) {}

pub fn java_security_AccessController_doPrivileged(
    jenv: &mut JvmEnv,
    class: &Class,
    args: Vec<Operand>,
) {
    let action = &args[0];
    let class_name = jenv.heap.get_class_name(action);
    let class = jenv.load_and_init_class(&class_name);
    let method = class
        .get_method("run", "()Ljava/lang/Object;", false)
        .unwrap();
    execute_method(jenv, method, vec![action.clone()])
}

pub fn java_lang_Thread_currentThread(jenv: &mut JvmEnv, class: &Class, args: Vec<Operand>) {
    jenv.thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push(Operand::ObjectRef(jenv.thread.object_addr));
}

pub fn java_lang_Class_getName0(jenv: &mut JvmEnv, class: &Class, args: Vec<Operand>) {
    let addr = jenv.new_java_lang_string(&class.mirror_class_name());
    jenv.thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push(Operand::ObjectRef(addr));
}

pub fn java_lang_Class_for_Name0(jenv: &mut JvmEnv, class: &Class, args: Vec<Operand>) {
    let name = jenv.get_java_string(&args[0]);
    let class_name = name.replace('.', "/");
    let class = jenv.load_and_init_class(&class_name);
    // let addr = jenv.new_java_lang_string(name);
    // jenv.thread
    //     .stack
    //     .frames
    //     .back_mut()
    //     .unwrap()
    //     .operand_stack
    //     .push(Operand::ObjectRef(addr));
}
