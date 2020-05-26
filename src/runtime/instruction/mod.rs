#![allow(unused_variables)]
use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::runtime::class::Class;
use crate::runtime::execute_method;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::heap::JAVA_LANG_OBJECT;
use crate::runtime::jvm_env::JvmEnv;
use tracing::debug;

pub fn iconst_n(jenv: &mut JvmEnv, class: &Class, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(n);
}

pub fn lconst_n(jenv: &mut JvmEnv, class: &Class, n: i64) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_long(n);
}

pub fn fconst_n(jenv: &mut JvmEnv, class: &Class, n: f32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_float(n);
}

pub fn ldc(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u8().unwrap();
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
            let str_ref = jenv.new_java_lang_string(s);
            let frame = jenv.thread.stack.frames.back_mut().unwrap();
            frame.operand_stack.push_object_ref(str_ref)
        }
        ConstPoolInfo::ConstantClassInfo { name_index } => {
            let name = class.constant_pool().get_utf8_string_at(*name_index);
            let addr = jenv.new_java_lang_class(name);
            let frame = jenv.thread.stack.frames.back_mut().unwrap();
            frame.operand_stack.push_object_ref(addr);
        }
        ConstPoolInfo::ConstantMethodHandleInfo { .. } => unimplemented!(),
        ConstPoolInfo::ConstantMethodTypeInfo { .. } => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn istore_n(jenv: &mut JvmEnv, class: &Class, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    frame.local_variable_array.set(n as u16, Operand::Int(val));
}

pub fn istore(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u8().unwrap();
    let val = frame.operand_stack.pop_integer();
    frame
        .local_variable_array
        .set(index as u16, Operand::Int(val));
}

pub fn astore_n(jenv: &mut JvmEnv, class: &Class, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    frame.local_variable_array.set(n as u16, val);
}

pub fn astore(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u8().unwrap();
    let val = frame.operand_stack.pop();
    frame.local_variable_array.set(index as u16, val);
}

pub fn aastore(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    let index = frame.operand_stack.pop_integer();
    let array_ref = frame.operand_stack.pop();
    let array = jenv.heap.get_object_array_mut(&array_ref);
    array[index as usize] = val;
}

pub fn iload_n(jenv: &mut JvmEnv, class: &Class, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_integer(n as u16);
    frame.operand_stack.push_integer(val);
}

pub fn lload_n(jenv: &mut JvmEnv, class: &Class, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_long(n as u16);
    frame.operand_stack.push_long(val);
}

pub fn lload(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u8().unwrap();
    let val = frame.local_variable_array.get_long(index as u16);
    frame.operand_stack.push_long(val);
}

pub fn iload(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u8().unwrap();
    let val = frame.local_variable_array.get_integer(index as u16);
    frame.operand_stack.push_integer(val);
}

pub fn iinc(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u8().unwrap();
    // amount is signed
    let amount = frame.read_u8().unwrap() as i8 as i32;
    let val = frame.local_variable_array.get_integer(index as u16);
    frame
        .local_variable_array
        .set_integer(index as u16, val + amount);
}

pub fn aload_n(jenv: &mut JvmEnv, class: &Class, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_object(n as u16);
    frame.operand_stack.push(val);
}

pub fn aload(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u8().unwrap();
    let val = frame.local_variable_array.get_object(index as u16);
    frame.operand_stack.push(val);
}

pub fn aaload(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.operand_stack.pop_integer();
    let array_ref = frame.operand_stack.pop();
    let array = jenv.heap.get_object_array(&array_ref);
    debug!(index, ?array_ref, ?array, "aaload");
    frame.operand_stack.push(array[index as usize].clone());
}

pub fn caload(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.operand_stack.pop_integer();
    let array_ref = frame.operand_stack.pop();
    let array = jenv.heap.get_char_array(&array_ref);
    debug!(index, ?array_ref, ?array, "caload");
    frame
        .operand_stack
        .push_integer(array[index as usize] as i32);
}

pub fn fload_n(jenv: &mut JvmEnv, class: &Class, n: i32) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.local_variable_array.get_float(n as u16);
    frame.operand_stack.push_float(val);
}

pub fn irem(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 % val2);
}

pub fn iadd(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 + val2);
}

pub fn ladd(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_long();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 + val2);
}

