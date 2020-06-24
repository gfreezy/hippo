use crate::gc::global_definition::{JArray, JDouble, JFloat, JInt, JLong, JObject, JValue};

#[derive(Debug)]
pub struct OperandStack {
    stack: Vec<JValue>,
}

impl OperandStack {
    pub fn new() -> Self {
        OperandStack { stack: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        OperandStack {
            stack: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, val: JValue) {
        self.stack.push(val)
    }

    pub fn push_jint(&mut self, num: JInt) {
        self.push(JValue::Int(num))
    }

    pub fn push_jlong(&mut self, num: JLong) {
        self.push(JValue::Long(num))
    }
    pub fn push_jdouble(&mut self, num: JDouble) {
        self.push(JValue::Double(num))
    }

    pub fn push_jfloat(&mut self, num: JFloat) {
        self.push(JValue::Float(num))
    }

    pub fn push_jobject(&mut self, v: JObject) {
        self.push(JValue::Object(v))
    }

    pub fn push_jarray(&mut self, v: JArray) {
        self.push(JValue::Array(v))
    }

    pub fn pop(&mut self) -> JValue {
        self.stack.pop().unwrap()
    }

    pub fn pop_jarray(&mut self) -> JArray {
        self.pop().as_jarray()
    }

    pub fn pop_jint(&mut self) -> JInt {
        match self.stack.pop() {
            Some(JValue::Int(num)) => num,
            v => unreachable!("{:?}", v),
        }
    }

    pub fn pop_jdouble(&mut self) -> JDouble {
        match self.stack.pop() {
            Some(JValue::Double(num)) => num,
            v => unreachable!("{:?}", v),
        }
    }

    pub fn pop_jlong(&mut self) -> JLong {
        match self.stack.pop() {
            Some(JValue::Long(num)) => num,
            v => unreachable!("{:?}", v),
        }
    }

    pub fn pop_jfloat(&mut self) -> JFloat {
        match self.stack.pop() {
            Some(JValue::Float(num)) => num,
            v => unreachable!("{:?}", v),
        }
    }

    pub fn pop_jobject(&mut self) -> JObject {
        match self.stack.pop() {
            Some(JValue::Object(o)) => o,
            v => unreachable!("{:?}", v),
        }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
