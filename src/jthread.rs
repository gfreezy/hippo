use crate::class::{Class, Method};
use crate::frame::JvmFrame;
use crate::gc::global_definition::{JInt, JObject};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct JvmStack {
    pub frames: VecDeque<JvmFrame>,
}

#[derive(Debug)]
pub struct JvmThread {
    pub stack: JvmStack,
    pub object: JObject,
}

impl JvmThread {
    pub fn new() -> Self {
        JvmThread {
            stack: JvmStack {
                frames: Default::default(),
            },
            object: JObject::null(),
        }
    }

    pub fn current_frame_mut(&mut self) -> &mut JvmFrame {
        self.stack.frames.back_mut().unwrap()
    }

    pub fn current_frame(&self) -> Option<&JvmFrame> {
        self.stack.frames.back()
    }

    pub fn caller_frame(&self) -> Option<&JvmFrame> {
        let frames = &self.stack.frames;
        let len = frames.len();
        if len >= 2 {
            Some(&frames[len - 2])
        } else {
            None
        }
    }

    pub fn caller_class(&self) -> Option<Class> {
        Some(self.caller_frame()?.class())
    }

    pub fn caller_method(&self) -> Option<Method> {
        Some(self.caller_frame()?.method())
    }
    pub fn pop_frame(&mut self) {
        self.stack.frames.pop_back();
    }
    pub fn push_frame(&mut self, frame: JvmFrame) {
        self.stack.frames.push_back(frame);
    }

    pub fn current_class(&self) -> Option<Class> {
        Some(self.current_frame()?.class.clone())
    }

    pub fn current_method(&self) -> Option<Method> {
        Some(self.current_frame()?.method.clone())
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        self.current_frame_mut().code_reader.read_u8()
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        self.current_frame_mut().code_reader.read_u16()
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        self.current_frame_mut().code_reader.read_i16()
    }

    pub fn pc(&self) -> usize {
        self.current_frame().unwrap().code_reader.pc()
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.current_frame_mut().code_reader.set_pc(pc)
    }
}
