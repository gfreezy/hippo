use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::class_loader::BootstrapClassLoader;
use crate::runtime::execute_method;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::heap::{
    JvmHeap, JAVA_LANG_CLASS, JAVA_LANG_STRING, JAVA_LANG_STRING_DESCRIPTOR, JAVA_LANG_THREAD,
    JAVA_LANG_THREAD_GROUP, JAVA_LANG_THREAD_GROUP_DESCRIPTOR,
};
use crate::runtime::jvm_thread::JvmThread;
use crate::runtime::method::Method;
use nom::lib::std::collections::HashMap;
use std::hash::{Hash, Hasher};
use tracing::{debug, debug_span};

const JAVA_STRING_FIELD_VALUE_INDEX: usize = 0;
const JAVA_STRING_FIELD_HASH_INDEX: usize = 1;

pub type JvmPC = usize;

#[derive(Debug)]
pub struct ClassId {
    name: String,
    classloader: Operand,
}

impl Hash for ClassId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes())
    }
}

impl PartialEq for ClassId {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ClassId {}

#[derive(Debug)]
pub struct JvmEnv {
    pub heap: JvmHeap,
    pub thread: JvmThread,
    pub bootstrap_class_loader: BootstrapClassLoader,
    pub defining_classes: HashMap<ClassId, Class>,
    pub initiating_classes: HashMap<ClassId, Class>,
}

impl JvmEnv {
    pub fn new(jre_opt: Option<String>, cp_opt: Option<String>) -> Self {
        let mut jenv = JvmEnv {
            heap: JvmHeap::new(),
            thread: JvmThread::new(),
            bootstrap_class_loader: BootstrapClassLoader::new(ClassPath::new(jre_opt, cp_opt)),
            defining_classes: Default::default(),
            initiating_classes: Default::default(),
        };
        let thread_addr = jenv.new_java_lang_thread("main");
        jenv.thread.object_addr = thread_addr;
        jenv
    }

    pub fn get_classloader(&self, class: &Class) -> Operand {
        self.defining_classes
            .keys()
            .find_map(|k| {
                if k.name == class.name() {
                    Some(k.classloader.clone())
                } else {
                    None
                }
            })
            .unwrap_or(Operand::Null)
    }

    pub fn load_and_init_class(&mut self, class_name: &str) -> Class {
        let current_class = self.thread.current_class();
        let class_loader_addr = (current_class.as_ref())
            .map(|c| self.get_classloader(c))
            .unwrap_or(Operand::Null);
        if let Some(class) = self.initiating_classes.get(&ClassId {
            name: class_name.to_string(),
            classloader: class_loader_addr.clone(),
        }) {
            return class.clone();
        }

        let mut class = if class_loader_addr == Operand::Null {
            let class = self.bootstrap_class_loader.load_class(class_name);
            self.defining_classes.insert(
                ClassId {
                    name: class_name.to_string(),
                    classloader: class_loader_addr.clone(),
                },
                class.clone(),
            );
            class
        } else {
            let class_loader = self.heap.get_object(&class_loader_addr);
            let load_class_method = class_loader
                .get_method_by_name("loadClass", "(Ljava/lang/String;)Ljava/lang/Class;", false)
                .unwrap();
            let jclass_name = self.new_java_lang_string(class_name);
            execute_method(
                self,
                load_class_method,
                vec![Operand::ObjectRef(jclass_name)],
            );
            let _jclass_addr = self.thread.current_frame_mut().operand_stack.pop();
            self.load_and_init_class(class_name)
        };

        self.initiating_classes.insert(
            ClassId {
                name: class_name.to_string(),
                classloader: class_loader_addr,
            },
            class.clone(),
        );

        if let Class::InstanceClass(class) = &mut class {
            if !class.is_inited() {
                let span = debug_span!("init_class", %class_name);
                let _s = span.enter();
                class.set_inited();
                debug!("init successfully.");
                let clinit_method = class.clinit_method();
                if let Some(clinit_method) = clinit_method {
                    execute_method(self, clinit_method, vec![]);
                }
            }
        }
        class
    }

    pub fn new_java_lang_string(&mut self, s: &str) -> u32 {
        let bytes_str = s.encode_utf16();
        let array = self.heap.new_char_array(bytes_str.collect());

        let class = self.load_and_init_class(JAVA_LANG_STRING);
        let (object, addr) = self.heap.new_object(class);
        object.set_field_by_name("value", "[C", Operand::ArrayRef(array));
        object.set_field_by_name("hash", "I", Operand::Int(addr as i32));
        addr
    }

    pub fn new_java_lang_class(&mut self, name: &str) -> u32 {
        let class = self.load_and_init_class(JAVA_LANG_CLASS);
        class.set_mirror_class_name(name.to_string());
        let (_, addr) = self.heap.new_object(class);
        addr
    }

    pub fn new_java_lang_thread(&mut self, name: &str) -> u32 {
        let jstring_main = self.new_java_lang_string("main");
        let jstring_thread_name = self.new_java_lang_string(name);
        let thread_class = self.load_and_init_class(JAVA_LANG_THREAD);
        let thread_group_class = self.load_and_init_class(JAVA_LANG_THREAD_GROUP);
        let (jthread_group, jthread_group_addr) = self.heap.new_object(thread_group_class);
        jthread_group.set_field_by_name(
            "name",
            JAVA_LANG_STRING_DESCRIPTOR,
            Operand::ObjectRef(jstring_main),
        );
        let (jthread, jthread_addr) = self.heap.new_object(thread_class);
        jthread.set_field_by_name(
            "name",
            JAVA_LANG_STRING_DESCRIPTOR,
            Operand::ObjectRef(jstring_thread_name),
        );
        jthread.set_field_by_name(
            "group",
            JAVA_LANG_THREAD_GROUP_DESCRIPTOR,
            Operand::ObjectRef(jthread_group_addr),
        );
        jthread.set_field_by_name("priority", "I", Operand::Int(5));

        jthread_addr
    }

    pub fn get_java_string(&mut self, str_ref: &Operand) -> String {
        let string_operand = self.heap.get_object(str_ref);
        let chars_ref = string_operand.get_field_by_name("value", "[C");
        String::from_utf16(self.heap.get_char_array(chars_ref)).unwrap()
    }

    pub fn did_override_method(&mut self, method: &Method, other: &Method) -> bool {
        if method == other {
            return true;
        }
        let this_class = self.load_and_init_class(method.name()).instance_class();
        let other_class = self.load_and_init_class(other.name()).instance_class();
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
}
