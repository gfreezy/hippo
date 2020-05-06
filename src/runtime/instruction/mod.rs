#![allow(unused_variables)]
use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::runtime::class::InstanceClass;
use crate::runtime::code_reader::CodeReader;
use crate::runtime::execute_method;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::jvm_env::JvmEnv;
use tracing::debug;

pub fn iconst_n(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(n);
}

pub fn fconst_n(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass, n: f32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_float(n);
}

pub fn ldc(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let index = code_reader.read_u8().unwrap();
    let const_pool_info = class.constant_pool().get_const_pool_info_at(index as u16);
    match const_pool_info {
        ConstPoolInfo::ConstantIntegerInfo(num) => {
            let frame = jenv.thread.stack.frames.back_mut().unwrap();
            frame.operand_stack.push_integer(*num);
        }
        ConstPoolInfo::ConstantFloatInfo(num) => {
            let frame = jenv.thread.stack.frames.back_mut().unwrap();
            frame.operand_stack.push_float(*num);
        }
        ConstPoolInfo::ConstantStringInfo { string_index } => {
            let s = class.constant_pool().get_utf8_string_at(*string_index);
            let str_ref = jenv.new_java_string(s);
            let frame = jenv.thread.stack.frames.back_mut().unwrap();
            frame.operand_stack.push_object_ref(str_ref)
        }
        ConstPoolInfo::ConstantClassInfo { name_index } => {
            let frame = jenv.thread.stack.frames.back_mut().unwrap();
            let name = class.constant_pool().get_utf8_string_at(*name_index);
            let obj_ref = jenv.heap.new_class_object(name.clone());
            frame.operand_stack.push_object_ref(obj_ref);
        }
        ConstPoolInfo::ConstantMethodHandleInfo { .. } => unimplemented!(),
        ConstPoolInfo::ConstantMethodTypeInfo { .. } => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn store_n(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    frame.local_variable_array.set(n as u16, val);
}

pub fn store(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = code_reader.read_u8().unwrap();
    let val = frame.operand_stack.pop();
    frame.local_variable_array.set(index as u16, val);
}

pub fn aastore(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    let index = frame.operand_stack.pop_integer();
    let array_ref = frame.operand_stack.pop();
    let array = jenv.heap.get_object_array_mut(&array_ref);
    array[index as usize] = val;
}

pub fn iload_n(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_integer(n as u16);
    frame.operand_stack.push_integer(val);
}

pub fn iload(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let index = code_reader.read_u8().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_integer(index as u16);
    frame.operand_stack.push_integer(val);
}

pub fn iinc(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let index = code_reader.read_u8().unwrap();
    // amount is signed
    let amount = code_reader.read_u8().unwrap() as i8 as i32;
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_integer(index as u16);
    frame
        .local_variable_array
        .set_integer(index as u16, val + amount);
}

pub fn aload_n(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_object(n as u16);
    frame.operand_stack.push(val);
}

pub fn aload(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let index = code_reader.read_u8().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_object(index as u16);
    frame.operand_stack.push(val);
}

pub fn aaload(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.operand_stack.pop_integer();
    let array_ref = frame.operand_stack.pop();
    let array = jenv.heap.get_object_array(&array_ref);
    debug!(index, ?array_ref, ?array, "aaload");
    frame.operand_stack.push(array[index as usize].clone());
}

pub fn fload_n(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_float(n as u16);
    frame.operand_stack.push_float(val);
}

pub fn irem(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 % val2);
}

pub fn iadd(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 + val2);
}

pub fn ladd(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_long();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 + val2);
}

pub fn invokestatic(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let index = code_reader.read_u16().unwrap();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let class_name = method_ref.class_name;
    let class = jenv.load_and_init_class(class_name);
    let method = class
        .get_method(method_ref.method_name, method_ref.descriptor, true)
        .expect("get method");

    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let n_args = method.n_args();
    let mut args = Vec::with_capacity(n_args);
    for _ in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    args.reverse();
    execute_method(jenv, method, args);
}

pub fn ireturn(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    let _ = jenv.thread.stack.frames.pop_back();
    let last_frame = jenv.thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_integer(val);
}

pub fn dreturn(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_double();
    let _ = jenv.thread.stack.frames.pop_back();
    let last_frame = jenv.thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_double(val);
}

pub fn freturn(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_float();
    let _ = jenv.thread.stack.frames.pop_back();
    let last_frame = jenv.thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_float(val);
}

pub fn areturn(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    let _ = jenv.thread.stack.frames.pop_back();
    let last_frame = jenv.thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push(val);
}

