#![allow(unused_variables)]

pub mod opcode;

use crate::class::Class;
use crate::class_loader::{get_class_by_id, load_class};
use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::gc::global_definition::{JObject, JValue};
use crate::gc::oop::ArrayOop;
use crate::java_const::JAVA_LANG_OBJECT;
use crate::jenv::{
    did_override_method, new_java_lang_string, new_jobject, new_jobject_array, new_jtype_array,
};
use crate::jthread::JvmThread;
use crate::jvm::execute_method;
use tracing::debug;

pub fn iconst_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jint(n);
}

pub fn lconst_n(thread: &mut JvmThread, class: &Class, n: i64) {
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jlong(n);
}

pub fn fconst_n(thread: &mut JvmThread, class: &Class, n: f32) {
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jfloat(n);
}

pub fn ldc(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u8().unwrap();
    let const_pool_info = class.constant_pool().get_const_pool_info_at(index as u16);
    match const_pool_info {
        ConstPoolInfo::ConstantIntegerInfo(num) => {
            let frame = thread.current_frame_mut();
            frame.operand_stack.push_jint(*num);
        }
        ConstPoolInfo::ConstantFloatInfo(num) => {
            let frame = thread.current_frame_mut();
            frame.operand_stack.push_jfloat(*num);
        }
        ConstPoolInfo::ConstantStringInfo { string_index } => {
            let s = class.constant_pool().get_utf8_string_at(*string_index);
            let str_ref = new_java_lang_string(s);
            let frame = thread.current_frame_mut();
            frame.operand_stack.push_jobject(str_ref)
        }
        ConstPoolInfo::ConstantClassInfo { name_index } => {
            let name = class.constant_pool().get_utf8_string_at(*name_index);
            let class = load_class(class.class_loader(), name);
            let frame = thread.current_frame_mut();
            frame.operand_stack.push_jobject(class.mirror_class());
        }
        ConstPoolInfo::ConstantMethodHandleInfo { .. } => unimplemented!(),
        ConstPoolInfo::ConstantMethodTypeInfo { .. } => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn istore_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop_jint();
    frame.local_variable_array.set(n as u16, JValue::Int(val));
}

pub fn istore(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u8().unwrap();
    let val = frame.operand_stack.pop_jint();
    frame
        .local_variable_array
        .set(index as u16, JValue::Int(val));
}

pub fn astore_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop();
    frame.local_variable_array.set(n as u16, val);
}

pub fn astore(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u8().unwrap();
    let val = frame.operand_stack.pop();
    frame.local_variable_array.set(index as u16, val);
}

pub fn aastore(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop();
    let index = frame.operand_stack.pop_jint();
    let array_ref = frame.operand_stack.pop_jarray();
    array_ref.set(index as usize, val);
}

pub fn bipush(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let byte = frame.read_u8().unwrap();
    frame.operand_stack.push_jint(byte as i32);
}

pub fn iload_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let frame = thread.current_frame_mut();
    let val = frame.local_variable_array.get_integer(n as u16);
    frame.operand_stack.push_jint(val);
}

pub fn lload_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let frame = thread.current_frame_mut();
    let val = frame.local_variable_array.get_long(n as u16);
    frame.operand_stack.push_jlong(val);
}

pub fn lload(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u8().unwrap();
    let val = frame.local_variable_array.get_long(index as u16);
    frame.operand_stack.push_jlong(val);
}

pub fn iload(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u8().unwrap();
    let val = frame.local_variable_array.get_integer(index as u16);
    frame.operand_stack.push_jint(val);
}

pub fn iinc(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u8().unwrap();
    // amount is signed
    let amount = frame.read_u8().unwrap() as i8 as i32;
    let val = frame.local_variable_array.get_integer(index as u16);
    frame
        .local_variable_array
        .set_integer(index as u16, val + amount);
}

pub fn aload_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let frame = thread.current_frame_mut();
    let val = frame.local_variable_array.get_jobject(n as u16);
    frame.operand_stack.push(val);
}

pub fn aload(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u8().unwrap();
    let val = frame.local_variable_array.get_jobject(index as u16);
    frame.operand_stack.push(val);
}

