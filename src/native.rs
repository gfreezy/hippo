#![allow(non_snake_case, unused_variables)]

use crate::class::{alloc_empty_jobject, alloc_jobject, Class};
use crate::class_loader::{get_class_by_id, load_class};
use crate::gc::global_definition::{JObject, JValue};
use crate::jenv::{get_java_string, new_java_lang_string};
use crate::jthread::JvmThread;
use crate::jvm::execute_method;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

pub fn java_lang_Class_getPrimitiveClass(
    thread: &mut JvmThread,
    class: &Class,
    mut args: Vec<JValue>,
) {
    static PRIMITIVE_CLASSES: OnceCell<HashMap<String, JObject>> = OnceCell::new();
    let primitive_classes = PRIMITIVE_CLASSES.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("float".to_string(), alloc_empty_jobject());
        map.insert("int".to_string(), alloc_empty_jobject());
        map.insert("double".to_string(), alloc_empty_jobject());
        map.insert("short".to_string(), alloc_empty_jobject());
        map.insert("byte".to_string(), alloc_empty_jobject());
        map
    });
    let string_ref = args.pop().unwrap();
    let class_name = get_java_string(string_ref.as_jobject());
    let primitive_class = primitive_classes.get(&class_name).unwrap();
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jobject(primitive_class.clone());
}

pub fn jvm_desiredAssertionStatus0(thread: &mut JvmThread, _class: &Class, _args: Vec<JValue>) {
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jint(0);
}

pub fn java_lang_Float_floatToRawIntBits(
    thread: &mut JvmThread,
    _class: &Class,
    args: Vec<JValue>,
) {
    let n = args[0].as_jfloat();
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jint(n.to_bits() as i32);
}

pub fn java_lang_Double_doubleToRawLongBits(
    thread: &mut JvmThread,
    _class: &Class,
    args: Vec<JValue>,
) {
    let n = args[0].as_jdouble();
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jlong(n.to_bits() as i64);
}

pub fn java_lang_Double_longBitsToDouble(
    thread: &mut JvmThread,
    _class: &Class,
    args: Vec<JValue>,
) {
    let n = args[0].as_jlong();
    let frame = thread.current_frame_mut();
    frame
        .operand_stack
        .push_jdouble(f64::from_be_bytes(n.to_be_bytes()));
}

pub fn java_lang_System_initProperties(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    let props_ref = &args[0];
    let properties = props_ref.as_jobject();
    let class_id = properties.class_id();
    let propertiesClass = get_class_by_id(class_id);
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
        let key = JValue::Object(new_java_lang_string(key));
        let value = JValue::Object(new_java_lang_string(value));
        let args = vec![props_ref.clone(), key, value];
        execute_method(thread, method.clone(), args);
    }
    let frame = thread.current_frame_mut();
    frame.operand_stack.push(props_ref.clone());
}

pub fn java_lang_Object_hashCode(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    let obj = &args[0].as_jobject();
    thread
        .current_frame_mut()
        .operand_stack
        .push_jint(obj.hash_code());
}

pub fn java_lang_System_registerNatives(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
}

pub fn java_lang_Object_registerNatives(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
}

pub fn registerNatives(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {}

pub fn sun_misc_VM_initalize(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {}

pub fn sun_misc_Unsafe_registerNatives(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {}

pub fn sun_misc_Unsafe_arrayBaseOffset(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    thread.current_frame_mut().operand_stack.push_jint(0);
}

pub fn sun_misc_Unsafe_arrayIndexScale(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    thread.current_frame_mut().operand_stack.push_jint(1);
}

pub fn sun_misc_Unsafe_addressSize(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    thread.current_frame_mut().operand_stack.push_jint(8);
}

pub fn sun_reflect_Reflection_getCallerClass(
    thread: &mut JvmThread,
    class: &Class,
    args: Vec<JValue>,
) {
    let caller_class = if let Some(class) = thread.caller_class() {
        class.mirror_class()
    } else {
        JObject::null()
    };
    thread
        .current_frame_mut()
        .operand_stack
        .push_jobject(caller_class);
}

pub fn java_io_FileInputStream_initIDs(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {}
pub fn java_io_FileDescriptor_initIDs(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {}
pub fn java_lang_Throwable_fillInStackTrace(
    thread: &mut JvmThread,
    _class: &Class,
    args: Vec<JValue>,
) {
    let obj = &args[0];
    thread.current_frame_mut().operand_stack.push(obj.clone());
}

pub fn java_io_FileOutputStream_initIDs(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
}

pub fn java_security_AccessController_doPrivileged(
    thread: &mut JvmThread,
    class: &Class,
    args: Vec<JValue>,
) {
    let action = args[0].as_jobject();
    let class_id = action.class_id();
    let class = get_class_by_id(class_id);
    let method = class
        .get_method("run", "()Ljava/lang/Object;", false)
        .unwrap();
    execute_method(thread, method, vec![JValue::Object(action)])
}

pub fn java_lang_Thread_currentThread(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let t = thread.object.clone();
    thread.current_frame_mut().operand_stack.push_jobject(t)
}

pub fn java_lang_Class_getName0(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let addr = new_java_lang_string(class.as_instance_mirror_class().mirrored_class_name());
    thread.current_frame_mut().operand_stack.push_jobject(addr);
}

pub fn java_lang_Class_for_Name0(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let name = get_java_string(args[0].as_jobject());
    let class_name = name.replace('.', "/");
    eprintln!("class_for_Name0: {}", &class_name);
    let class = load_class(class.class_loader(), &class_name);
    thread
        .current_frame_mut()
        .operand_stack
        .push_jobject(class.mirror_class());
}

pub fn java_security_AccessController_getStackAccessControlContext(
    thread: &mut JvmThread,
    class: &Class,
    args: Vec<JValue>,
) {
    thread
        .current_frame_mut()
        .operand_stack
        .push_jobject(JObject::null());
}

pub fn java_lang_Thread_setPriority0(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let priority = args[1].as_jint();
    if priority < 1 {
        let object_ref = args[0].as_jobject();
        let class_id = object_ref.class_id();
        let class = get_class_by_id(class_id);
        let field = class.get_field("priority", "I").unwrap();
        object_ref.set_field_by_offset(field.offset(), 5);
    }
}
