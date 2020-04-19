#![allow(non_snake_case)]

use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::heap::JvmHeap;
use crate::runtime::{execute_method, load_and_init_class, JvmThread};

pub fn java_lang_Class_getPrimitiveClass(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    _class_loader: &mut ClassLoader,
    _class: &Class,
    mut args: Vec<Operand>,
) {
    let string_ref = args.pop().unwrap();
    let class_name = heap.get_string(&string_ref);
    let obj_ref = heap.new_class_object(class_name);
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::ObjectRef(obj_ref));
}

pub fn jvm_desiredAssertionStatus0(
    _heap: &mut JvmHeap,
    thread: &mut JvmThread,
    _class_loader: &mut ClassLoader,
    _class: &Class,
    _args: Vec<Operand>,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(0);
}

pub fn java_lang_Float_floatToRawIntBits(
    _heap: &mut JvmHeap,
    thread: &mut JvmThread,
    _class_loader: &mut ClassLoader,
    _class: &Class,
    args: Vec<Operand>,
) {
    let n = args[0].get_float();
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(n.to_bits() as i32);
}

pub fn java_lang_Double_doubleToRawLongBits(
    _heap: &mut JvmHeap,
    thread: &mut JvmThread,
    _class_loader: &mut ClassLoader,
    _class: &Class,
    args: Vec<Operand>,
) {
    let n = args[0].get_double();
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_long(n.to_bits() as i64);
}

pub fn java_lang_Double_longBitsToDouble(
    _heap: &mut JvmHeap,
    thread: &mut JvmThread,
    _class_loader: &mut ClassLoader,
    _class: &Class,
    args: Vec<Operand>,
) {
    let n = args[0].get_long();
    let frame = thread.stack.frames.back_mut().unwrap();
    frame
        .operand_stack
        .push_double(f64::from_be_bytes(n.to_be_bytes()));
}

pub fn java_lang_System_initProperties(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    _class: &Class,
    args: Vec<Operand>,
) {
    let props_ref = &args[0];
    let properties = heap.get_object(props_ref);
    let class_name = properties.class_name().to_string();
    let propertiesClass = load_and_init_class(heap, thread, class_loader, &class_name);
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
        let key = Operand::ObjectRef(heap.new_java_string(key, thread, class_loader));
        let value = Operand::ObjectRef(heap.new_java_string(value, thread, class_loader));
        let args = vec![props_ref.clone(), key, value];
        execute_method(heap, thread, class_loader, method.clone(), args);
    }
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(props_ref.clone());
}

pub fn java_lang_Object_hashCode(
    _heap: &mut JvmHeap,
    thread: &mut JvmThread,
    _class_loader: &mut ClassLoader,
    _class: &Class,
    args: Vec<Operand>,
) {
    let obj = &args[0];
    thread
        .stack
        .frames
        .back_mut()
        .unwrap()
        .operand_stack
        .push_integer(obj.hash_code());
}
