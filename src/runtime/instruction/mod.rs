#![allow(unused_variables)]
use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::heap::JvmHeap;
use crate::runtime::{did_override_method, execute_method, load_and_init_class, JvmThread};
use tracing::debug;

pub fn iconst_n(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
    n: i32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(n);
}

pub fn fconst_n(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
    n: f32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_float(n);
}

pub fn ldc(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
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
            let s = class.constant_pool().get_utf8_string_at(*string_index);
            let str_ref = heap.new_java_string(s);
            frame.operand_stack.push_object_ref(str_ref)
        }
        ConstPoolInfo::ConstantClassInfo { name_index } => {
            let name = class.constant_pool().get_utf8_string_at(*name_index);
            let obj_ref = heap.new_class_object(name.clone());
            frame.operand_stack.push_object_ref(obj_ref);
        }
        ConstPoolInfo::ConstantMethodHandleInfo { .. } => unimplemented!(),
        ConstPoolInfo::ConstantMethodTypeInfo { .. } => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn store_n(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
    n: i32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    frame.local_variable_array.set(n as u16, val);
}

pub fn store(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let index = code_reader.read_u8().unwrap();
    let val = frame.operand_stack.pop();
    frame.local_variable_array.set(index as u16, val);
}

pub fn iload_n(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
    n: i32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_integer(n as u16);
    frame.operand_stack.push_integer(val);
}

pub fn aload_n(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
    n: i32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_object(n as u16);
    frame.operand_stack.push(val);
}

pub fn fload_n(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
    n: i32,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_float(n as u16);
    frame.operand_stack.push_float(val);
}
pub fn iadd(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 + val2);
}

pub fn ladd(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_long();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 + val2);
}

pub fn invokestatic(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let index = code_reader.read_u16().unwrap();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let class_name = method_ref.class_name;
    let class = load_and_init_class(heap, thread, class_loader, class_name);
    let method = class
        .get_method(method_ref.method_name, method_ref.descriptor, true)
        .expect("get method");

    let frame = thread.stack.frames.back_mut().unwrap();
    let n_args = method.n_args();
    let mut args = Vec::with_capacity(n_args);
    for _ in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    args.reverse();
    execute_method(heap, thread, class_loader, method, args);
}

pub fn ireturn(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    let _ = thread.stack.frames.pop_back();
    let last_frame = thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_integer(val);
}

pub fn dreturn(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_double();
    let _ = thread.stack.frames.pop_back();
    let last_frame = thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_double(val);
}

pub fn freturn(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_float();
    let _ = thread.stack.frames.pop_back();
    let last_frame = thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_float(val);
}

pub fn areturn(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    let _ = thread.stack.frames.pop_back();
    let last_frame = thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push(val);
}

pub fn return_(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let _ = thread.stack.frames.pop_back();
}

pub fn getstatic(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let index = code_reader.read_u16().unwrap();
    let field_ref = class.constant_pool().get_field_ref_at(index);
    let field_class = load_and_init_class(heap, thread, class_loader, field_ref.class_name);
    let field = field_class
        .get_field(field_ref.field_name, field_ref.descriptor)
        .expect(&format!("resolve field: {:?}", field_ref));
    let value = field.value();
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(value)
}

pub fn putstatic(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let index = code_reader.read_u16().unwrap();
    let field_ref = class.constant_pool().get_field_ref_at(index);
    let field_class = load_and_init_class(heap, thread, class_loader, field_ref.class_name);
    let field = field_class
        .get_field(field_ref.field_name, field_ref.descriptor)
        .expect(&format!("resolve field: {:?}", field_ref));
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop();
    field.set_value(value);
}

pub fn aconst_null(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::Null)
}

pub fn invokevirtual(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let index = code_reader.read_u16().unwrap();
    let method_ref = class.constant_pool().get_method_ref_at(index);
    debug!(?method_ref, "invokevirtual");
    let resolved_class = load_and_init_class(heap, thread, class_loader, method_ref.class_name);
    let resolved_method = resolved_class
        .get_method(method_ref.method_name, method_ref.descriptor, false)
        .expect(&format!("get method: {}", &method_ref.method_name));
    let method_class =
        load_and_init_class(heap, thread, class_loader, resolved_method.class_name());
    assert!(
        resolved_method.name() != "<init>" && resolved_method.name() != "<clinit>",
        "<init> and <clinit> are not allowed here"
    );
    let frame = thread.stack.frames.back_mut().unwrap();
    let n_args = resolved_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    let object_ref = frame.operand_stack.pop();
    args.push(object_ref.clone());
    args.reverse();

    let class_name = heap.get_class_name(&object_ref).to_string();
    let object_class = load_and_init_class(heap, thread, class_loader, &class_name);

    if resolved_method.is_native() {
        execute_method(heap, thread, class_loader, resolved_method, args);
        return;
    }

    let acutal_method = if !resolved_method.is_signature_polymorphic() {
        if let Some(actual_method) = object_class
            .get_self_method(resolved_method.name(), resolved_method.descriptor(), false)
            .filter(|m| did_override_method(heap, thread, class_loader, m, &resolved_method))
        {
            actual_method
        } else if let Some(actual_method) = object_class
            .iter_super_classes()
            .filter_map(|klass| {
                klass.get_self_method(resolved_method.name(), resolved_method.descriptor(), false)
            })
            .find(|m| did_override_method(heap, thread, class_loader, m, &resolved_method))
        {
            actual_method
        } else if let Some(actual_method) =
            object_class.get_interface_method(resolved_method.name(), resolved_method.descriptor())
        {
            actual_method
        } else {
            unreachable!("no method found")
        }
    } else {
        unimplemented!("is_signature_polymorphic")
    };

    let actual_class = load_and_init_class(heap, thread, class_loader, acutal_method.class_name());

    execute_method(heap, thread, class_loader, acutal_method, args);
}

