#![allow(unused_variables)]

pub mod opcode;

use crate::class::Class;
use crate::class_loader::{get_class_by_id, init_class, load_class};
use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::gc::global_definition::{JChar, JInt, JObject, JValue};

use crate::gc::mem::is_aligned;
use crate::java_const::JAVA_LANG_OBJECT;
use crate::jenv::{
    did_override_method, new_java_lang_string, new_jobject, new_jobject_array, new_jtype_array,
};
use crate::jthread::JvmThread;
use crate::jvm::execute_method;
use tracing::debug;

pub fn iconst_n(thread: &mut JvmThread, class: &Class, n: i32) {
    thread.push_jint(n);
}

pub fn lconst_n(thread: &mut JvmThread, class: &Class, n: i64) {
    thread.push_jlong(n);
}

pub fn fconst_n(thread: &mut JvmThread, class: &Class, n: f32) {
    thread.push_jfloat(n);
}

pub fn ldc(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u8();
    let const_pool_info = class.constant_pool().get_const_pool_info_at(index as u16);
    match const_pool_info {
        ConstPoolInfo::ConstantIntegerInfo(num) => {
            thread.push_jint(*num);
        }
        ConstPoolInfo::ConstantFloatInfo(num) => {
            thread.push_jfloat(*num);
        }
        ConstPoolInfo::ConstantStringInfo { string_index } => {
            let s = class.constant_pool().get_utf8_string_at(*string_index);
            let str_ref = new_java_lang_string(s);
            thread.push_jobject(str_ref)
        }
        ConstPoolInfo::ConstantClassInfo { name_index } => {
            let name = class.constant_pool().get_utf8_string_at(*name_index);
            let class = load_class(class.class_loader(), name);
            init_class(thread, &class);
            thread.push_jobject(class.mirror_class());
        }
        ConstPoolInfo::ConstantMethodHandleInfo { .. } => unimplemented!(),
        ConstPoolInfo::ConstantMethodTypeInfo { .. } => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn istore_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let val = thread.pop_jint();
    thread.set_local_variable(n as u16, JValue::Int(val));
}

pub fn istore(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u8();
    let val = thread.pop_jint();
    thread.set_local_variable(index as u16, JValue::Int(val));
}

pub fn astore_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let val = thread.pop();
    thread.set_local_variable(n as u16, val);
}

pub fn astore(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u8();
    let val = thread.pop();
    thread.set_local_variable(index as u16, val);
}

pub fn aastore(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jobject();
    let index = thread.pop_unsigned_jint();
    let array_ref = thread.pop_jarray();
    array_ref.set(index as usize, val);
}

pub fn iastore(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jint();
    let index = thread.pop_unsigned_jint();
    let array_ref = thread.pop_jarray();
    array_ref.set(index as usize, val);
}

pub fn iaload(thread: &mut JvmThread, class: &Class) {
    let index = thread.pop_unsigned_jint();
    let array_ref = thread.pop_jarray();
    let val: JInt = array_ref.get(index as usize);
    thread.push_jint(val)
}

pub fn bipush(thread: &mut JvmThread, class: &Class) {
    let byte = thread.read_u8();
    thread.push_jint(byte as i32);
}

pub fn iload_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let val = thread.get_local_variable_jint(n as u16);
    thread.push_jint(val);
}

pub fn lload_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let val = thread.get_local_variable_jlong(n as u16);
    thread.push_jlong(val);
}

pub fn lload(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u8();
    let val = thread.get_local_variable_jlong(index as u16);
    thread.push_jlong(val);
}

pub fn iload(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u8();
    let val = thread.get_local_variable_jint(index as u16);
    thread.push_jint(val);
}

pub fn iinc(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u8();
    // amount is signed
    let amount = thread.read_u8() as i8 as i32;
    let val = thread.get_local_variable_jint(index as u16);
    thread.set_local_variable_jint(index as u16, val + amount);
}

pub fn aload_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let val = thread.get_local_variable_jobject(n as u16);
    thread.push(val);
}

pub fn aload(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u8();
    let val = thread.get_local_variable_jobject(index as u16);
    thread.push(val);
}

pub fn aaload(thread: &mut JvmThread, class: &Class) {
    let index = thread.pop_unsigned_jint();
    let array_ref = thread.pop_jarray();
    thread.push_jobject(array_ref.get::<JObject>(index as usize));
}

pub fn caload(thread: &mut JvmThread, class: &Class) {
    let index = thread.pop_unsigned_jint();
    let array_ref = thread.pop_jarray();
    thread.push_jchar(array_ref.get(index as usize));
}