pub fn aaload(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.operand_stack.pop_jint();
    let array_ref = frame.operand_stack.pop_jarray();
    frame.operand_stack.push(array_ref.get(index as usize));
}

pub fn caload(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.operand_stack.pop_jint();
    let array_ref = frame.operand_stack.pop_jarray();
    frame.operand_stack.push_jint(array_ref.get(index as usize));
}

pub fn fload_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let frame = thread.current_frame_mut();
    let val = frame.local_variable_array.get_float(n as u16);
    frame.operand_stack.push_jfloat(val);
}

pub fn irem(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jint();
    let val1 = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jint(val1 % val2);
}

pub fn iadd(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jint();
    let val1 = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jint(val1 + val2);
}

pub fn ladd(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jlong();
    let val1 = frame.operand_stack.pop_jlong();
    frame.operand_stack.push_jlong(val1 + val2);
}

pub fn invokestatic(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u16().unwrap();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let class_name = method_ref.class_name;
    let class = load_class(class.class_loader(), class_name);
    let method = class
        .get_method(method_ref.method_name, method_ref.descriptor, true)
        .expect("get method");

    let frame = thread.current_frame_mut();
    let n_args = method.n_args();
    let mut args = Vec::with_capacity(n_args);
    for _ in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    args.reverse();
    execute_method(thread, method, args);
}

pub fn ireturn(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop_jint();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push_jint(val);
}

pub fn dreturn(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop_jdouble();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push_jdouble(val);
}

pub fn freturn(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop_jfloat();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push_jfloat(val);
}

pub fn areturn(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push(val);
}

pub fn return_(thread: &mut JvmThread, class: &Class) {
    thread.pop_frame();
}

pub fn getstatic(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let opcode_pc = frame.pc() - 1;
    let index = frame.read_u16().unwrap();
    let method = frame.method();
    let (field_class, field_offset) = if let Some(v) = method.resolve_static_field(opcode_pc) {
        v
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let field_class = load_class(class.class_loader(), field_ref.class_name);
        let field = field_class
            .get_static_field(field_ref.field_name, field_ref.descriptor)
            .unwrap_or_else(|| panic!("resolve field: {:?}", field_ref));
        let field_offset = field.offset();
        method.set_static_field(opcode_pc, field_class.clone(), field_offset);

        (field_class, field_offset)
    };

    let mirror_class = field_class.mirror_class();
    let frame = thread.current_frame_mut();
    frame
        .operand_stack
        .push(mirror_class.get_field_by_offset(field_offset))
}

pub fn putstatic(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let opcode_pc = frame.pc() - 1;
    let index = frame.read_u16().unwrap();
    let value = frame.operand_stack.pop();
    let method = frame.method();
    let (field_class, field_offset) = if let Some(v) = method.resolve_static_field(opcode_pc) {
        v
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let field_class = load_class(class.class_loader(), field_ref.class_name);
        let field = field_class
            .get_static_field(field_ref.field_name, field_ref.descriptor)
            .unwrap_or_else(|| panic!("resolve field: {:?}", field_ref));
        let field_offset = field.offset();
        debug!(?field_ref, %field_offset, ?class, field=?value, "putstatic");
        method.set_static_field(opcode_pc, field_class.clone(), field_offset);
        (field_class, field_offset)
    };

    let mirror_class = field_class.mirror_class();
    mirror_class.set_field_by_offset(field_offset, value);
}

pub fn aconst_null(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    frame.operand_stack.push(JValue::Object(JObject::null()))
}

