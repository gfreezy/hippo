use crate::class::{alloc_jarray, alloc_jobject, Class, Method};
use crate::class_loader::{get_class_id_by_name, load_class};

use crate::gc::global_definition::{BasicType, JArray, JObject};

use crate::java_const::JAVA_LANG_STRING;
use crate::jthread::JvmThread;
use std::cell::RefCell;

thread_local! {
    pub static JTHREAD: RefCell<JvmThread> = RefCell::new(JvmThread::new());
}

pub fn new_jobject(class: Class) -> JObject {
    alloc_jobject(&class)
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

pub fn new_java_lang_string(s: &str) -> JObject {
    let bytes_str: Vec<u16> = s.encode_utf16().collect();
    let array = new_jtype_array(BasicType::Char, bytes_str.len());
    array.copy_from(&bytes_str);
    let class = load_class(JObject::null(), JAVA_LANG_STRING);
    let obj = alloc_jobject(&class);
    let f = class.get_field("value", "[C").unwrap();
    obj.set_field_by_offset(f.offset(), array);
    obj
}

pub fn get_java_string(obj: JObject) -> String {
    let class = load_class(JObject::null(), JAVA_LANG_STRING);
    let f = class.get_field("value", "[C").unwrap();
    let chars_ref = obj.get_field_by_offset::<JArray>(f.offset());
    let bytes: Vec<u16> = chars_ref.as_slice().to_vec();
    String::from_utf16(&bytes).unwrap()
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
    use crate::gc::tlab::initialize_tlab;
    use crate::jenv::{get_java_string, new_java_lang_string};
    use std::sync::Arc;

    fn init() {
        let jre = Some("./jre".to_string());
        let cp = Some("./jre/lib/rt".to_string());

        BOOTSTRAP_LOADER.set(BootstrapClassLoader::new(ClassPath::new(jre, cp)));
        initialize_tlab(AllocatorLocal::new(Arc::new(Space::new(1024 * 1024 * 100))));
    }

    #[test]
    fn test_new_string() {
        init();
        let s = "hello";
        let obj = new_java_lang_string(s);
        let s2 = get_java_string(obj);
        assert_eq!(s, s2);
    }
}
