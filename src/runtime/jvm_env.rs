use crate::class_path::ClassPath;
use crate::runtime::class::Class;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::execute_method;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::frame::JvmFrame;
use crate::runtime::heap::{JvmHeap, STRING_CLASS_NAME};
use crate::runtime::method::Method;
use std::collections::{HashMap, VecDeque};
use tracing::{debug, debug_span};

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
        let class = self.class_loader.load_class(class_name);
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
        class
    }

    pub fn new_java_string(&mut self, s: &str) -> u32 {
        let bytes_str = s.as_bytes();
        let array = self.heap.new_byte_array(bytes_str.len() as i32);
        let mut fields = HashMap::new();
        fields.insert("value".to_string(), Operand::ArrayRef(array));

        let class = self.load_and_init_class(STRING_CLASS_NAME);
        let obj_ref = self.heap.new_object(class);
        obj_ref as u32
    }

    pub(crate) fn did_override_method(&mut self, method: &Method, other: &Method) -> bool {
        if method == other {
            return true;
        }
        let this_class = self.load_and_init_class(method.name());
        let other_class = self.load_and_init_class(other.name());
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