pub fn invokestatic(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u16().unwrap();
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

pub fn ireturn(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    let _ = jenv.thread.stack.frames.pop_back();
    let last_frame = jenv.thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_integer(val);
}

pub fn dreturn(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_double();
    let _ = jenv.thread.stack.frames.pop_back();
    let last_frame = jenv.thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_double(val);
}

pub fn freturn(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_float();
    let _ = jenv.thread.stack.frames.pop_back();
    let last_frame = jenv.thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push_float(val);
}

pub fn areturn(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    let _ = jenv.thread.stack.frames.pop_back();
    let last_frame = jenv.thread.stack.frames.back_mut().unwrap();
    last_frame.operand_stack.push(val);
}

pub fn return_(jenv: &mut JvmEnv, class: &Class) {
    let _ = jenv.thread.stack.frames.pop_back();
}

pub fn getstatic(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let opcode_pc = frame.pc() - 1;
    let index = frame.read_u16().unwrap();
    let method = frame.method();
    let (field_class, field_index) = if let Some(v) = method.resolve_static_field(opcode_pc) {
        v
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let field_class = jenv.load_and_init_class(field_ref.class_name);
        let field = field_class
            .get_static_field(field_ref.field_name, field_ref.descriptor)
            .unwrap_or_else(|| panic!("resolve field: {:?}", field_ref));
        let field_index = field.index();
        method.set_static_field(opcode_pc, field_class.clone(), field_index);
        debug!(?field_ref, %field_index, ?class, field=?field_class.get_static_field_value(field_index), "getstatic");

        (field_class, field_index)
    };

    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame
        .operand_stack
        .push(field_class.get_static_field_value(field_index))
}

pub fn putstatic(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let opcode_pc = frame.pc() - 1;
    let index = frame.read_u16().unwrap();
    let value = frame.operand_stack.pop();
    let method = frame.method();
    let (field_class, field_index) = if let Some(v) = method.resolve_static_field(opcode_pc) {
        v
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let field_class = jenv.load_and_init_class(field_ref.class_name);
        let field = field_class
            .get_static_field(field_ref.field_name, field_ref.descriptor)
            .unwrap_or_else(|| panic!("resolve field: {:?}", field_ref));
        let field_index = field.index();
        debug!(?field_ref, %field_index, ?class, field=?value, "putstatic");
        method.set_static_field(opcode_pc, field_class.clone(), field_index);
        (field_class, field_index)
    };

    field_class.set_static_field_value(field_index, value);
}

pub fn aconst_null(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::Null)
}

