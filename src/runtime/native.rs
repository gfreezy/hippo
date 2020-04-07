#![allow(non_snake_case)]

use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::heap::JvmHeap;
use crate::runtime::{execute_method, load_and_init_class, JvmThread};

pub fn java_java_lang_Class_getPrimitiveClass(
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

pub fn java_java_lang_Float_floatToRawIntBits(
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

pub fn java_java_lang_Double_doubleToRawLongBits(
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

pub fn java_java_lang_Double_longBitsToDouble(
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
    class: &Class,
    args: Vec<Operand>,
) {
    let props_ref = &args[0];
    let properties = heap.get_object(props_ref);
    let propertiesClass = load_and_init_class(heap, thread, class_loader, properties.class_name());
    let method = propertiesClass
        .get_method(
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            false,
        )
        .unwrap();
    let args = vec![];
    execute_method(heap, thread, class_loader, method, args);
    let frame = thread.stack.frames.back_mut().unwrap();
    frame
        .operand_stack
        .push_double(f64::from_be_bytes(n.to_be_bytes()));
}
