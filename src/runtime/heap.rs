use crate::runtime::class::Class;
use crate::runtime::frame::operand_stack::Operand;
use crate::runtime::jvm_env::JvmEnv;
use std::collections::HashMap;

#[derive(Debug)]
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

const CLASS_CLASS_NAME: &str = "java/lang/Class";
pub const STRING_CLASS_NAME: &str = "java/lang/String";

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
    ReferenceArray { class_name: String, array: Vec<u32> },
}

#[derive(Debug)]
pub enum Object {
    Class {
        class_name: String,
    },
    Object {
        class: Class,
        fields: HashMap<String, Operand>,
    },
}

impl Object {
    pub fn new_object(class: Class) -> Self {
        Object::Object {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn new_class(class_name: String) -> Self {
        Object::Class { class_name }
    }

    pub fn class_name(&self) -> &str {
        match self {
            Object::Class { .. } => CLASS_CLASS_NAME,
            Object::Object { class, .. } => class.name(),
        }
    }

    pub fn set_field(&mut self, field_name: String, value: Operand) {
        match self {
            Object::Class { .. } => unreachable!(),
            Object::Object { fields, .. } => fields.insert(field_name, value),
        };
    }

    pub fn get_field(&self, field_name: &str) -> Option<&Operand> {
        match self {
            Object::Object { fields, .. } => fields.get(field_name),
            Object::Class { .. } => Some(&Operand::Null),
        }
    }
}

impl JvmHeap {
    pub fn new() -> Self {
        JvmHeap {
            mem: Vec::with_capacity(100),
        }
    }

    pub fn new_class_object(&mut self, class_name: String) -> u32 {
        let obj_ref = self.mem.len();
        self.mem.push(Memory::Object(Object::new_class(class_name)));
        obj_ref as u32
    }

    pub fn new_object(&mut self, class: Class) -> u32 {
        let obj_ref = self.mem.len();
        self.mem.push(Memory::Object(Object::new_object(class)));
        obj_ref as u32
    }

    pub fn new_java_string(&mut self, s: &str, jenv: &mut JvmEnv) -> u32 {
        let bytes_str = s.as_bytes();
        let array = self.new_array(T_CHAR, bytes_str.len() as i32);
        let mut fields = HashMap::new();
        fields.insert("value".to_string(), Operand::ArrayRef(array));

        let class = jenv.load_and_init_class(STRING_CLASS_NAME);
        let obj = Object::new_object(class);
        let obj_ref = self.mem.len();
        self.mem.push(Memory::Object(obj));
        obj_ref as u32
    }

    pub fn new_array(&mut self, ty: u8, count: i32) -> u32 {
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
        let array_ref = self.mem.len();
        self.mem.push(m);
        array_ref as u32
    }

    pub fn new_byte_array(&mut self, count: i32) -> u32 {
        self.new_array(T_BYTE, count)
    }

    pub fn new_reference_array(&mut self, class_name: String, count: i32) -> u32 {
        let a = Memory::ReferenceArray {
            class_name,
            array: vec![0; count as usize],
        };
        let array_ref = self.mem.len();
        self.mem.push(a);
        array_ref as u32
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

    pub fn get_mut_int_array(&mut self, array_ref: Operand) -> &mut Vec<i32> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::IntArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_mut_boolean_array(&mut self, array_ref: Operand) -> &mut Vec<i8> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::BooleanArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_mut_float_array(&mut self, array_ref: Operand) -> &mut Vec<f32> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::FloatArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_mut_double_array(&mut self, array_ref: Operand) -> &mut Vec<f64> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::DoubleArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_mut_byte_array(&mut self, array_ref: Operand) -> &mut Vec<i8> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::BooleanArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn get_mut_long_array(&mut self, array_ref: Operand) -> &mut Vec<i64> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::LongArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn get_mut_short_array(&mut self, array_ref: Operand) -> &mut Vec<i16> {
        match array_ref {
            Operand::ArrayRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::ShortArray(array) => array,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn get_mut_object_array(&mut self, array_ref: &Operand) -> &mut Vec<u32> {
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

    pub fn get_object_array(&mut self, array_ref: &Operand) -> &Vec<u32> {
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

    pub fn get_mut_object(&mut self, obj_ref: Operand) -> &mut Object {
        match obj_ref {
            Operand::ObjectRef(ref_i) => match &mut self.mem[ref_i as usize] {
                Memory::Object(obj) => obj,
                _ => unreachable!(),
            },
            v => unreachable!("{:?}", v),
        }
    }

    pub fn get_class_name(&self, obj_ref: &Operand) -> &str {
        match obj_ref {
            Operand::ObjectRef(ref_i) => match &self.mem[*ref_i as usize] {
                Memory::Object(obj) => obj.class_name(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn get_object(&self, obj_ref: &Operand) -> &Object {
        match obj_ref {
            Operand::ObjectRef(ref_i) => match &self.mem[*ref_i as usize] {
                Memory::Object(obj) => obj,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn get_string(&self, str_ref: &Operand) -> String {
        let string_operand = self.get_object(str_ref);
        let chars_ref = string_operand.get_field("value").unwrap();
        String::from_utf16(self.get_char_array(chars_ref)).unwrap()
    }
}