pub fn invokevirtual(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u16().unwrap();
    let method_ref = class.constant_pool().get_method_ref_at(index);
    debug!(?method_ref, "invokevirtual");
    let resolved_class = load_class(class.class_loader(), method_ref.class_name);
    let resolved_method = resolved_class
        .get_method(method_ref.method_name, method_ref.descriptor, false)
        .unwrap_or_else(|| panic!("get method: {}", &method_ref.method_name));
    assert!(
        resolved_method.name() != "<init>" && resolved_method.name() != "<clinit>",
        "<init> and <clinit> are not allowed here"
    );
    let frame = thread.current_frame_mut();
    let n_args = resolved_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    let object_ref = frame.operand_stack.pop_jobject();
    args.push(JValue::Object(object_ref));
    args.reverse();

    if resolved_method.is_native() {
        execute_method(thread, resolved_method, args);
        return;
    }

    let class_id = object_ref.class_id();
    let object_class = get_class_by_id(class_id);

    let acutal_method = if !resolved_method.is_signature_polymorphic() {
        if let Some(actual_method) = object_class
            .get_self_method(resolved_method.name(), resolved_method.descriptor(), false)
            .filter(|m| did_override_method(m, &resolved_method))
        {
            actual_method
        } else if let Some(actual_method) = object_class
            .iter_super_classes()
            .filter_map(|klass| {
                klass.get_self_method(resolved_method.name(), resolved_method.descriptor(), false)
            })
            .find(|m| did_override_method(m, &resolved_method))
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

    let actual_class = load_class(class.class_loader(), acutal_method.class_name());

    execute_method(thread, acutal_method, args);
}

pub fn invokeinterface(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u16().unwrap();
    let count = frame.read_u8().unwrap();
    assert_ne!(count, 0);
    let forth = frame.read_u8().unwrap();
    assert_eq!(forth, 0);
    let method_ref = class.constant_pool().get_interface_method_ref_at(index);
    debug!(?method_ref, "invokeinterface");
    let resolved_class = load_class(class.class_loader(), method_ref.class_name);
    let resolved_method = resolved_class
        .get_interface_method(method_ref.method_name, method_ref.descriptor)
        .unwrap_or_else(|| panic!("get interface method: {}", &method_ref.method_name));
    assert!(
        resolved_method.name() != "<init>" && resolved_method.name() != "<clinit>",
        "<init> and <clinit> are not allowed here"
    );
    let frame = thread.current_frame_mut();
    let n_args = resolved_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    let object_ref = frame.operand_stack.pop();
    args.push(object_ref.clone());
    args.reverse();

    if resolved_method.is_native() {
        execute_method(thread, resolved_method, args);
        return;
    }

    let class_id = object_ref.as_jobject().class_id();
    let object_class = get_class_by_id(class_id);

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

    let actual_class = load_class(class.class_loader(), acutal_method.class_name());

    execute_method(thread, acutal_method, args);
}

pub fn new(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), class_name);
    let jobject = new_jobject(class);
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jobject(jobject)
}

pub fn newarray(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let count = frame.operand_stack.pop_jint();
    let atype = frame.read_u8().unwrap();
    let array = new_jtype_array(atype.into(), count as usize);
    frame.operand_stack.push_jarray(array)
}

pub fn anewarray(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let count = frame.operand_stack.pop_jint();
    let index = frame.read_u16().unwrap();
    let resolved_class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), resolved_class_name);
    let array_ref = new_jobject_array(class, count as usize);
    frame.operand_stack.push_jarray(array_ref)
}

pub fn arraylength(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let array_ref = frame.operand_stack.pop_jarray();
    let len = array_ref.len();
    frame.operand_stack.push_jint(len as i32);
}

pub fn pop(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let _ = frame.operand_stack.pop();
}

fn can_cast_to(thread: &mut JvmThread, s: Class, t: Class) -> bool {
    match (s, t) {
        (Class::InstanceClass(s), Class::InstanceClass(t))
            if (s.is_class() && t.is_class()) || (s.is_interface() && t.is_interface()) =>
        {
            s == t || s.is_subclass_of(Class::InstanceClass(t))
        }
        (Class::InstanceClass(s), Class::InstanceClass(t)) if s.is_class() && t.is_interface() => {
            s.did_implement_interface(Class::InstanceClass(t))
        }
        (Class::InstanceClass(s), Class::InstanceClass(t)) if s.is_interface() && t.is_class() => {
            t.name() == JAVA_LANG_OBJECT
        }
        (s, Class::InstanceClass(t)) if t.is_class() => t.name() == JAVA_LANG_OBJECT,
        (s, Class::InstanceClass(t)) if t.is_interface() => unimplemented!(),
        (Class::TypeArrayClass(s), Class::TypeArrayClass(t)) => s.ty == t.ty,
        (Class::ObjArrayClass(s), Class::ObjArrayClass(t)) => {
            let sc = s.element_class.clone();
            let tc = t.element_class.clone();
            can_cast_to(thread, sc, tc)
        }
        _ => false,
    }
}