pub fn invokevirtual(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u16().unwrap();
    let method_ref = class.constant_pool().get_method_ref_at(index);
    debug!(?method_ref, "invokevirtual");
    let resolved_class = jenv.load_and_init_class(method_ref.class_name);
    let resolved_method = resolved_class
        .get_method(method_ref.method_name, method_ref.descriptor, false)
        .unwrap_or_else(|| panic!("get method: {}", &method_ref.method_name));
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

    let class_name = jenv.heap.get_class_name(&object_ref);
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

pub fn invokeinterface(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u16().unwrap();
    let count = frame.read_u8().unwrap();
    assert_ne!(count, 0);
    let forth = frame.read_u8().unwrap();
    assert_eq!(forth, 0);
    let method_ref = class.constant_pool().get_interface_method_ref_at(index);
    debug!(?method_ref, "invokeinterface");
    let resolved_class = jenv.load_and_init_class(method_ref.class_name);
    let resolved_method = resolved_class
        .get_interface_method(method_ref.method_name, method_ref.descriptor)
        .unwrap_or_else(|| panic!("get interface method: {}", &method_ref.method_name));
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

    let class_name = jenv.heap.get_class_name(&object_ref);
    let object_class = jenv.load_and_init_class(&class_name);

    let acutal_method = if let Some(actual_method) =
        object_class.get_self_method(resolved_method.name(), resolved_method.descriptor(), false)
    {
        actual_method
    } else if let Some(actual_method) = object_class.iter_super_classes().find_map(|klass| {
        klass.get_self_method(resolved_method.name(), resolved_method.descriptor(), false)
    }) {
        actual_method
    } else if let Some(actual_method) =
        object_class.get_interface_method(resolved_method.name(), resolved_method.descriptor())
    {
        actual_method
    } else {
        unreachable!("no method found")
    };

    let actual_class = jenv.load_and_init_class(acutal_method.class_name());

    execute_method(jenv, acutal_method, args);
}

pub fn new(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = jenv.load_and_init_class(class_name);
    let (object, addr) = jenv.heap.new_object(class);
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(Operand::ObjectRef(addr))
}

pub fn newarray(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let count = frame.operand_stack.pop_integer();
    let atype = frame.read_u8().unwrap();
    let array_ref = jenv.heap.new_empty_array(atype, count);
    frame.operand_stack.push(Operand::ArrayRef(array_ref))
}

pub fn anewarray(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let count = frame.operand_stack.pop_integer();
    let index = frame.read_u16().unwrap();
    let resolved_class_name = class.constant_pool().get_class_name_at(index);
    let array_ref = jenv
        .heap
        .new_reference_array(resolved_class_name.clone(), count);
    frame.operand_stack.push(Operand::ArrayRef(array_ref))
}

pub fn arraylength(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let array_ref = frame.operand_stack.pop();
    let len = jenv.heap.get_array_length(&array_ref);
    frame.operand_stack.push_integer(len);
}

pub fn pop(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let _ = frame.operand_stack.pop();
}

fn can_cast_to(jenv: &mut JvmEnv, s: Class, t: Class) -> bool {
    match (s, t) {
        (Class::InstanceClass(s), Class::InstanceClass(t))
            if (s.is_class() && t.is_class()) || (s.is_interface() && t.is_interface()) =>
        {
            s == t || s.is_subclass_of(t)
        }
        (Class::InstanceClass(s), Class::InstanceClass(t)) if s.is_class() && t.is_interface() => {
            s.did_implement_interface(t)
        }
        (Class::InstanceClass(s), Class::InstanceClass(t)) if s.is_interface() && t.is_class() => {
            t.name() == JAVA_LANG_OBJECT
        }
        (s, Class::InstanceClass(t)) if t.is_class() => t.name() == JAVA_LANG_OBJECT,
        (s, Class::InstanceClass(t)) if t.is_interface() => unimplemented!(),
        (Class::TypeArrayClass(s), Class::TypeArrayClass(t)) => s.ty == t.ty,
        (Class::ObjArrayClass(s), Class::ObjArrayClass(t)) => {
            let sc = jenv.load_and_init_class(&s.class);
            let tc = jenv.load_and_init_class(&t.class);
            can_cast_to(jenv, sc, tc)
        }
        _ => false,
    }
}

pub fn checkcast(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = jenv.load_and_init_class(class_name);
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let obj_ref = frame.operand_stack.pop();
    if obj_ref == Operand::Null {
        frame.operand_stack.push(obj_ref);
        return;
    }
    let obj_class_name = jenv.heap.get_class_name(&obj_ref);
    let obj_class = jenv.load_and_init_class(&obj_class_name);

    assert!(can_cast_to(jenv, obj_class, class));

    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(obj_ref);
}

pub fn dup(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop();
    frame.operand_stack.push(val.clone());
    frame.operand_stack.push(val);
}

pub fn dup_x1(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val1 = frame.operand_stack.pop();
    let val2 = frame.operand_stack.pop();
    frame.operand_stack.push(val1.clone());
    frame.operand_stack.push(val2);
    frame.operand_stack.push(val1);
}

pub fn castore(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val = frame.operand_stack.pop_integer();
    let index = frame.operand_stack.pop_integer();
    let array_ref = frame.operand_stack.pop();
    let array = jenv.heap.get_mut_char_array(array_ref);
    array[index as usize] = val as u16;
}

pub fn invokespecial(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u16().unwrap();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let resolved_class = jenv.load_and_init_class(method_ref.class_name);

    let resolved_method = resolved_class
        .get_method(method_ref.method_name, method_ref.descriptor, false)
        .unwrap_or_else(|| {
            panic!(
                "get method: {}, descriptor: {}, class: {}",
                method_ref.method_name,
                method_ref.descriptor,
                resolved_class.name()
            )
        });

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
        .unwrap_or_else(|| {
            panic!(
                "class: {}, method: {}, descriptor: {}",
                actual_class.name(),
                resolved_method.name(),
                resolved_method.descriptor()
            )
        });

    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let n_args = actual_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    let object_ref = frame.operand_stack.pop();
    args.push(object_ref);
    args.reverse();

    execute_method(jenv, actual_method, args);
}

pub fn putfield(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let opcode_pc = frame.pc() - 1;
    let index = frame.read_u16().unwrap();
    let value = frame.operand_stack.pop();
    let object_ref = frame.operand_stack.pop();

    let method = frame.method();
    let field_index = if let Some(index) = method.resolve_field(opcode_pc) {
        index
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        debug!(?object_ref, ?field_ref, "putfield");
        let object_class_name = jenv.heap.get_object(&object_ref).class_name().to_string();
        let field_class = jenv.load_and_init_class(&object_class_name);
        let class_field = field_class
            .get_field(field_ref.field_name, field_ref.descriptor)
            .unwrap();
        let index = class_field.index();
        method.set_field(opcode_pc, index);
        index
    };
    let obj = jenv.heap.get_object_mut(&object_ref);
    obj.set_field(field_index, value);
}

pub fn getfield(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let opcode_pc = frame.pc() - 1;
    let index = frame.read_u16().unwrap();
    let object_ref = frame.operand_stack.pop();

    let method = frame.method();
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
    let value = obj.get_field(field_index).clone();
    debug!(?value, "getfield");
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push(value);
}

pub fn ifge(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_integer();
    if value >= 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifgt(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_integer();
    if value > 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn iflt(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_integer();
    if value < 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifle(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_integer();
    if value <= 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpeq(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 == value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpne(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 != value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_acmpne(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop();
    let value1 = frame.operand_stack.pop();
    if value1 != value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_acmpeq(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop();
    let value1 = frame.operand_stack.pop();
    if value1 == value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmplt(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 < value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmple(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 <= value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpgt(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 > value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpge(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_integer();
    let value1 = frame.operand_stack.pop_integer();
    if value1 >= value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifeq(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_integer();
    if value == 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifne(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_integer();
    if value != 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifnonnull(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop();
    if value != Operand::Null {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifnull(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop();
    if value == Operand::Null {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}
pub fn goto(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    frame.set_pc((pc as i32 - 1 + offset) as usize);
}

pub fn i2f(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    frame.operand_stack.push_float(value as f32);
}

pub fn f2i(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_float();
    frame.operand_stack.push_integer(value as i32);
}

pub fn i2l(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value = frame.operand_stack.pop_integer();
    frame.operand_stack.push_long(value as i64);
}

pub fn fmul(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let value2 = frame.operand_stack.pop_float();
    let value1 = frame.operand_stack.pop_float();
    frame.operand_stack.push_float(value1 * value2);
}

pub fn fcmpg(jenv: &mut JvmEnv, class: &Class) {
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

pub fn ldc2_w(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let offset = frame.read_u16().unwrap();
    let n = match class.constant_pool().get_const_pool_info_at(offset) {
        ConstPoolInfo::ConstantLongInfo(n) => Operand::Long(*n),
        ConstPoolInfo::ConstantDoubleInfo(n) => Operand::Double(*n),
        _ => unreachable!(),
    };
    frame.operand_stack.push(n);
}

pub fn sipush(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let n = frame.read_u16().unwrap();
    frame.operand_stack.push_integer(n as i32);
}

pub fn lshl(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 << (val2 & 0x0011_1111));
}

pub fn ishl(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame
        .operand_stack
        .push_integer(val1 << (val2 & 0x0011_1111));
}

pub fn iushr(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame
        .operand_stack
        .push_integer(val1 >> (val2 & 0x0001_1111));
}

pub fn ixor(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 | val2);
}

pub fn land(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_long();
    let val1 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(val1 & val2);
}

pub fn iand(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 & val2);
}

pub fn isub(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let val2 = frame.operand_stack.pop_integer();
    let val1 = frame.operand_stack.pop_integer();
    frame.operand_stack.push_integer(val1 - val2);
}

pub fn instanceof(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = jenv.load_and_init_class(class_name);
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let obj_ref = frame.operand_stack.pop();
    if obj_ref == Operand::Null {
        frame.operand_stack.push_integer(0);
        return;
    }
    let obj_class_name = jenv.heap.get_class_name(&obj_ref);
    let obj_class = jenv.load_and_init_class(&obj_class_name);
    let v = if can_cast_to(jenv, obj_class, class) {
        1
    } else {
        0
    };
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(v);
}

pub fn athrow(jenv: &mut JvmEnv, class: &Class) {
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let index = frame.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = jenv.load_and_init_class(class_name);
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    let obj_ref = frame.operand_stack.pop();
    if obj_ref == Operand::Null {
        frame.operand_stack.push_integer(0);
        return;
    }
    let obj_class_name = jenv.heap.get_class_name(&obj_ref);
    let obj_class = jenv.load_and_init_class(&obj_class_name);
    let v = if can_cast_to(jenv, obj_class, class) {
        1
    } else {
        0
    };
    let frame = jenv.thread.stack.frames.back_mut().unwrap();
    frame.operand_stack.push_integer(v);
}
