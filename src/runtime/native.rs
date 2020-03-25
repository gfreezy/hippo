#![allow(non_snake_case)]

use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::heap::JvmHeap;
use crate::runtime::{load_and_init_class, JvmThread};

pub fn java_java_lang_Class_getPrimitiveClass(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    _class: &Class,
    mut args: Vec<Operand>,
) {
    let string_ref = args.pop().unwrap();
    let class_name = heap.get_string(&string_ref);
    let obj_ref = heap.new_class_object(class_name);
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::ObjectRef(obj_ref));
}

pub fn jvm_desiredAssertionStatus(
    _heap: &mut JvmHeap,
    thread: &mut JvmThread,
    _class_loader: &mut ClassLoader,
    _class: &Class,
    _args: Vec<Operand>,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::Int(0));
}
