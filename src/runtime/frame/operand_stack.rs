#[derive(Debug, Clone)]
pub enum Operand {
    Int(i32),
    Float(f32),
    Double(f64),
    Long(i64),
    Str(u16),
    ObjectRef(u32),
    ArrayRef(u32),
    ClassRef(String),
    Null,
}

#[derive(Debug)]
pub struct OperandStack {
    stack: Vec<Operand>,
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

    pub fn push(&mut self, val: Operand) {
        self.stack.push(val)
    }

    pub fn push_integer(&mut self, num: i32) {
        self.push(Operand::Int(num))
    }

    pub fn push_float(&mut self, num: f32) {
        self.push(Operand::Float(num))
    }

    pub fn push_object_ref(&mut self, reference: u32) {
        self.push(Operand::ObjectRef(reference))
    }

    pub fn push_class_ref(&mut self, class: String) {
        self.push(Operand::ClassRef(class))
    }

    pub fn pop(&mut self) -> Operand {
        self.stack.pop().unwrap()
    }

    pub fn pop_integer(&mut self) -> i32 {
        match self.stack.pop() {
            Some(Operand::Int(num)) => num,
            _ => unreachable!(),
        }
    }

    pub fn pop_float(&mut self) -> f32 {
        match self.stack.pop() {
            Some(Operand::Float(num)) => num,
            _ => unreachable!(),
        }
    }

    pub fn pop_object_ref(&mut self) -> u32 {
        match self.stack.pop() {
            Some(Operand::ObjectRef(num)) => num,
            _ => unreachable!(),
        }
    }

    pub fn pop_array_ref(&mut self) -> u32 {
        match self.stack.pop() {
            Some(Operand::ArrayRef(num)) => num,
            _ => unreachable!(),
        }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