pub fn fload_n(thread: &mut JvmThread, class: &Class, n: i32) {
    let val = thread.get_local_variable_jfloat(n as u16);
    thread.push_jfloat(val);
}

pub fn irem(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 % val2);
}

pub fn iadd(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 + val2);
}

pub fn ineg(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jint();
    thread.push_jint(-val);
}
pub fn imul(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 * val2);
}

pub fn ladd(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jlong();
    let val1 = thread.pop_jlong();
    thread.push_jlong(val1 + val2);
}

pub fn invokestatic(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u16();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let class_name = method_ref.class_name;
    let class = load_class(class.class_loader(), class_name);
    init_class(thread, &class);
    let method = class
        .get_method(method_ref.method_name, method_ref.descriptor, true)
        .expect("get method");

    let n_args = method.n_args();
    let mut args = Vec::with_capacity(n_args);
    for _ in 0..n_args {
        args.push(thread.pop());
    }
    args.reverse();
    execute_method(thread, method, args);
}

pub fn ireturn(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jint();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push_jint(val);
}

pub fn dreturn(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jdouble();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push_jdouble(val);
}

pub fn freturn(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jfloat();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push_jfloat(val);
}

pub fn areturn(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jobject();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push_jobject(val);
}

pub fn lreturn(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jlong();
    thread.pop_frame();
    let last_frame = thread.current_frame_mut();
    last_frame.operand_stack.push_jlong(val);
}

pub fn return_(thread: &mut JvmThread, class: &Class) {
    thread.pop_frame();
}

pub fn getstatic(thread: &mut JvmThread, class: &Class) {
    let opcode_pc = thread.pc() - 1;
    let index = thread.read_u16();
    let method = thread.current_method().unwrap();
    let (field_class, basic_type, field_offset) =
        if let Some(v) = method.resolve_static_field(opcode_pc) {
            v
        } else {
            let field_ref = class.constant_pool().get_field_ref_at(index);
            let field_class = load_class(class.class_loader(), field_ref.class_name);
            init_class(thread, &field_class);
            let field = field_class
                .get_static_field(field_ref.field_name, field_ref.descriptor)
                .unwrap_or_else(|| panic!("resolve field: {:?}", field_ref));
            let field_offset = field.offset();
            let basic_type = field.basic_type();
            method.set_static_field(opcode_pc, field_class.clone(), basic_type, field_offset);

            (field_class, basic_type, field_offset)
        };

    let mirror_class = field_class.mirror_class();
    thread.push(mirror_class.get_field_by_basic_type_and_offset(basic_type, field_offset))
}

pub fn putstatic(thread: &mut JvmThread, class: &Class) {
    let opcode_pc = thread.pc() - 1;
    let index = thread.read_u16();
    let value = thread.pop();
    let method = thread.current_method().unwrap();
    let (field_class, basic_type, field_offset) =
        if let Some(v) = method.resolve_static_field(opcode_pc) {
            v
        } else {
            let field_ref = class.constant_pool().get_field_ref_at(index);
            let field_class = load_class(class.class_loader(), field_ref.class_name);
            init_class(thread, &field_class);
            let field = field_class
                .get_static_field(field_ref.field_name, field_ref.descriptor)
                .unwrap_or_else(|| panic!("resolve field: {:?}", field_ref));
            let field_offset = field.offset();
            debug!(?field_ref, %field_offset, ?class, field=?value, "putstatic");
            let basic_type = field.basic_type();
            method.set_static_field(opcode_pc, field_class.clone(), basic_type, field_offset);
            (field_class, basic_type, field_offset)
        };

    let mirror_class = field_class.mirror_class();
    mirror_class.set_field_by_jvalue_and_offset(field_offset, value);
}

pub fn aconst_null(thread: &mut JvmThread, class: &Class) {
    thread.push(JValue::Object(JObject::null()))
}

pub fn invokevirtual(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u16();
    let method_ref = class.constant_pool().get_method_ref_at(index);
    debug!(?method_ref, "invokevirtual");
    let resolved_class = load_class(class.class_loader(), method_ref.class_name);
    init_class(thread, &resolved_class);
    let resolved_method = resolved_class
        .get_method(method_ref.method_name, method_ref.descriptor, false)
        .unwrap_or_else(|| panic!("get method: {}", &method_ref.method_name));
    assert!(
        resolved_method.name() != "<init>" && resolved_method.name() != "<clinit>",
        "<init> and <clinit> are not allowed here"
    );
    let n_args = resolved_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(thread.pop());
    }
    let object_ref = thread.pop_jobject();
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

    execute_method(thread, acutal_method, args);
}

