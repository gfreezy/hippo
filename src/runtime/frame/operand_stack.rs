#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Byte(i8),
    Short(i16),
    Char(u16),
    Int(i32),
    Float(f32),
    Double(f64),
    Long(i64),
    Str(u16),
    ObjectRef(u32),
    ArrayRef(u32),
    Null,
}

impl Operand {
    pub fn get_float(&self) -> f32 {
        match self {
            Operand::Float(n) => *n,
            _ => unreachable!(),
        }
    }
    pub fn get_double(&self) -> f64 {
        match self {
            Operand::Double(n) => *n,
            _ => unreachable!(),
        }
    }
    pub fn get_long(&self) -> i64 {
        match self {
            Operand::Long(n) => *n,
            _ => unreachable!(),
        }
    }

    pub fn get_int(&self) -> i32 {
        match self {
            Operand::Int(n) => *n,
            _ => unreachable!(),
        }
    }

    pub fn hash_code(&self) -> i32 {
        match self {
            Operand::ObjectRef(i) | Operand::ArrayRef(i) => *i as i32,
            _ => unreachable!(),
        }
    }
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

    pub fn push_long(&mut self, num: i64) {
        self.push(Operand::Long(num))
    }
    pub fn push_double(&mut self, num: f64) {
        self.push(Operand::Double(num))
    }

    pub fn push_float(&mut self, num: f32) {
        self.push(Operand::Float(num))
    }

    pub fn push_object_ref(&mut self, reference: u32) {
        self.push(Operand::ObjectRef(reference))
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

    pub fn pop_double(&mut self) -> f64 {
        match self.stack.pop() {
            Some(Operand::Double(num)) => num,
            _ => unreachable!(),
        }
    }

    pub fn pop_long(&mut self) -> i64 {
        match self.stack.pop() {
            Some(Operand::Long(num)) => num,
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
            Some(Operand::ArrayRef(num)) => num,
            _ => unreachable!(),
        }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
