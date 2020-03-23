#![allow(non_snake_case)]

use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::heap::JvmHeap;
use crate::runtime::JvmThread;

pub fn java_java_lang_Class_getPrimitiveClass(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    class: Class,
    mut args: Vec<Operand>,
) -> String {
    let string_ref = args.pop().unwrap();
    heap.get_string(&string_ref)
}
