use crate::runtime::class::Class;
use crate::runtime::frame::JvmFrame;
use crate::runtime::method::Method;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct JvmStack {
    pub frames: VecDeque<JvmFrame>,
}

#[derive(Debug)]
pub struct JvmThread {
    pub stack: JvmStack,
    pub object_addr: u32,
}

impl JvmThread {
    pub fn new() -> Self {
        JvmThread {
            stack: JvmStack {
                frames: Default::default(),
            },
            object_addr: 0,
        }
    }

    pub fn current_frame_mut(&mut self) -> &mut JvmFrame {
        self.stack.frames.back_mut().unwrap()
    }

    pub fn current_frame(&self) -> Option<&JvmFrame> {
        self.stack.frames.back()
    }

    pub fn current_class(&self) -> Option<Class> {
        Some(self.current_frame()?.class.clone())
    }

    pub fn current_method(&self) -> Option<Method> {
        Some(self.current_frame()?.method.clone())
    }
}
