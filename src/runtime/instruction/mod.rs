#![allow(unused_variables)]
use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::{execute_method, load_and_init_class, JvmThread};

pub fn iconst_n(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
    n: i32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(n);
}

pub fn ldc(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let index = code_reader.read_u8().unwrap();
    let const_pool_info = class.constant_pool().get_const_pool_info_at(index as u16);
    match const_pool_info {
        ConstPoolInfo::ConstantIntegerInfo(num) => {
            frame.operand_stack.push_integer(*num);
        }
        ConstPoolInfo::ConstantFloatInfo(num) => {
            frame.operand_stack.push_float(*num);
        }
        ConstPoolInfo::ConstantStringInfo { string_index } => {
            frame.operand_stack.push_object_ref(*string_index)
        }
        ConstPoolInfo::ConstantClassInfo { name_index } => {
            let name = class.constant_pool().get_utf8_string_at(*name_index);
            let _class = class_loader.load_class(name.clone());
            frame.operand_stack.push_class_ref(name.clone());
        }
        ConstPoolInfo::ConstantMethodHandleInfo { .. } => unimplemented!(),
        ConstPoolInfo::ConstantMethodTypeInfo { .. } => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn istore_n(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
    n: i32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    frame.local_variable_array.set_integer(n as u16, val);
}

pub fn istore(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let index = code_reader.read_u8().unwrap();
    let val = frame.operand_stack.pop_integer();
    frame.local_variable_array.set_integer(index as u16, val);
}

pub fn iload_n(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
    n: i32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_integer(n as u16);
    frame.operand_stack.push_integer(val);
}

pub fn iadd(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val1 = frame.operand_stack.pop_integer();
    let val2 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 + val2);
}

pub fn invokestatic(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let index = code_reader.read_u16().unwrap();
    let const_pool_info = class.constant_pool().get_const_pool_info_at(index);
    let constant_pool = class.constant_pool();
    match const_pool_info {
        ConstPoolInfo::ConstantMethodRefInfo {
            class_index,
            name_and_type_index,
        }
        | ConstPoolInfo::ConstantInterfaceMethodRefInfo {
            class_index,
            name_and_type_index,
        } => {
            let class_name = constant_pool.get_class_name_at(*class_index);
            let class = load_and_init_class(thread, class_loader, class_name.clone());
            let (method_name, method_type) =
                constant_pool.get_name_and_type_at(*name_and_type_index);

            let method = class
                .get_method(method_name, method_type, true)
                .expect("get method");

            let frame = thread.stack.frames.back_mut().unwrap();
            let n_args = method.parameters().len();
            let mut args = Vec::with_capacity(n_args);
            for _ in 0..n_args {
                args.push(frame.operand_stack.pop());
            }
            execute_method(thread, class_loader, class, method, Some(args));
        }
        ConstPoolInfo::ConstantClassInfo { name_index } => {
            let class_name = constant_pool.get_utf8_string_at(*name_index);
            panic!("{}", class_name);
        }
        _ => unreachable!("{:?}", const_pool_info),
    }
}

pub fn ireturn(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    let _ = thread.stack.frames.pop_back();
    let last_frame = thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_integer(val);
}

pub fn return_(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let _ = thread.stack.frames.pop_back();
}

pub fn getstatic(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let index = code_reader.read_u16().unwrap();
    let field_ref = class.constant_pool().get_field_ref_at(index);
    let field_class = load_and_init_class(thread, class_loader, field_ref.class_name.to_string());
    let field = field_class
        .get_field(field_ref.field_name, field_ref.descriptor)
        .expect(&format!("resolve field: {:?}", field_ref));
    let value = field.value();
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(value)
}

pub fn putstatic(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let index = code_reader.read_u16().unwrap();
    let field_ref = class.constant_pool().get_field_ref_at(index);
    let field_class = load_and_init_class(thread, class_loader, field_ref.class_name.to_string());
    let field = field_class
        .get_field(field_ref.field_name, field_ref.descriptor)
        .expect(&format!("resolve field: {:?}", field_ref));
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop();
    field.set_value(value);
}

pub fn aconst_null(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::ObjectRef(0))
}

pub fn invokevirtual(
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: Class,
) {
    //todo: not finished
    let index = code_reader.read_u16().unwrap();
    let method_ref = class.constant_pool().get_method_ref_at(index);

    let class = load_and_init_class(thread, class_loader, method_ref.class_name.to_string());
    let method = class.get_method(method_ref.method_name, method_ref.descriptor, false);

    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::ObjectRef(0))
}
