use crate::class::{Class, ClassId, Method};
use crate::class_loader::{
    get_class_by_id, get_class_id_by_name, init_class, load_class, load_mirror_class,
};

use crate::gc::global_definition::{
    BasicType, JArray, JBoolean, JByte, JChar, JDouble, JFloat, JInt, JLong, JObject, JShort,
};

use crate::gc::global_definition::type_to_basic_type::TypeToBasicType;
use crate::gc::oop::Oop;
use crate::gc::oop_desc::{ArrayOopDesc, InstanceOopDesc};
use crate::gc::tlab::{alloc_tlab, occupy_remaining_tlab};
use crate::java_const::{JAVA_LANG_ARRAY_INDEX_OUT_OF_BOUNDS_EXCEPTION, JAVA_LANG_STRING};
use crate::jthread::JvmThread;
use crate::string_table::StringTable;
use nom::lib::std::collections::HashSet;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::cell::RefCell;
use std::sync::atomic::AtomicUsize;

thread_local! {
    pub static JTHREAD: RefCell<JvmThread> = RefCell::new(JvmThread::new());
}

pub static THREADS: Lazy<Mutex<HashSet<i64>>> = Lazy::new(|| Mutex::new(HashSet::new()));
pub static FRAME_ID: AtomicUsize = AtomicUsize::new(0);
pub static OPCODE_ID: AtomicUsize = AtomicUsize::new(0);
pub static STRING_MAP: Lazy<StringTable> = Lazy::new(|| StringTable::new());

const ALIGN: usize = 8;
fn alloc_memory(size: usize) -> Oop {
    let oop = alloc_tlab(size, ALIGN);
    oop.clear(size);
    // print!("oop: {:?}, size: {:?}, ", oop, size);
    if let Some((occupy_oop, size)) = occupy_remaining_tlab(ALIGN) {
        let class_id = Lazy::new(|| {
            let _class = load_class(JObject::null(), "[B");
            get_class_id_by_name("[B")
        });
        let base_offset = ArrayOopDesc::base_offset_in_bytes();
        let len = size - base_offset;
        let _ = JArray::new(occupy_oop, *class_id, len);
        // println!("occupy_oop: {:?}, size: {:?}", occupy_oop, size);
    }
    // println!("-------------");
    oop
}

pub fn alloc_jobject(class: &Class) -> JObject {
    let size = class.instance_size() + InstanceOopDesc::header_size_in_bytes();

    let jobj = JObject::new(alloc_memory(size), get_class_id_by_name(class.name()));
    // println!("alloc_jobject: {:?}, class: {:?}", jobj, class);
    jobj
}

pub fn alloc_empty_jobject() -> JObject {
    let size = InstanceOopDesc::header_size_in_bytes();
    let jobj = JObject::new(alloc_memory(size), 0);
    // println!("alloc_empyt_jobj: {:?}", jobj);
    jobj
}

pub fn alloc_jarray(ty: BasicType, class_id: ClassId, len: usize) -> JArray {
    let size = match ty {
        BasicType::Boolean => ArrayOopDesc::array_size_in_bytes::<JBoolean>(len),
        BasicType::Char => ArrayOopDesc::array_size_in_bytes::<JChar>(len),
        BasicType::Float => ArrayOopDesc::array_size_in_bytes::<JFloat>(len),
        BasicType::Double => ArrayOopDesc::array_size_in_bytes::<JDouble>(len),
        BasicType::Byte => ArrayOopDesc::array_size_in_bytes::<JByte>(len),
        BasicType::Short => ArrayOopDesc::array_size_in_bytes::<JShort>(len),
        BasicType::Int => ArrayOopDesc::array_size_in_bytes::<JInt>(len),
        BasicType::Long => ArrayOopDesc::array_size_in_bytes::<JLong>(len),
        BasicType::Object => ArrayOopDesc::array_size_in_bytes::<JObject>(len),
        BasicType::Array => unreachable!(),
    };
    let jarray = JArray::new(alloc_memory(size), class_id, len);
    // println!("alloc_jarray: {:?}, ty: {:?}, len: {}", jarray, ty, len);
    jarray
}

pub fn new_jobject(class: &Class) -> JObject {
    alloc_jobject(&class)
}

pub fn new_jclass(class: &Class) -> JObject {
    new_jobject(class)
}