pub fn checkcast(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), class_name);
    let frame = thread.current_frame_mut();
    let obj_ref = frame.operand_stack.pop();
    if obj_ref.is_null() {
        frame.operand_stack.push(obj_ref);
        return;
    }
    let class_id = obj_ref.class_id();
    let obj_class = get_class_by_id(class_id);

    assert!(can_cast_to(thread, obj_class, class));

    let frame = thread.current_frame_mut();
    frame.operand_stack.push(obj_ref);
}

pub fn dup(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop();
    frame.operand_stack.push(val.clone());
    frame.operand_stack.push(val);
}

pub fn dup_x1(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val1 = frame.operand_stack.pop();
    let val2 = frame.operand_stack.pop();
    frame.operand_stack.push(val1.clone());
    frame.operand_stack.push(val2);
    frame.operand_stack.push(val1);
}

pub fn castore(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val = frame.operand_stack.pop_jint();
    let index = frame.operand_stack.pop_jint();
    let array_ref = frame.operand_stack.pop_jarray();
    array_ref.set(index as usize, val as u16)
}

pub fn invokespecial(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u16().unwrap();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let resolved_class = load_class(class.class_loader(), method_ref.class_name);

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

    let frame = thread.current_frame_mut();
    let n_args = actual_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(frame.operand_stack.pop());
    }
    let object_ref = frame.operand_stack.pop();
    args.push(object_ref);
    args.reverse();

    execute_method(thread, actual_method, args);
}

pub fn putfield(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let opcode_pc = frame.pc() - 1;
    let index = frame.read_u16().unwrap();
    let value = frame.operand_stack.pop();
    let object_ref = frame.operand_stack.pop_jobject();

    let method = frame.method();
    let field_offset = if let Some(offset) = method.resolve_field(opcode_pc) {
        offset
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        debug!(?object_ref, ?field_ref, "putfield");
        let object_class_id = object_ref.class_id();
        let field_class = get_class_by_id(object_class_id);
        let class_field = field_class
            .get_field(field_ref.field_name, field_ref.descriptor)
            .unwrap();
        let offset = class_field.offset();
        method.set_field(opcode_pc, offset);
        offset
    };
    object_ref.set_field_by_offset(field_offset, value)
}

pub fn getfield(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let opcode_pc = frame.pc() - 1;
    let index = frame.read_u16().unwrap();
    let object_ref = frame.operand_stack.pop_jobject();

    let method = frame.method();
    let field_offset = if let Some(offset) = method.resolve_field(opcode_pc) {
        offset
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let object_class_id = object_ref.class_id();
        let obj_class = get_class_by_id(object_class_id);
        let class_field = obj_class
            .get_field(field_ref.field_name, field_ref.descriptor)
            .unwrap();
        let offset = class_field.offset();
        debug!(?object_ref, ?field_ref, offset, "getfield");
        method.set_field(opcode_pc, offset);
        offset
    };
    let value = object_ref.get_field_by_offset(field_offset);
    debug!(?value, "getfield");
    let frame = thread.current_frame_mut();
    frame.operand_stack.push(value);
}