pub fn invokeinterface(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u16();
    let count = thread.read_u8();
    assert_ne!(count, 0);
    let forth = thread.read_u8();
    assert_eq!(forth, 0);
    let method_ref = class.constant_pool().get_interface_method_ref_at(index);
    debug!(?method_ref, "invokeinterface");
    let resolved_class = load_class(class.class_loader(), method_ref.class_name);
    init_class(thread, &resolved_class);
    let resolved_method = resolved_class
        .get_interface_method(method_ref.method_name, method_ref.descriptor)
        .unwrap_or_else(|| panic!("get interface method: {}", &method_ref.method_name));
    assert!(
        resolved_method.name() != "<init>" && resolved_method.name() != "<clinit>",
        "<init> and <clinit> are not allowed here"
    );
    let n_args = resolved_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(thread.pop());
    }
    let object_ref = thread.pop();
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

    execute_method(thread, acutal_method, args);
}

pub fn new(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u16();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), class_name);
    init_class(thread, &class);
    let jobject = new_jobject(&class);
    thread.push_jobject(jobject)
}

pub fn newarray(thread: &mut JvmThread, class: &Class) {
    let count = thread.pop_jint();
    let atype = thread.read_u8();
    let array = new_jtype_array(atype.into(), count as usize);
    thread.push_jarray(array);
}

pub fn anewarray(thread: &mut JvmThread, class: &Class) {
    let count = thread.pop_jint();
    let index = thread.read_u16();
    let resolved_class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), resolved_class_name);
    init_class(thread, &class);
    let array_ref = new_jobject_array(class, count as usize);
    thread.push_jarray(array_ref);
}

pub fn arraylength(thread: &mut JvmThread, class: &Class) {
    let array_ref = thread.pop_jarray();
    let len = array_ref.len();
    thread.push_jint(len as i32);
}

pub fn pop(thread: &mut JvmThread, class: &Class) {
    let _ = thread.pop();
}

pub fn can_cast_to(thread: &mut JvmThread, s: Class, t: Class) -> bool {
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
        (Class::TypeArrayClass(s), Class::TypeArrayClass(t)) => s.ty() == t.ty(),
        (Class::ObjArrayClass(s), Class::ObjArrayClass(t)) => {
            let sc = s.element_class();
            let tc = t.element_class();
            can_cast_to(thread, sc, tc)
        }
        _ => false,
    }
}

pub fn checkcast(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u16();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), class_name);
    init_class(thread, &class);
    let obj_ref = thread.pop();
    if obj_ref.is_null() {
        thread.push(obj_ref);
        return;
    }
    let class_id = obj_ref.class_id();
    let obj_class = get_class_by_id(class_id);

    assert!(can_cast_to(thread, obj_class, class));

    thread.push(obj_ref);
}

pub fn dup(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop();
    thread.push(val.clone());
    thread.push(val);
}

pub fn dup2(thread: &mut JvmThread, class: &Class) {
    let val1 = thread.pop();
    if val1.is_category1() {
        let val2 = thread.pop();
        assert!(val2.is_category1());
        thread.push(val2);
        thread.push(val1);
        thread.push(val2);
        thread.push(val1);
    } else {
        thread.push(val1);
        thread.push(val1);
    }
}

pub fn dup_x1(thread: &mut JvmThread, class: &Class) {
    let val1 = thread.pop();
    let val2 = thread.pop();
    thread.push(val1.clone());
    thread.push(val2);
    thread.push(val1);
}

pub fn castore(thread: &mut JvmThread, class: &Class) {
    let val = thread.pop_jint();
    let index = thread.pop_unsigned_jint();
    let array_ref = thread.pop_jarray();
    array_ref.set(index as usize, val as u16)
}

pub fn invokespecial(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u16();
    let method_ref = class
        .constant_pool()
        .get_class_method_or_interface_method_at(index);

    let resolved_class = load_class(class.class_loader(), method_ref.class_name);
    init_class(thread, &resolved_class);
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

    let n_args = actual_method.n_args();
    let mut args = Vec::with_capacity(n_args + 1);
    for i in 0..n_args {
        args.push(thread.pop());
    }
    let object_ref = thread.pop();
    args.push(object_ref);
    args.reverse();

    execute_method(thread, actual_method, args);
}

pub fn putfield(thread: &mut JvmThread, class: &Class) {
    let opcode_pc = thread.pc() - 1;
    let index = thread.read_u16();
    let value = thread.pop();
    let object_ref = thread.pop_jobject();

    let method = thread.current_method().unwrap();
    let (ty, field_offset) = if let Some(offset) = method.resolve_field(opcode_pc) {
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
        let ty = class_field.basic_type();
        method.set_field(opcode_pc, ty, offset);
        (ty, offset)
    };
    object_ref.set_field_by_jvalue_and_offset(field_offset, value)
}

