use crate::gc::oop::Oop;

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
    Oop(Oop),
    Null,
}

pub type JvmPC = usize;
