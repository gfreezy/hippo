use crate::class::{Class, Method};
use crate::frame::operand_stack::OperandStack;
use crate::frame::JvmFrame;
use crate::gc::global_definition::{JArray, JDouble, JFloat, JInt, JLong, JObject, JValue};
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

    pub fn set_thread_object(&mut self, thread: JObject) {
        self.object = thread;
    }

    pub fn current_frame_mut(&mut self) -> &mut JvmFrame {
        self.stack.frames.back_mut().unwrap()
    }

    pub fn current_frame(&self) -> Option<&JvmFrame> {
        self.stack.frames.back()
    }

    pub fn operand_stack(&mut self) -> &mut OperandStack {
        &mut self.current_frame_mut().operand_stack
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

    pub fn read_u8(&mut self) -> u8 {
        self.current_frame_mut().code_reader.read_u8().unwrap()
    }

    pub fn read_u16(&mut self) -> u16 {
        self.current_frame_mut().code_reader.read_u16().unwrap()
    }

    pub fn read_i16(&mut self) -> i16 {
        self.current_frame_mut().code_reader.read_i16().unwrap()
    }

    pub fn pc(&self) -> usize {
        self.current_frame().unwrap().code_reader.pc()
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.current_frame_mut().code_reader.set_pc(pc)
    }

    pub fn push(&mut self, val: JValue) {
        self.operand_stack().push(val)
    }

    pub fn push_jint(&mut self, num: JInt) {
        self.operand_stack().push_jint(num)
    }

    pub fn push_jlong(&mut self, num: JLong) {
        self.operand_stack().push_jlong(num)
    }
    pub fn push_jdouble(&mut self, num: JDouble) {
        self.operand_stack().push_jdouble(num)
    }

    pub fn push_jfloat(&mut self, num: JFloat) {
        self.operand_stack().push_jfloat(num)
    }

    pub fn push_jobject(&mut self, v: JObject) {
        self.operand_stack().push_jobject(v)
    }

    pub fn push_jarray(&mut self, v: JArray) {
        self.operand_stack().push_jarray(v)
    }

    pub fn pop(&mut self) -> JValue {
        self.operand_stack().pop()
    }

    pub fn pop_jarray(&mut self) -> JArray {
        self.operand_stack().pop_jarray()
    }

    pub fn pop_jint(&mut self) -> JInt {
        self.operand_stack().pop_jint()
    }

    pub fn pop_jdouble(&mut self) -> JDouble {
        self.operand_stack().pop_jdouble()
    }

    pub fn pop_jlong(&mut self) -> JLong {
        self.operand_stack().pop_jlong()
    }

    pub fn pop_jfloat(&mut self) -> JFloat {
        self.operand_stack().pop_jfloat()
    }

    pub fn pop_jobject(&mut self) -> JObject {
        self.operand_stack().pop_jobject()
    }
}