pub fn new(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let index = code_reader.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let object_ref = heap.new_object(class_name.clone());
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::ObjectRef(object_ref))
}

pub fn newarray(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let count = frame.operand_stack.pop_integer();
    let atype = code_reader.read_u8().unwrap();
    let array_ref = heap.new_array(atype, count);
    frame.operand_stack.push(Operand::ArrayRef(array_ref))
}

pub fn anewarray(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let count = frame.operand_stack.pop_integer();
    let index = code_reader.read_u16().unwrap();
    let resolved_class_name = class.constant_pool().get_class_name_at(index);
    let array_ref = heap.new_reference_array(resolved_class_name.clone(), count);
    frame.operand_stack.push(Operand::ArrayRef(array_ref))
}

pub fn dup(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    frame.operand_stack.push(val.clone());
    frame.operand_stack.push(val);
}

pub fn castore(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    let index = frame.operand_stack.pop_integer();
    let array_ref = frame.operand_stack.pop();
    let array = heap.get_mut_char_array(array_ref);
    array[index as usize] = val as u16;
}

pub fn invokespecial(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let index = code_reader.read_u16().unwrap();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let resolved_class = load_and_init_class(heap, thread, class_loader, method_ref.class_name);

    let resolved_method = class
        .get_method(method_ref.method_name, method_ref.descriptor, false)
        .expect("get method");

    let actual_class = if !resolved_method.is_initialization_method()
        && (resolved_class.is_interface()
            || (resolved_class.is_class() && class.is_subclass_of(resolved_class.clone())))
        && class.is_static()
    {
        class.super_class().unwrap()
    } else {
        resolved_class
    };

    let actual_method = actual_class
        .get_method(
            resolved_method.name(),
            resolved_method.descriptor(),
            resolved_method.is_static(),
        )
        .unwrap();

    let frame = thread.stack.frames.back_mut().unwrap();
    let n_args = actual_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    let object_ref = frame.operand_stack.pop();
    args.push(object_ref);
    args.reverse();

    if !actual_method.is_native() {
        execute_method(heap, thread, class_loader, actual_method, args);
    } else {
        debug!(method = %actual_method, class = actual_method.class_name(), "invokespecial");
    }
}

pub fn putfield(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let index = code_reader.read_u16().unwrap();
    let field_ref = class.constant_pool().get_field_ref_at(index);
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop();
    let object_ref = frame.operand_stack.pop();
    let object = heap.get_mut_object(object_ref);
    object.set_field(field_ref.field_name.to_string(), value);
}

pub fn getfield(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let index = code_reader.read_u16().unwrap();
    let field_ref = class.constant_pool().get_field_ref_at(index);
    let frame = thread.stack.frames.back_mut().unwrap();
    let object_ref = frame.operand_stack.pop();
    debug!(?object_ref, ?field_ref, "getfield");
    let object = heap.get_object(&object_ref);
    let v = object.get_field(field_ref.field_name).unwrap();
    frame.operand_stack.push(v.clone());
}

pub fn ifge(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value >= 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifgt(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value > 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifle(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value <= 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifeq(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value == 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifne(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value != 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifnonnull(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop();
    if value != Operand::Null {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn goto(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = thread.stack.frames.back_mut().unwrap();
    code_reader.set_pc(pc - 1 + offset as usize);
}

pub fn i2f(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    frame.operand_stack.push_float(value as f32);
}

pub fn f2i(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_float();
    frame.operand_stack.push_integer(value as i32);
}

pub fn i2l(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    frame.operand_stack.push_long(value as i64);
}

pub fn fmul(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_float();
    let value1 = frame.operand_stack.pop_float();
    frame.operand_stack.push_float(value1 * value2);
}

pub fn fcmpg(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_float();
    let value1 = frame.operand_stack.pop_float();
    if value1 > value2 {
        frame.operand_stack.push_integer(1)
    } else if value1 < value2 {
        frame.operand_stack.push_integer(-1)
    } else {
        frame.operand_stack.push_integer(0)
    }
}

pub fn ldc2_w(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let offset = code_reader.read_u16().unwrap();
    let n = match class.constant_pool().get_const_pool_info_at(offset) {
        ConstPoolInfo::ConstantLongInfo(n) => Operand::Long(*n),
        ConstPoolInfo::ConstantDoubleInfo(n) => Operand::Double(*n),
        _ => unreachable!(),
    };
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(n);
}

pub fn sipush(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let n = code_reader.read_u16().unwrap();
    let frame = thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(n as i32);
}

pub fn lshl(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 << (val2 & 0x111111));
}

pub fn land(
    heap: &mut JvmHeap,
    thread: &mut JvmThread,
    class_loader: &mut ClassLoader,
    code_reader: &mut CodeReader,
    class: &Class,
) {
    let frame = thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_long();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 & val2);
}
