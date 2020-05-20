use crate::runtime::class::{Class, InstanceClass};
use crate::runtime::frame::operand_stack::Operand;
use std::fmt;
use std::fmt::Debug;

pub struct JvmHeap {
    mem: Vec<Memory>,
}

const T_BOOLEAN: u8 = 4;
const T_CHAR: u8 = 5;
const T_FLOAT: u8 = 6;
const T_DOUBLE: u8 = 7;
const T_BYTE: u8 = 8;
const T_SHORT: u8 = 9;
const T_INT: u8 = 10;
const T_LONG: u8 = 11;

pub const CLASS_CLASS_NAME: &str = "java/lang/Class";
pub const STRING_CLASS_NAME: &str = "java/lang/String";
pub const OBJECT_CLASS_NAME: &str = "java/lang/Object;";

#[derive(Debug)]
enum Memory {
    Object(Object),
    BooleanArray(Vec<i8>),
    CharArray(Vec<u16>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    ByteArray(Vec<i8>),
    ShortArray(Vec<i16>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
    ReferenceArray {
        class_name: String,
        array: Vec<Operand>,
    },
}

pub struct Object {
    class: InstanceClass,
    fields: Vec<Operand>,
}

impl Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object {{ class: {}}}", self.class.name())
    }
}

impl Object {
    pub fn new_object(class: InstanceClass) -> Self {
        let mut fields = Vec::with_capacity(class.total_instance_fields());
        let all_fields = class.all_instance_fields();
        assert_eq!(all_fields.len(), class.total_instance_fields());
        for field in all_fields {
            fields.push(field.default_value())
        }
        Object { class, fields }
    }

    pub fn class_name(&self) -> &str {
        self.class.name()
    }

    pub fn set_field(&mut self, idx: usize, value: Operand) {
        self.fields[idx] = value;
    }

    pub fn get_field(&self, idx: usize) -> &Operand {
        &self.fields[idx]
    }

    pub fn print_fields(&self) {
        dbg!(&self.fields);
    }
}

impl JvmHeap {
    pub fn new() -> Self {
        JvmHeap {
            mem: Vec::with_capacity(100),
        }
    }

    fn alloc(&mut self, mem: Memory) -> u32 {
        let obj_ref = self.mem.len();
        self.mem.push(mem);
        obj_ref as u32
    }

    pub fn new_object(&mut self, class: Class) -> u32 {
        match class {
            Class::InstanceClass(class) => self.alloc(Memory::Object(Object::new_object(class))),
            _ => unreachable!(),
        }
    }

    pub fn new_empty_array(&mut self, ty: u8, count: i32) -> u32 {
        let m = match ty {
            T_BOOLEAN => Memory::BooleanArray(vec![0; count as usize]),
            T_CHAR => Memory::CharArray(vec![0; count as usize]),
            T_FLOAT => Memory::FloatArray(vec![0f32; count as usize]),
            T_DOUBLE => Memory::DoubleArray(vec![0f64; count as usize]),
            T_BYTE => Memory::ByteArray(vec![0; count as usize]),
            T_SHORT => Memory::ShortArray(vec![0; count as usize]),
            T_INT => Memory::IntArray(vec![0; count as usize]),
            T_LONG => Memory::LongArray(vec![0; count as usize]),
            _ => unreachable!(),
        };
        self.alloc(m)
    }

    pub fn new_char_array(&mut self, data: Vec<u16>) -> u32 {
        self.alloc(Memory::CharArray(data))
    }

    pub fn new_reference_array(&mut self, class_name: String, count: i32) -> u32 {
        let a = Memory::ReferenceArray {
            class_name,
            array: vec![Operand::Null; count as usize],
        };
        self.alloc(a)
    }