pub fn ifge(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_jint();
    if value >= 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifgt(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_jint();
    if value > 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn iflt(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_jint();
    if value < 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifle(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_jint();
    if value <= 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpeq(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_jint();
    let value1 = frame.operand_stack.pop_jint();
    if value1 == value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpne(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_jint();
    let value1 = frame.operand_stack.pop_jint();
    if value1 != value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_acmpne(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop();
    let value1 = frame.operand_stack.pop();
    if value1 != value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_acmpeq(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop();
    let value1 = frame.operand_stack.pop();
    if value1 == value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmplt(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_jint();
    let value1 = frame.operand_stack.pop_jint();
    if value1 < value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmple(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_jint();
    let value1 = frame.operand_stack.pop_jint();
    if value1 <= value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpgt(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_jint();
    let value1 = frame.operand_stack.pop_jint();
    if value1 > value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpge(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value2 = frame.operand_stack.pop_jint();
    let value1 = frame.operand_stack.pop_jint();
    if value1 >= value2 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifeq(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_jint();
    if value == 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifne(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop_jint();
    if value != 0 {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifnonnull(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop();
    if !value.is_null() {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifnull(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    let value = frame.operand_stack.pop();
    if value.is_null() {
        frame.set_pc((pc as i32 - 1 + offset) as usize);
    }
}
pub fn goto(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let pc = frame.pc();
    let offset = frame.read_i16().unwrap() as i32;
    frame.set_pc((pc as i32 - 1 + offset) as usize);
}

pub fn i2f(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let value = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jfloat(value as f32);
}

pub fn f2i(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let value = frame.operand_stack.pop_jfloat();
    frame.operand_stack.push_jint(value as i32);
}

pub fn i2l(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let value = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jlong(value as i64);
}

pub fn fmul(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let value2 = frame.operand_stack.pop_jfloat();
    let value1 = frame.operand_stack.pop_jfloat();
    frame.operand_stack.push_jfloat(value1 * value2);
}

pub fn fcmpg(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let value2 = frame.operand_stack.pop_jfloat();
    let value1 = frame.operand_stack.pop_jfloat();
    if value1 > value2 {
        frame.operand_stack.push_jint(1)
    } else if value1 < value2 {
        frame.operand_stack.push_jint(-1)
    } else {
        frame.operand_stack.push_jint(0)
    }
}

pub fn ldc2_w(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let offset = frame.read_u16().unwrap();
    let n = match class.constant_pool().get_const_pool_info_at(offset) {
        ConstPoolInfo::ConstantLongInfo(n) => JValue::Long(*n),
        ConstPoolInfo::ConstantDoubleInfo(n) => JValue::Double(*n),
        _ => unreachable!(),
    };
    frame.operand_stack.push(n);
}

pub fn sipush(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let n = frame.read_u16().unwrap();
    frame.operand_stack.push_jint(n as i32);
}

pub fn lshl(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jint();
    let val1 = frame.operand_stack.pop_jlong();
    frame.operand_stack.push_jlong(val1 << (val2 & 0x0011_1111));
}

pub fn ishl(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jint();
    let val1 = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jint(val1 << (val2 & 0x0011_1111));
}

pub fn iushr(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jint();
    let val1 = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jint(val1 >> (val2 & 0x0001_1111));
}

pub fn ixor(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jint();
    let val1 = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jint(val1 | val2);
}

pub fn land(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jlong();
    let val1 = frame.operand_stack.pop_jlong();
    frame.operand_stack.push_jlong(val1 & val2);
}

pub fn iand(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jint();
    let val1 = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jint(val1 & val2);
}

pub fn isub(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let val2 = frame.operand_stack.pop_jint();
    let val1 = frame.operand_stack.pop_jint();
    frame.operand_stack.push_jint(val1 - val2);
}

pub fn instanceof(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), class_name);
    let frame = thread.current_frame_mut();
    let obj_ref = frame.operand_stack.pop_jobject();
    if obj_ref.is_null() {
        frame.operand_stack.push_jint(0);
        return;
    }
    let obj_class_id = obj_ref.class_id();
    let obj_class = get_class_by_id(obj_class_id);
    let v = if can_cast_to(thread, obj_class, class) {
        1
    } else {
        0
    };
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jint(v);
}

pub fn athrow(thread: &mut JvmThread, class: &Class) {
    let frame = thread.current_frame_mut();
    let index = frame.read_u16().unwrap();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), class_name);
    let frame = thread.current_frame_mut();
    let obj_ref = frame.operand_stack.pop_jobject();
    if obj_ref.is_null() {
        frame.operand_stack.push_jint(0);
        return;
    }
    let obj_class_id = obj_ref.class_id();
    let obj_class = get_class_by_id(obj_class_id);
    let v = if can_cast_to(thread, obj_class, class) {
        1
    } else {
        0
    };
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jint(v);
}
