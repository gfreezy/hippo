#![allow(non_snake_case, unused_variables)]

use crate::class::{copy_array, Class, TypeArrayClass};
use crate::class_loader::{
    get_class_by_id, get_class_id_by_name, init_class, load_class, load_mirror_class,
};
use crate::class_parser::JVM_RECOGNIZED_FIELD_MODIFIERS;
use crate::debug::pretty_print;
use crate::gc::global_definition::{JArray, JInt, JLong, JObject, JValue};
use crate::gc::oop::Oop;
use crate::gc::oop_desc::{ArrayOopDesc, InstanceOopDesc, TypeArrayOopDesc};
use crate::java_const::{
    class_name_to_descriptor, JAVA_LANG_CLASS, JAVA_LANG_REFLECT_FIELD, JAVA_LANG_STRING,
    JAVA_LANG_THREAD, JAVA_LANG_THREAD_GROUP,
};
use crate::jenv::{
    alloc_jobject, get_java_class_object, get_java_string, get_object_field, new_java_lang_string,
    new_jclass, new_jobject, new_jobject_array, set_object_field, JTHREAD, THREADS,
};
use crate::jthread::JvmThread;
use crate::jvm::{execute_method, execute_method_by_name};
use std::convert::TryInto;
use std::sync::atomic::{AtomicPtr, Ordering};

pub fn java_lang_Class_getPrimitiveClass(
    thread: &mut JvmThread,
    class: &Class,
    mut args: Vec<JValue>,
) {
    let string_ref = args.pop().unwrap();
    let class_name = get_java_string(string_ref.as_jobject());
    let primitive_class = load_mirror_class(class.class_loader(), &class_name);
    thread.push_jobject(primitive_class.mirror_class());
}

pub fn java_lang_Class_getDeclaredFields0(
    thread: &mut JvmThread,
    class: &Class,
    args: Vec<JValue>,
) {
    let obj = args[0].as_jobject();
    let public_only = args[1].as_jbool();
    let class = get_class_by_id(obj.class_id());

    let fields: Vec<_> = if public_only {
        class.iter_fields().filter(|f| f.is_public()).collect()
    } else {
        class.iter_fields().collect()
    };
    let reflect_field_class = load_class(class.class_loader(), JAVA_LANG_REFLECT_FIELD);
    init_class(thread, &reflect_field_class);
    let obj_array = new_jobject_array(reflect_field_class.clone(), fields.len());

    for (i, f) in fields.iter().enumerate() {
        let field_obj = new_jobject(&reflect_field_class);
        set_object_field(
            field_obj,
            "clazz",
            &class_name_to_descriptor(JAVA_LANG_CLASS),
            class.mirror_class(),
        );
        set_object_field(field_obj, "slot", "I", i as JInt);
        set_object_field(
            field_obj,
            "name",
            &class_name_to_descriptor(JAVA_LANG_STRING),
            new_java_lang_string(f.name()),
        );
        let field_type = get_java_class_object(thread, class.class_loader(), f.descriptor());
        set_object_field(
            field_obj,
            "type",
            &class_name_to_descriptor(JAVA_LANG_CLASS),
            field_type,
        );
        set_object_field(
            field_obj,
            "modifiers",
            "I",
            f.access_flags() & JVM_RECOGNIZED_FIELD_MODIFIERS,
        );
        set_object_field(field_obj, "override", "Z", 0u8);
        // todo: generic signature
        // todo: annotation

        obj_array.set(i, field_obj);
    }
    println!("declared fields");
    pretty_print(obj_array);
    thread.push_jarray(obj_array);
}

pub fn jvm_desiredAssertionStatus0(thread: &mut JvmThread, _class: &Class, _args: Vec<JValue>) {
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jint(0);
}

pub fn java_lang_Float_floatToRawIntBits(
    thread: &mut JvmThread,
    _class: &Class,
    args: Vec<JValue>,
) {
    let n = args[0].as_jfloat();
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jint(n.to_bits() as i32);
}

pub fn java_lang_Double_doubleToRawLongBits(
    thread: &mut JvmThread,
    _class: &Class,
    args: Vec<JValue>,
) {
    let n = args[0].as_jdouble();
    let frame = thread.current_frame_mut();
    frame.operand_stack.push_jlong(n.to_bits() as i64);
}

pub fn java_lang_Double_longBitsToDouble(
    thread: &mut JvmThread,
    _class: &Class,
    args: Vec<JValue>,
) {
    let n = args[0].as_jlong();
    let frame = thread.current_frame_mut();
    frame
        .operand_stack
        .push_jdouble(f64::from_be_bytes(n.to_be_bytes()));
}

pub fn java_lang_System_initProperties(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    let props_ref = &args[0];
    let properties = props_ref.as_jobject();
    let class_id = properties.class_id();
    let propertiesClass = get_class_by_id(class_id);
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
        let key = JValue::Object(new_java_lang_string(key));
        let value = JValue::Object(new_java_lang_string(value));
        let args = vec![props_ref.clone(), key, value];
        execute_method(thread, method.clone(), args);
    }
    let frame = thread.current_frame_mut();
    frame.operand_stack.push(props_ref.clone());
}