    pub fn get_mut_char_array(&mut self, array_ref: Operand) -> &mut Vec<u16> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::CharArray(array) => array,
                _ => unreachable!(),
            },
            v => unreachable!("{:?}", v),
        }
    }

    pub fn get_char_array(&self, array_ref: &Operand) -> &Vec<u16> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &self.mem[*ref_i as usize] {
                Memory::CharArray(array) => array,
                _ => unreachable!(),
            },
            v => unreachable!("{:?}", v),
        }
    }

    pub fn get_int_array_mut(&mut self, array_ref: Operand) -> &mut Vec<i32> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::IntArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_boolean_array_mut(&mut self, array_ref: Operand) -> &mut Vec<i8> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::BooleanArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_float_array_mut(&mut self, array_ref: Operand) -> &mut Vec<f32> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::FloatArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_double_array_mut(&mut self, array_ref: Operand) -> &mut Vec<f64> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::DoubleArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_byte_array_mut(&mut self, array_ref: Operand) -> &mut Vec<i8> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::BooleanArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_long_array_mut(&mut self, array_ref: Operand) -> &mut Vec<i64> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::LongArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn get_short_array_mut(&mut self, array_ref: Operand) -> &mut Vec<i16> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::ShortArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn get_object_array_mut(&mut self, array_ref: &Operand) -> &mut Vec<Operand> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[*ref_i as usize] {
                Memory::ReferenceArray {
                    class_name: _,
                    array,
                } => array,
                _ => unreachable!(),
            },
            v => unreachable!("{:?}", v),
        }
    }

    pub fn get_object_array(&mut self, array_ref: &Operand) -> &Vec<Operand> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &self.mem[*ref_i as usize] {
                Memory::ReferenceArray {
                    class_name: _,
                    array,
                } => array,
                _ => unreachable!(),
            },
            v => unreachable!("{:?}", v),
        }
    }

    pub fn get_array_length(&mut self, array_ref: &Operand) -> i32 {
        (match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[*ref_i as usize] {
                Memory::ShortArray(array) => array.len(),
                Memory::BooleanArray(array) => array.len(),
                Memory::CharArray(array) => array.len(),
                Memory::FloatArray(array) => array.len(),
                Memory::DoubleArray(array) => array.len(),
                Memory::ByteArray(array) => array.len(),
                Memory::IntArray(array) => array.len(),
                Memory::LongArray(array) => array.len(),
                Memory::ReferenceArray {
                    class_name: _,
                    array,
                } => array.len(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }) as i32
    }

    pub fn get_object_mut(&mut self, obj_ref: &Operand) -> &mut Object {
        match obj_ref {
            Operand::ObjectRef(ref_i) => match &mut self.mem[*ref_i as usize] {
                Memory::Object(obj) => obj,
                _ => unreachable!(),
            },
            v => unreachable!("{:?}", v),
        }
    }

    pub fn get_class_name(&self, obj_ref: &Operand) -> String {
        match obj_ref {
            Operand::ObjectRef(ref_i) | Operand::ArrayRef(ref_i) => {
                match &self.mem[*ref_i as usize] {
                    Memory::Object(obj) => obj.class_name().to_string(),
                    Memory::BooleanArray(_) => "[Z".to_string(),
                    Memory::CharArray(_) => "[C".to_string(),
                    Memory::FloatArray(_) => "[F".to_string(),
                    Memory::DoubleArray(_) => "[D".to_string(),
                    Memory::ByteArray(_) => "[B".to_string(),
                    Memory::ShortArray(_) => "[S".to_string(),
                    Memory::IntArray(_) => "[I".to_string(),
                    Memory::LongArray(_) => "[J".to_string(),
                    Memory::ReferenceArray { class_name, .. } => format!("[L{};", class_name),
                }
            }
            v => unreachable!("{:?}", v),
        }
    }

    pub fn get_object(&self, obj_ref: &Operand) -> &Object {
        match obj_ref {
            Operand::ObjectRef(ref_i) => match &self.mem[*ref_i as usize] {
                Memory::Object(obj) => obj,
                Memory::CharArray(array) => {
                    let s = String::from_utf16_lossy(array);
                    unreachable!("{:?}", s);
                }
                v => unreachable!("{:?}", v),
            },
            v => unreachable!("{:?}", v),
        }
    }
}

impl Debug for JvmHeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.mem.iter().enumerate() {
            write!(f, "{}: {:?}, ", i, item)?;
        }
        Ok(())
    }
}