pub fn return_(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let _ = jenv.thread.stack.frames.pop_back();
}

pub fn getstatic(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let opcode_pc = code_reader.pc() - 1;
    let index = code_reader.read_u16().unwrap();
    let method = code_reader.method();
    let field_index = if let Some(field_index) = method.resolve_field(opcode_pc) {
        field_index
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let field_class = jenv.load_and_init_class(field_ref.class_name);
        let field = field_class
            .get_static_field(field_ref.field_name, field_ref.descriptor)
            .expect(&format!("resolve field: {:?}", field_ref));
        let field_index = field.index();
        method.set_field(opcode_pc, field_index);
        field_index
    };

    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame
        .operand_stack
        .push(class.get_static_field_value(field_index))
}

pub fn putstatic(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let opcode_pc = code_reader.pc() - 1;
    let index = code_reader.read_u16().unwrap();
    let method = code_reader.method();
    let field_index = if let Some(field_index) = method.resolve_field(opcode_pc) {
        field_index
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let field_class = jenv.load_and_init_class(field_ref.class_name);
        let field = field_class
            .get_static_field(field_ref.field_name, field_ref.descriptor)
            .expect(&format!("resolve field: {:?}", field_ref));
        let field_index = field.index();
        method.set_field(opcode_pc, field_index);
        field_index
    };
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop();
    class.set_static_field_value(field_index, value);
}

pub fn aconst_null(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::Null)
}

pub fn invokevirtual(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let index = code_reader.read_u16().unwrap();
    let method_ref = class.constant_pool().get_method_ref_at(index);
    debug!(?method_ref, "invokevirtual");
    let resolved_class = jenv.load_and_init_class(method_ref.class_name);
    let resolved_method = resolved_class
        .get_method(method_ref.method_name, method_ref.descriptor, false)
        .expect(&format!("get method: {}", &method_ref.method_name));
    assert!(
        resolved_method.name() != "<init>" && resolved_method.name() != "<clinit>",
        "<init> and <clinit> are not allowed here"
    );
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let n_args = resolved_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    let object_ref = frame.operand_stack.pop();
    args.push(object_ref.clone());
    args.reverse();

    if resolved_method.is_native() {
        execute_method(jenv, resolved_method, args);
        return;
    }

    let class_name = jenv.heap.get_class_name(&object_ref).to_string();
    let object_class = jenv.load_and_init_class(&class_name);

    let acutal_method = if !resolved_method.is_signature_polymorphic() {
        if let Some(actual_method) = object_class
            .get_self_method(resolved_method.name(), resolved_method.descriptor(), false)
            .filter(|m| jenv.did_override_method(m, &resolved_method))
        {
            actual_method
        } else if let Some(actual_method) = object_class
            .iter_super_classes()
            .filter_map(|klass| {
                klass.get_self_method(resolved_method.name(), resolved_method.descriptor(), false)
            })
            .find(|m| jenv.did_override_method(m, &resolved_method))
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

    let actual_class = jenv.load_and_init_class(acutal_method.class_name());

    execute_method(jenv, acutal_method, args);
}

pub fn new(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let index = code_reader.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = jenv.load_and_init_class(class_name);
    let object_ref = jenv.heap.new_object(class);
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::ObjectRef(object_ref))
}

pub fn newarray(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let count = frame.operand_stack.pop_integer();
    let atype = code_reader.read_u8().unwrap();
    let array_ref = jenv.heap.new_empty_array(atype, count);
    frame.operand_stack.push(Operand::ArrayRef(array_ref))
}

pub fn anewarray(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let count = frame.operand_stack.pop_integer();
    let index = code_reader.read_u16().unwrap();
    let resolved_class_name = class.constant_pool().get_class_name_at(index);
    let array_ref = jenv
        .heap
        .new_reference_array(resolved_class_name.clone(), count);
    frame.operand_stack.push(Operand::ArrayRef(array_ref))
}

pub fn arraylength(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let array_ref = frame.operand_stack.pop();
    let len = jenv.heap.get_array_length(&array_ref);
    frame.operand_stack.push_integer(len);
}

pub fn dup(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    frame.operand_stack.push(val.clone());
    frame.operand_stack.push(val);
}

pub fn castore(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    let index = frame.operand_stack.pop_integer();
    let array_ref = frame.operand_stack.pop();
    let array = jenv.heap.get_mut_char_array(array_ref);
    array[index as usize] = val as u16;
}