pub fn new_jtype_array(basic_ty: BasicType, len: usize) -> JArray {
    let class = load_class(JObject::null(), &format!("[{}", basic_ty.descriptor()));
    let class_id = get_class_id_by_name(class.name());
    alloc_jarray(basic_ty, class_id, len)
}

pub fn new_jobject_array(class: Class, len: usize) -> JArray {
    let array_class = load_class(JObject::null(), &format!("[L{};", class.name()));
    let class_id = get_class_id_by_name(array_class.name());
    alloc_jarray(BasicType::Object, class_id, len)
}

pub fn get_java_class_object(thread: &mut JvmThread, loader: JObject, name: &str) -> JObject {
    let ty: BasicType = name.into();
    if ty.is_reference_type() {
        let class = load_class(loader, name);
        init_class(thread, &class);
        class.mirror_class()
    } else {
        let mirror_class = load_mirror_class(loader, name);
        mirror_class.mirror_class()
    }
}

pub fn new_array_index_out_of_bounds_exception(thread: &mut JvmThread) -> JObject {
    let class = load_class(
        JObject::null(),
        JAVA_LANG_ARRAY_INDEX_OUT_OF_BOUNDS_EXCEPTION,
    );
    init_class(thread, &class);
    let obj = alloc_jobject(&class);
    obj
}

pub fn new_java_lang_string(s: &str) -> JObject {
    if let Some(j) = STRING_MAP.get(s) {
        return j;
    }
    let bytes_str: Vec<u16> = s.encode_utf16().collect();
    let array = new_jtype_array(BasicType::Char, bytes_str.len());
    array.copy_from(&bytes_str);
    let class = load_class(JObject::null(), JAVA_LANG_STRING);
    let obj = alloc_jobject(&class);
    let f = class.get_field("value", "[C").unwrap();
    obj.set_field_by_offset(f.offset(), array);
    STRING_MAP.intern(s, obj);
    obj
}

pub fn get_java_string(obj: JObject) -> String {
    let class = load_class(JObject::null(), JAVA_LANG_STRING);
    let f = class.get_field("value", "[C").unwrap();
    let chars_ref = obj.get_field_by_offset::<JArray>(f.offset());
    let bytes: Vec<u16> = chars_ref.as_slice().to_vec();
    String::from_utf16(&bytes).unwrap()
}

pub fn set_object_field<T>(obj: JObject, name: &str, descriptor: &str, value: T)
where
    TypeToBasicType<T>: Into<BasicType>,
{
    let class_id = obj.class_id();
    let class = get_class_by_id(class_id);
    let f = class.get_field(name, descriptor).unwrap();
    obj.set_field_by_offset::<T>(f.offset(), value);
}

pub fn get_object_field<T>(obj: JObject, name: &str, descriptor: &str) -> T
where
    TypeToBasicType<T>: Into<BasicType>,
{
    let class_id = obj.class_id();
    let class = get_class_by_id(class_id);
    let f = class.get_field(name, descriptor).unwrap();
    obj.get_field_by_offset::<T>(f.offset())
}

pub fn did_override_method(method: &Method, other: &Method) -> bool {
    if method == other {
        return true;
    }
    let this_class = load_class(method.class_loader(), method.class_name());
    let other_class = load_class(method.class_loader(), other.class_name());
    if !this_class.is_subclass_of(other_class) {
        return false;
    }
    if method.name() != other.name() {
        return false;
    }
    if method.descriptor() != other.descriptor() {
        return false;
    }
    if method.is_private() {
        return false;
    }
    if (other.is_protected() || other.is_public())
        || (!other.is_public() && !other.is_private() && !other.is_protected())
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;
    use crate::class_loader::class_path::ClassPath;
    use crate::class_loader::BOOTSTRAP_LOADER;
    use crate::gc::allocator_local::AllocatorLocal;
    use crate::gc::space::Space;
    use crate::jenv::{get_java_string, new_java_lang_string};
    use crate::jvm::Jvm;
    use std::sync::Arc;

    fn init() -> Jvm {
        let jre = Some("./jre".to_string());
        let cp = Some("./jre/lib/rt".to_string());
        Jvm::new(jre, cp)
    }

    #[test]
    fn test_new_string() {
        let _ = init();
        let s = "hello";
        let obj = new_java_lang_string(s);
        let s2 = get_java_string(obj);
        assert_eq!(s, s2);
    }
}
