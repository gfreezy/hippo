use crate::class::{Class, Method};
use crate::gc::global_definition::JValue;
use crate::jthread::JvmThread;
use crate::native::*;
use tracing::debug;

pub fn execute_native_method(
    thread: &mut JvmThread,
    class: &Class,
    method: Method,
    args: Vec<JValue>,
) {
    let frame = thread.current_frame_mut();
    debug!(
        ?frame,
        ?args,
        %method,
        descriptor = method.descriptor(),
        "execute_native_method"
    );

    match (class.name(), method.name(), method.descriptor()) {
        ("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;") => {
            java_lang_Class_getPrimitiveClass(thread, class, args);
        }
        (_, "desiredAssertionStatus0", "(Ljava/lang/Class;)Z") => {
            jvm_desiredAssertionStatus0(thread, class, args);
        }
        ("java/lang/Float", "floatToRawIntBits", "(F)I") => {
            java_lang_Float_floatToRawIntBits(thread, class, args);
        }
        ("java/lang/Double", "doubleToRawLongBits", "(D)J") => {
            java_lang_Double_doubleToRawLongBits(thread, class, args);
        }
        ("java/lang/Double", "longBitsToDouble", "(J)D") => {
            java_lang_Double_longBitsToDouble(thread, class, args);
        }
        (
            "java/lang/System",
            "initProperties",
            "(Ljava/util/Properties;)Ljava/util/Properties;",
        ) => {
            java_lang_System_initProperties(thread, class, args);
        }
        ("java/lang/Object", "hashCode", "()I") => {
            java_lang_Object_hashCode(thread, class, args);
        }
        ("java/lang/System", "registerNatives", "()V") => {
            java_lang_System_registerNatives(thread, class, args);
        }
        ("java/lang/Object", "registerNatives", "()V") => {
            java_lang_Object_registerNatives(thread, class, args);
        }
        ("java/lang/Class", "registerNatives", "()V") => {
            registerNatives(thread, class, args);
        }
        ("java/lang/ClassLoader", "registerNatives", "()V") => {
            registerNatives(thread, class, args);
        }
        ("java/lang/Thread", "registerNatives", "()V") => {
            registerNatives(thread, class, args);
        }
        ("sun/misc/VM", "initialize", "()V") => {
            sun_misc_VM_initialize(thread, class, args);
        }
        ("sun/misc/Unsafe", "registerNatives", "()V") => {
            sun_misc_Unsafe_registerNatives(thread, class, args);
        }
        ("sun/misc/Unsafe", "arrayBaseOffset", "(Ljava/lang/Class;)I") => {
            sun_misc_Unsafe_arrayBaseOffset(thread, class, args);
        }
        ("sun/misc/Unsafe", "arrayIndexScale", "(Ljava/lang/Class;)I") => {
            sun_misc_Unsafe_arrayIndexScale(thread, class, args);
        }
        ("sun/misc/Unsafe", "addressSize", "()I") => {
            sun_misc_Unsafe_addressSize(thread, class, args);
        }
        ("sun/reflect/Reflection", "getCallerClass", "()Ljava/lang/Class;") => {
            sun_reflect_Reflection_getCallerClass(thread, class, args);
        }
        ("java/io/FileInputStream", "initIDs", "()V") => {
            java_io_FileInputStream_initIDs(thread, class, args);
        }
        ("java/io/FileDescriptor", "initIDs", "()V") => {
            java_io_FileDescriptor_initIDs(thread, class, args);
        }
        ("java/lang/Throwable", "fillInStackTrace", "(I)Ljava/lang/Throwable;") => {
            java_lang_Throwable_fillInStackTrace(thread, class, args);
        }
        ("java/io/FileOutputStream", "initIDs", "()V") => {
            java_io_FileOutputStream_initIDs(thread, class, args);
        }
        (
            "java/security/AccessController",
            "doPrivileged",
            "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
        ) => {
            java_security_AccessController_doPrivileged(thread, class, args);
        }
        (
            "java/security/AccessController",
            "doPrivileged",
            "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
        ) => {
            java_security_AccessController_doPrivileged(thread, class, args);
        }
        ("java/lang/Thread", "currentThread", "()Ljava/lang/Thread;") => {
            java_lang_Thread_currentThread(thread, class, args);
        }
        ("java/lang/Class", "getName0", "()Ljava/lang/String;") => {
            java_lang_Class_getName0(thread, class, args);
        }
        (
            "java/lang/Class",
            "forName0",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;",
        ) => {
            java_lang_Class_for_Name0(thread, class, args);
        }
        (
            "java/security/AccessController",
            "getStackAccessControlContext",
            "()Ljava/security/AccessControlContext;",
        ) => {
            java_security_AccessController_getStackAccessControlContext(thread, class, args);
        }
        ("java/lang/Thread", "setPriority0", "(I)V") => {
            java_lang_Thread_setPriority0(thread, class, args)
        }
        ("java/lang/Thread", "isAlive", "()Z") => java_lang_Thread_isAlive(thread, class, args),
        ("java/lang/Thread", "start0", "()V") => {
            java_lang_Thread_start0(thread, class, args);
        }
        ("java/lang/Class", "getDeclaredFields0", "(Z)[Ljava/lang/reflect/Field;") => {
            java_lang_Class_getDeclaredFields0(thread, class, args);
        }
        (
            "sun/misc/Unsafe",
            "compareAndSwapObject",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
        ) => {
            sun_misc_Unsafe_compareAndSwapObject(thread, class, args);
        }
        ("java/lang/System", "arraycopy", "(Ljava/lang/Object;ILjava/lang/Object;II)V") => {
            java_lang_System_arraycopy(thread, class, args);
        }
        ("java/lang/String", "intern", "()Ljava/lang/String;") => {
            java_lang_String_intern(thread, class, args);
        }
        ("sun/misc/Unsafe", "objectFieldOffset", "(Ljava/lang/reflect/Field;)J") => {
            sun_misc_Unsafe_objectFieldOffset(thread, class, args);
        }
        ("java/lang/Class", "isPrimitive", "()Z") => {
            java_lang_Class_isPrimitive(thread, class, args);
        }
        ("java/lang/Class", "isAssignableFrom", "(Ljava/lang/Class;)Z") => {
            java_lang_Class_isAssignableFrom(thread, class, args);
        }
        ("java/lang/System", "setIn0", "(Ljava/io/InputStream;)V") => {
            java_lang_System_setIn0(thread, class, args);
        }
        ("java/lang/System", "setOut0", "(Ljava/io/PrintStream;)V") => {
            java_lang_System_setOut0(thread, class, args);
        }
        ("java/lang/System", "setErr0", "(Ljava/io/PrintStream;)V") => {
            java_lang_System_setErr0(thread, class, args);
        }
        ("sun/misc/Unsafe", "getIntVolatile", "(Ljava/lang/Object;J)I") => {
            sun_misc_Unsafe_getIntVolatile(thread, class, args);
        }
        ("sun/misc/Unsafe", "compareAndSwapInt", "(Ljava/lang/Object;JII)Z") => {
            sun_misc_Unsafe_compareAndSwapInt(thread, class, args);
        }
        ("java/lang/Class", "isInterface", "()Z") => {
            java_lang_Class_isInterface(thread, class, args);
        }
        (class_name, name, descriptor) => {
            panic!(
                r#"native method: is_static: {}, ("{}", "{}", "{}") => {{
                    {}_{}(thread, class, args);
                }}"#,
                method.is_static(),
                class_name,
                name,
                descriptor,
                class_name.replace("/", "_"),
                name,
            );
        }
    };
}