pub fn invokespecial(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let index = code_reader.read_u16().unwrap();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let resolved_class = jenv.load_and_init_class(method_ref.class_name);

    let resolved_method = resolved_class
        .get_method(method_ref.method_name, method_ref.descriptor, false)
        .expect(&format!(
            "get method: {}, descriptor: {}, class: {}",
            method_ref.method_name,
            method_ref.descriptor,
            resolved_class.name()
        ));

    let actual_class = if !resolved_method.is_initialization_method()
        && (resolved_class.is_interface()
            || (resolved_class.is_class() && class.is_subclass_of(resolved_class.clone())))
        && class.is_super()
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
        .expect(&format!(
            "class: {}, method: {}, descriptor: {}",
            actual_class.name(),
            resolved_method.name(),
            resolved_method.descriptor()
        ));

    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let n_args = actual_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    let object_ref = frame.operand_stack.pop();
    args.push(object_ref);
    args.reverse();

    if !actual_method.is_native() {
        execute_method(jenv, actual_method, args);
    } else {
        debug!(method = %actual_method, class = actual_method.class_name(), "invokespecial");
    }
}

pub fn putfield(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let opcode_pc = code_reader.pc() - 1;
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = code_reader.read_u16().unwrap();
    let value = frame.operand_stack.pop();
    let object_ref = frame.operand_stack.pop();

    let method = code_reader.method();
    let field_index = if let Some(index) = method.resolve_field(opcode_pc) {
        index
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        debug!(?object_ref, ?field_ref, "putfield");
        let object_class_name = jenv.heap.get_object(&object_ref).class_name().to_string();
        let obj_class = jenv.load_and_init_class(&object_class_name);
        let class_field = obj_class
            .get_field(field_ref.field_name, field_ref.descriptor)
            .unwrap();
        let index = class_field.index();
        method.set_field(opcode_pc, index);
        index
    };
    let obj = jenv.heap.get_object_mut(&object_ref);
    obj.set_field(field_index, value);
}

pub fn getfield(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let opcode_pc = code_reader.pc() - 1;
    let index = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let object_ref = frame.operand_stack.pop();

    let method = code_reader.method();
    let field_index = if let Some(index) = method.resolve_field(opcode_pc) {
        index
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let object_class_name = jenv.heap.get_object(&object_ref).class_name().to_string();
        let obj_class = jenv.load_and_init_class(&object_class_name);
        let class_field = obj_class
            .get_field(field_ref.field_name, field_ref.descriptor)
            .unwrap();
        let index = class_field.index();
        debug!(?object_ref, ?field_ref, index, "getfield");
        method.set_field(opcode_pc, index);
        index
    };
    let obj = jenv.heap.get_object(&object_ref);
    let field = obj.get_field(field_index).clone();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(field);
}

pub fn ifge(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value >= 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifgt(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value > 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifle(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value <= 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn if_icmpeq(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 == value2 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn if_icmpne(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 != value2 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn if_icmplt(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 < value2 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn if_icmple(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 <= value2 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn if_icmpgt(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 > value2 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn if_icmpge(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 >= value2 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifeq(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value == 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifne(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    if value != 0 {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifnonnull(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop();
    if value != Operand::Null {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}

pub fn ifnull(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop();
    if value == Operand::Null {
        code_reader.set_pc(pc - 1 + offset as usize);
    }
}
pub fn goto(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let pc = code_reader.pc();
    let offset = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    code_reader.set_pc(pc - 1 + offset as usize);
}

pub fn i2f(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    frame.operand_stack.push_float(value as f32);
}

pub fn f2i(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_float();
    frame.operand_stack.push_integer(value as i32);
}

pub fn i2l(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    frame.operand_stack.push_long(value as i64);
}

pub fn fmul(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_float();
    let value1 = frame.operand_stack.pop_float();
    frame.operand_stack.push_float(value1 * value2);
}

pub fn fcmpg(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
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

pub fn ldc2_w(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let offset = code_reader.read_u16().unwrap();
    let n = match class.constant_pool().get_const_pool_info_at(offset) {
        ConstPoolInfo::ConstantLongInfo(n) => Operand::Long(*n),
        ConstPoolInfo::ConstantDoubleInfo(n) => Operand::Double(*n),
        _ => unreachable!(),
    };
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(n);
}

pub fn sipush(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let n = code_reader.read_u16().unwrap();
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(n as i32);
}

pub fn lshl(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 << (val2 & 0x111111));
}

pub fn ishl(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 << (val2 & 0x111111));
}

pub fn land(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_long();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 & val2);
}

pub fn iand(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 & val2);
}

pub fn isub(jenv: &mut JvmEnv, code_reader: &mut CodeReader, class: &InstanceClass) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 - val2);
}