pub fn getfield(thread: &mut JvmThread, class: &Class) {
    let opcode_pc = thread.pc() - 1;
    let index = thread.read_u16();
    let object_ref = thread.pop_jobject();

    let method = thread.current_method().unwrap();
    let (ty, field_offset) = if let Some(offset) = method.resolve_field(opcode_pc) {
        offset
    } else {
        let field_ref = class.constant_pool().get_field_ref_at(index);
        let object_class_id = object_ref.class_id();
        let obj_class = get_class_by_id(object_class_id);
        let class_field = obj_class
            .get_field(field_ref.field_name, field_ref.descriptor)
            .unwrap();
        let offset = class_field.offset();
        let ty = class_field.basic_type();
        debug!(?object_ref, ?field_ref, offset, "getfield");
        method.set_field(opcode_pc, ty, offset);
        (ty, offset)
    };
    let value = object_ref.get_field_by_basic_type_and_offset(ty, field_offset);
    debug!(?value, "getfield");
    thread.push(value);
}

pub fn ifge(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value = thread.pop_jint();
    if value >= 0 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifgt(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value = thread.pop_jint();
    if value > 0 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn iflt(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value = thread.pop_jint();
    if value < 0 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifle(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value = thread.pop_jint();
    if value <= 0 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpeq(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value2 = thread.pop_jint();
    let value1 = thread.pop_jint();
    if value1 == value2 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpne(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value2 = thread.pop_jint();
    let value1 = thread.pop_jint();
    if value1 != value2 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_acmpne(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value2 = thread.pop();
    let value1 = thread.pop();
    if value1 != value2 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_acmpeq(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value2 = thread.pop();
    let value1 = thread.pop();
    if value1 == value2 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmplt(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value2 = thread.pop_jint();
    let value1 = thread.pop_jint();
    if value1 < value2 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmple(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value2 = thread.pop_jint();
    let value1 = thread.pop_jint();
    if value1 <= value2 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpgt(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value2 = thread.pop_jint();
    let value1 = thread.pop_jint();
    if value1 > value2 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn if_icmpge(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value2 = thread.pop_jint();
    let value1 = thread.pop_jint();
    if value1 >= value2 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifeq(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value = thread.pop_jint();
    if value == 0 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifne(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value = thread.pop_jint();
    if value != 0 {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifnonnull(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value = thread.pop();
    if !value.is_null() {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}

pub fn ifnull(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    let value = thread.pop();
    if value.is_null() {
        thread.set_pc((pc as i32 - 1 + offset) as usize);
    }
}
pub fn goto(thread: &mut JvmThread, class: &Class) {
    let pc = thread.pc();
    let offset = thread.read_i16() as i32;
    thread.set_pc((pc as i32 - 1 + offset) as usize);
}

pub fn i2f(thread: &mut JvmThread, class: &Class) {
    let value = thread.pop_jint();
    thread.push_jfloat(value as f32);
}

pub fn i2c(thread: &mut JvmThread, class: &Class) {
    let value = thread.pop_jint();
    thread.push_jchar(value as JChar);
}
pub fn f2i(thread: &mut JvmThread, class: &Class) {
    let value = thread.pop_jfloat();
    thread.push_jint(value as i32);
}

pub fn i2l(thread: &mut JvmThread, class: &Class) {
    let value = thread.pop_jint();
    thread.push_jlong(value as i64);
}

pub fn fmul(thread: &mut JvmThread, class: &Class) {
    let value2 = thread.pop_jfloat();
    let value1 = thread.pop_jfloat();
    thread.push_jfloat(value1 * value2);
}

pub fn fcmpg(thread: &mut JvmThread, class: &Class) {
    let value2 = thread.pop_jfloat();
    let value1 = thread.pop_jfloat();
    if value1 > value2 {
        thread.push_jint(1)
    } else if value1 < value2 {
        thread.push_jint(-1)
    } else {
        thread.push_jint(0)
    }
}

pub fn ldc2_w(thread: &mut JvmThread, class: &Class) {
    let offset = thread.read_u16();
    let n = match class.constant_pool().get_const_pool_info_at(offset) {
        ConstPoolInfo::ConstantLongInfo(n) => JValue::Long(*n),
        ConstPoolInfo::ConstantDoubleInfo(n) => JValue::Double(*n),
        _ => unreachable!(),
    };
    thread.push(n);
}

pub fn sipush(thread: &mut JvmThread, class: &Class) {
    let n = thread.read_u16();
    thread.push_jint(n as i32);
}

pub fn lshl(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jlong();
    thread.push_jlong(val1 << (val2 & 0x0011_1111));
}

pub fn ishl(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 << (val2 & 0x1f));
}

pub fn ishr(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 >> (val2 & 0x1f));
}

pub fn iushr(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    let ret = if val1 >= 0 {
        val1 >> (val2 & 0x1f)
    } else {
        let s = val2 & 0x1f;
        (val1 >> s) + (2 << !s)
    };
    thread.push_jint(val1 >> (val2 & 0x1f));
}

pub fn ixor(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 | val2);
}

pub fn land(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jlong();
    let val1 = thread.pop_jlong();
    thread.push_jlong(val1 & val2);
}

pub fn iand(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 & val2);
}

pub fn ior(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 | val2);
}

pub fn isub(thread: &mut JvmThread, class: &Class) {
    let val2 = thread.pop_jint();
    let val1 = thread.pop_jint();
    thread.push_jint(val1 - val2);
}

pub fn instanceof(thread: &mut JvmThread, class: &Class) {
    let index = thread.read_u16();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), class_name);
    init_class(thread, &class);
    let obj_ref = thread.pop_jobject();
    if obj_ref.is_null() {
        thread.push_jint(0);
        return;
    }
    let obj_class_id = obj_ref.class_id();
    let obj_class = get_class_by_id(obj_class_id);
    let v = if can_cast_to(thread, obj_class, class) {
        1
    } else {
        0
    };
    thread.push_jint(v);
}

pub fn athrow(thread: &mut JvmThread, class: &Class) {
    panic!("throw");
    // todo:
    let index = thread.read_u16();
    let class_name = class.constant_pool().get_class_name_at(index);
    let class = load_class(class.class_loader(), class_name);
    init_class(thread, &class);
    let obj_ref = thread.pop_jobject();
    if obj_ref.is_null() {
        thread.push_jint(0);
        return;
    }
    let obj_class_id = obj_ref.class_id();
    let obj_class = get_class_by_id(obj_class_id);
    let v = if can_cast_to(thread, obj_class, class) {
        1
    } else {
        0
    };
    thread.push_jint(v);
}

pub fn tableswitch(thread: &mut JvmThread, class: &Class) {
    let tableswitch_opcode_addr = thread.pc() - 1;
    let mut skip_bytes = 0;
    loop {
        let pc = thread.pc();
        // defaultbyte must be 4 bytes aligned
        if is_aligned(pc, 4) {
            break;
        }
        // skip one byte
        let _ = thread.read_u8();
        skip_bytes += 1;
    }
    // padding is at most 3 bytes
    assert!(skip_bytes <= 3);
    let default = thread.read_i32();
    let low = thread.read_i32();
    let high = thread.read_i32();
    assert!(low <= high);
    let count_offsets = (high - low + 1) as usize;
    let mut offsets = Vec::with_capacity(count_offsets);
    for i in 0..count_offsets {
        offsets.push(thread.read_i32());
    }
    let index = thread.pop_jint();
    let target_addr = if index < low || index > high {
        default as usize + tableswitch_opcode_addr
    } else {
        offsets[(index - low) as usize] as usize + tableswitch_opcode_addr
    };
    thread.set_pc(target_addr);
}

pub fn lookupswitch(thread: &mut JvmThread, class: &Class) {
    let opcode_addr = thread.pc() as i32 - 1;
    let mut skip_bytes = 0;
    loop {
        let pc = thread.pc();
        // defaultbyte must be 4 bytes aligned
        if is_aligned(pc, 4) {
            break;
        }
        // skip one byte
        let _ = thread.read_u8();
        skip_bytes += 1;
    }
    // padding is at most 3 bytes
    assert!(skip_bytes <= 3);
    let default = thread.read_i32();
    let n_pairs = thread.read_i32() as usize;
    let mut pairs = Vec::with_capacity(n_pairs);
    for i in 0..n_pairs {
        pairs.push((thread.read_i32(), thread.read_i32()));
    }
    pairs.sort_by_key(|(match_, offset)| *match_);
    let key = thread.read_i32();
    let found = pairs.binary_search_by_key(&key, |(match_, _)| *match_);
    let target_addr = match found {
        Ok(i) => pairs[i].1 + opcode_addr,
        Err(_) => default + opcode_addr,
    };
    thread.set_pc(target_addr as usize);

    println!(
        "opcode_addr: {}, default: {}, n_pairs: {}, pairs: {:?}, key: {}, target_addr: {}",
        opcode_addr, default, n_pairs, pairs, key, target_addr
    );
}