pub fn java_lang_Object_hashCode(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    let obj = &args[0].as_jobject();
    thread
        .current_frame_mut()
        .operand_stack
        .push_jint(obj.hash_code());
}

pub fn java_lang_System_registerNatives(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
}

pub fn java_lang_Object_registerNatives(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
}

pub fn registerNatives(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {}

pub fn sun_misc_VM_initalize(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {}

pub fn sun_misc_Unsafe_registerNatives(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {}

pub fn sun_misc_Unsafe_arrayBaseOffset(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    let acls = args[1].as_jobject();
    let mirror_class = get_class_by_id(acls.class_id());
    println!("array base offset {:?}", mirror_class);
    assert!(!acls.is_null());
    let base_offset = ArrayOopDesc::base_offset_in_bytes();
    thread.push_jint(base_offset as JInt);
}

pub fn sun_misc_Unsafe_arrayIndexScale(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let (unsafe_obj, class_ojb) = match args.as_slice() {
        &[JValue::Object(unsafe_obj), JValue::Object(mirror)] => (unsafe_obj, mirror),
        _ => unreachable!(),
    };
    let mirror_class = get_class_by_id(class_ojb.class_id());
    let instance_mirror_class = mirror_class.as_instance_mirror_class().unwrap();
    let scale = match load_class(class.class_loader(), instance_mirror_class.mirror_name()) {
        Class::TypeArrayClass(c) => c.ty().size_in_bytes(),
        Class::ObjArrayClass(_) => std::mem::size_of::<usize>(),
        v => unreachable!("{:?}", v),
    };
    thread.push_jint(scale as JInt)
}

pub fn sun_misc_Unsafe_addressSize(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    thread
        .current_frame_mut()
        .operand_stack
        .push_jint(std::mem::size_of::<usize>() as JInt);
}

pub fn sun_reflect_Reflection_getCallerClass(
    thread: &mut JvmThread,
    class: &Class,
    args: Vec<JValue>,
) {
    let caller_class = if let Some(class) = thread.caller_class() {
        class.mirror_class()
    } else {
        JObject::null()
    };
    thread.push_jobject(caller_class);
}

pub fn java_io_FileInputStream_initIDs(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    // panic!()
}
pub fn java_io_FileDescriptor_initIDs(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    // panic!()
}

pub fn java_lang_Throwable_fillInStackTrace(
    thread: &mut JvmThread,
    _class: &Class,
    args: Vec<JValue>,
) {
    // todo: fill backtrace
    let obj = &args[0];
    thread.current_frame_mut().operand_stack.push(obj.clone());
}

pub fn java_io_FileOutputStream_initIDs(thread: &mut JvmThread, _class: &Class, args: Vec<JValue>) {
    // todo
}

pub fn java_security_AccessController_doPrivileged(
    thread: &mut JvmThread,
    class: &Class,
    args: Vec<JValue>,
) {
    let action = args[0].as_jobject();
    let class_id = action.class_id();
    let class = get_class_by_id(class_id);
    let method = class
        .get_method("run", "()Ljava/lang/Object;", false)
        .unwrap();
    execute_method(thread, method, vec![JValue::Object(action)])
}

pub fn java_lang_Thread_currentThread(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let mut thread_object = thread.object.clone();
    if thread_object.is_null() {
        let thread_group_class = load_class(JObject::null(), JAVA_LANG_THREAD_GROUP);
        init_class(thread, &thread_group_class);
        let thread_group_object = alloc_jobject(&thread_group_class);
        execute_method_by_name(
            thread,
            &thread_group_class,
            "<init>",
            "()V",
            false,
            vec![thread_group_object.into()],
        );

        let thread_class = load_class(JObject::null(), JAVA_LANG_THREAD);
        init_class(thread, &thread_class);
        thread_object = alloc_jobject(&thread_class);
        set_object_field(
            thread_object,
            "group",
            "Ljava/lang/ThreadGroup;",
            thread_group_object,
        );
        set_object_field(thread_object, "priority", "I", 1);
        let thread_id = thread_id::get() as i64;
        THREADS.lock().insert(thread_id);
        set_object_field(thread_object, "tid", "J", thread_id);

        thread.set_thread_object(thread_object);
    }
    thread.push_jobject(thread_object)
}

pub fn java_lang_Class_getName0(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let jclass = args[0].as_jobject();
    let mirror_class = get_class_by_id(jclass.class_id());
    let addr = new_java_lang_string(
        mirror_class
            .as_instance_mirror_class()
            .unwrap()
            .mirror_name(),
    );
    thread.current_frame_mut().operand_stack.push_jobject(addr);
}

pub fn java_lang_Class_for_Name0(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let name = get_java_string(args[0].as_jobject());
    let class_name = name.replace('.', "/");
    let class = load_class(class.class_loader(), &class_name);
    thread.push_jobject(class.mirror_class());
}

pub fn java_security_AccessController_getStackAccessControlContext(
    thread: &mut JvmThread,
    class: &Class,
    args: Vec<JValue>,
) {
    thread.push_jobject(JObject::null());
}

pub fn java_lang_Thread_setPriority0(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let thread_obj = args[0].as_jobject();
    let priority = args[1].as_jint();
    set_object_field(thread_obj, "priority", "I", priority);
    // let tid = get_object_field(thread_obj, "tid", "J");
    // set tid
}

pub fn java_lang_Thread_isAlive(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let obj = args[0].as_jobject();
    let tid: JLong = get_object_field(obj, "tid", "J");
    let alive = THREADS.lock().contains(&tid);
    thread.current_frame_mut().operand_stack.push_jbool(alive)
}

pub fn java_lang_Thread_start0(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let obj = args[0].as_jobject();
    let class = class.clone();
    std::thread::spawn(move || {
        let thread_id = thread_id::get() as i64;
        JTHREAD.with(|thread| {
            execute_method_by_name(
                &mut thread.borrow_mut(),
                &class,
                "run",
                "()V",
                false,
                vec![obj.into()],
            )
        });
        THREADS.lock().insert(thread_id);
    });
}

pub fn sun_misc_Unsafe_compareAndSwapObject(
    thread: &mut JvmThread,
    class: &Class,
    args: Vec<JValue>,
) {
    let (unsafe_obj, p, offset, e, x) = match args.as_slice() {
        [JValue::Object(unsafe_obj), JValue::Object(p), JValue::Long(offset), JValue::Object(e), JValue::Object(x)] => {
            (unsafe_obj, p, offset, e, x)
        }
        _ => unreachable!(),
    };
    let success = compare_and_swap_object(unsafe_obj, *offset, e, x);
    thread.push_jbool(success)
}

pub fn java_lang_System_arraycopy(thread: &mut JvmThread, class: &Class, args: Vec<JValue>) {
    let (src, src_pos, dst, dst_pos, length) = match args.as_slice() {
        &[JValue::Object(src), JValue::Int(src_pos), JValue::Object(dst), JValue::Int(dst_pos), JValue::Int(length)] => {
            (src, src_pos, dst, dst_pos, length)
        }
        _ => unreachable!(),
    };
    let src_array: JArray = src.into();
    let dst_array: JArray = dst.into();
    copy_array(
        thread,
        src_array.array_oop(),
        src_pos.try_into().unwrap(),
        dst_array.array_oop(),
        dst_pos.try_into().unwrap(),
        length.try_into().unwrap(),
    );
}

fn compare_and_swap_object(obj: &JObject, offset: i64, expect: &JObject, target: &JObject) -> bool {
    let target_oop = target.oop().address();
    let expect_oop = expect.oop().address();

    let addr = obj
        .oop()
        .address()
        .plus(InstanceOopDesc::base_offset_in_bytes() + offset as usize);
    let p = addr.as_atomic_ptr();
    let old = p.compare_and_swap(
        expect_oop.as_mut_ptr(),
        target_oop.as_mut_ptr(),
        Ordering::SeqCst,
    );
    let success = old == expect_oop.as_mut_ptr();
    success
}

#[cfg(test)]
mod tests {
    use crate::class_loader::load_class;
    use crate::debug::pretty_print;
    use crate::gc::global_definition::{BasicType, JArray, JObject};
    use crate::gc::oop::Oop;
    use crate::gc::oop_desc::InstanceOopDesc;
    use crate::java_const::JAVA_LANG_STRING;
    use crate::jenv::{alloc_jobject, new_java_lang_string, new_jtype_array};
    use crate::jvm::Jvm;
    use crate::native::compare_and_swap_object;
    use std::sync::atomic::{AtomicPtr, Ordering};

    #[test]
    fn test_compare_and_swap_object() {
        let _jvm = Jvm::new(Some("./jre".to_string()), Some("./jre/lib/rt".to_string()));
        let bytes_str: Vec<u16> = "hello".encode_utf16().collect();
        let expect = new_jtype_array(BasicType::Char, bytes_str.len());
        expect.copy_from(&bytes_str);
        let bytes_str2: Vec<u16> = "world".encode_utf16().collect();
        let target = new_jtype_array(BasicType::Char, bytes_str.len());
        target.copy_from(&bytes_str2);
        let class = load_class(JObject::null(), JAVA_LANG_STRING);
        let obj = alloc_jobject(&class);
        let f = class.get_field("value", "[C").unwrap();
        obj.set_field_by_offset(f.offset(), expect);

        let ret = compare_and_swap_object(&obj, f.offset() as i64, &expect.into(), &target.into());
        assert_eq!(obj.get_field_by_offset::<JArray>(f.offset()), target);
        assert!(ret);
    }
}
