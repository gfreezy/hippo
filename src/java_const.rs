pub const JAVA_LANG_CLASS: &str = "java/lang/Class";
pub const JAVA_LANG_STRING: &str = "java/lang/String";
pub const JAVA_LANG_OBJECT: &str = "java/lang/Object";
pub const JAVA_LANG_THREAD: &str = "java/lang/Thread";
pub const JAVA_LANG_THROWABLE: &str = "java/lang/Throwable";
pub const JAVA_LANG_ARRAY_INDEX_OUT_OF_BOUNDS_EXCEPTION: &str =
    "java/lang/ArrayIndexOutOfBoundsException";
pub const JAVA_LANG_THREAD_GROUP: &str = "java/lang/ThreadGroup";
pub const JAVA_LANG_REFLECT_FIELD: &str = "java/lang/reflect/Field";
pub const JAVA_LANG_INVOKE_METHOD_TYPE: &str = "java.lang.invoke.MethodType";

pub fn class_name_to_descriptor(name: &str) -> String {
    format!("L{};", name)
}
