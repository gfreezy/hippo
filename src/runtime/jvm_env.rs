use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::execute_method;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::frame::JvmFrame;
use crate::runtime::heap::{JvmHeap, CLASS_CLASS_NAME, STRING_CLASS_NAME};
use crate::runtime::method::Method;
use std::collections::VecDeque;
use tracing::{debug, debug_span};

const JAVA_STRING_FIELD_VALUE_INDEX: usize = 0;
const JAVA_STRING_FIELD_HASH_INDEX: usize = 1;

pub type JvmPC = usize;

#[derive(Debug)]
pub struct JvmStack {
    pub frames: VecDeque<JvmFrame>,
}

#[derive(Debug)]
pub struct JvmThread {
    pub stack: JvmStack,
    pub pc: JvmPC,
}

#[derive(Debug)]
pub struct JvmEnv {
    pub heap: JvmHeap,
    pub thread: JvmThread,
    pub class_loader: ClassLoader,
}

impl JvmEnv {
    pub fn new(jre_opt: Option<String>, cp_opt: Option<String>) -> Self {
        JvmEnv {
            heap: JvmHeap::new(),
            thread: JvmThread {
                stack: JvmStack {
                    frames: Default::default(),
                },
                pc: 0,
            },
            class_loader: ClassLoader::new(ClassPath::new(jre_opt, cp_opt)),
        }
    }

    pub fn load_and_init_class(&mut self, class_name: &str) -> Class {
        let mut class = self.class_loader.load_class(class_name);
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

    pub fn new_java_string(&mut self, s: &str) -> u32 {
        let bytes_str = s.encode_utf16();
        let array = self.heap.new_char_array(bytes_str.collect());

        let class = self.load_and_init_class(STRING_CLASS_NAME);
        let obj_ref = self.heap.new_object(class);
        let object = self.heap.get_object_mut(&Operand::ObjectRef(obj_ref));
        object.set_field(JAVA_STRING_FIELD_VALUE_INDEX, Operand::ArrayRef(array));
        object.set_field(JAVA_STRING_FIELD_HASH_INDEX, Operand::Int(obj_ref as i32));
        obj_ref
    }

    pub fn new_java_class(&mut self, _s: &str) -> u32 {
        let class = self.load_and_init_class(CLASS_CLASS_NAME);
        let obj_ref = self.heap.new_object(class);
        obj_ref
    }

    pub fn get_java_string(&mut self, str_ref: &Operand) -> String {
        let string_operand = self.heap.get_object(str_ref);
        let chars_ref = string_operand.get_field(JAVA_STRING_FIELD_VALUE_INDEX);
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
