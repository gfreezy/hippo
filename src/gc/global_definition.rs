pub(crate) mod type_to_basic_type;

use crate::class::ClassId;

use crate::gc::global_definition::type_to_basic_type::TypeToBasicType;
use crate::gc::mark_word::MarkWord;
use crate::gc::oop::{ArrayOop, InstanceOop, Oop};

use std::mem::size_of;

pub const HEAP_WORD_SIZE: usize = size_of::<usize>();
pub const BYTES_PER_LONG: usize = size_of::<i64>();
pub const LOG_HEAP_WORD_SIZE: usize = 3;
pub const LOG_BITS_PER_BYTE: usize = 3;
pub const LOG_BYTES_PER_WORD: usize = 3;

pub const HEAP_WORDS_PER_LONG: usize = BYTES_PER_LONG / HEAP_WORD_SIZE;
pub const HEAP_OOP_SIZE: usize = 1;
pub const LOG_BITS_PER_WORD: usize = LOG_BITS_PER_BYTE + LOG_BYTES_PER_WORD;
pub const BITS_PER_WORD: usize = 1 << LOG_BITS_PER_WORD;

#[repr(C)]
pub enum JvmType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}

#[repr(C)]
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum BasicType {
    // The values TBoolean..TLong (4..11) are derived from the JVMS.
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
    // The remaining values are not part of any standard.
    // TObject and TVoid denote two more semantic choices
    // for method return values.
    // TObject and TArray describe signature syntax.
    // TAddress, T_METADATA, TNarrowoop, T_NARROWKLASS describe
    // internal references within the JVM as if they were Java
    // types in their own right.
    Object,
    Array,
}

const T_BOOLEAN: u8 = 4;
const T_CHAR: u8 = 5;
const T_FLOAT: u8 = 6;
const T_DOUBLE: u8 = 7;
const T_BYTE: u8 = 8;
const T_SHORT: u8 = 9;
const T_INT: u8 = 10;
const T_LONG: u8 = 11;

impl BasicType {
    pub fn size_in_bytes(&self) -> usize {
        match self {
            BasicType::Boolean => 1,
            BasicType::Char => 2,
            BasicType::Float => 4,
            BasicType::Double => 8,
            BasicType::Byte => 1,
            BasicType::Short => 2,
            BasicType::Int => 4,
            BasicType::Long => 8,
            BasicType::Object => 8,
            BasicType::Array => 8,
        }
    }

    pub fn default_value(&self) -> JValue {
        match self {
            BasicType::Boolean => JValue::Boolean(0),
            BasicType::Char => JValue::Char(0),
            BasicType::Float => JValue::Float(0.0),
            BasicType::Double => JValue::Double(0.0),
            BasicType::Byte => JValue::Byte(0),
            BasicType::Short => JValue::Short(0),
            BasicType::Int => JValue::Int(0),
            BasicType::Long => JValue::Long(0),
            BasicType::Object => JValue::Object(JObject::null()),
            BasicType::Array => JValue::Object(JObject::null()),
        }
    }

    pub fn descriptor(&self) -> &'static str {
        match self {
            BasicType::Boolean => "Z",
            BasicType::Char => "C",
            BasicType::Float => "F",
            BasicType::Double => "D",
            BasicType::Byte => "B",
            BasicType::Short => "S",
            BasicType::Int => "I",
            BasicType::Long => "J",
            BasicType::Object => "L",
            BasicType::Array => "[",
        }
    }
}

impl From<u8> for BasicType {
    fn from(value: u8) -> Self {
        match value {
            T_BOOLEAN => BasicType::Boolean,
            T_CHAR => BasicType::Char,
            T_FLOAT => BasicType::Float,
            T_DOUBLE => BasicType::Double,
            T_BYTE => BasicType::Byte,
            T_SHORT => BasicType::Short,
            T_INT => BasicType::Int,
            T_LONG => BasicType::Long,
            b'Z' => BasicType::Boolean,
            b'C' => BasicType::Char,
            b'F' => BasicType::Float,
            b'D' => BasicType::Double,
            b'B' => BasicType::Byte,
            b'S' => BasicType::Short,
            b'I' => BasicType::Int,
            b'J' => BasicType::Long,
            b'L' => BasicType::Object,
            v => unreachable!(v),
        }
    }
}

impl From<&str> for BasicType {
    fn from(value: &str) -> Self {
        match value {
            "Z" => BasicType::Boolean,
            "C" => BasicType::Char,
            "F" => BasicType::Float,
            "D" => BasicType::Double,
            "B" => BasicType::Byte,
            "S" => BasicType::Short,
            "I" => BasicType::Int,
            "J" => BasicType::Long,
            ty if ty.starts_with("L") => BasicType::Object,
            ty if ty.starts_with("[") => BasicType::Array,
            v => unreachable!(v),
        }
    }
}

pub type JBoolean = u8;
pub type JByte = i8;
pub type JChar = u16;
pub type JShort = i16;
pub type JFloat = f32;
pub type JDouble = f64;
pub type JInt = i32;
pub type JLong = i64;
pub type JSize = JInt;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JObject(Oop);
impl JObject {
    pub fn new(mut oop: Oop, class_id: ClassId) -> Self {
        oop.class = class_id;
        oop.mark = MarkWord::default();
        JObject(oop)
    }

    pub fn class_id(&self) -> ClassId {
        self.0.class
    }

    pub fn null() -> JObject {
        JObject(Oop::empty())
    }

    pub fn is_null(&self) -> bool {
        self.0.is_empty()
    }

    pub fn set_field_by_offset<T>(&self, offset: usize, v: T)
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let instance_oop: InstanceOop = self.0.into();
        instance_oop.set_field_by_offset(offset, v)
    }

    pub fn set_field_by_jvalue_and_offset(&self, offset: usize, jvalue: JValue) {
        match jvalue {
            JValue::Boolean(v) => self.set_field_by_offset::<JBoolean>(offset, v),
            JValue::Char(v) => self.set_field_by_offset::<JChar>(offset, v),
            JValue::Float(v) => self.set_field_by_offset::<JFloat>(offset, v),
            JValue::Double(v) => self.set_field_by_offset::<JDouble>(offset, v),
            JValue::Byte(v) => self.set_field_by_offset::<JByte>(offset, v),
            JValue::Short(v) => self.set_field_by_offset::<JShort>(offset, v),
            JValue::Int(v) => self.set_field_by_offset::<JInt>(offset, v),
            JValue::Long(v) => self.set_field_by_offset::<JLong>(offset, v),
            JValue::Object(v) => self.set_field_by_offset::<JObject>(offset, v),
        }
    }

    pub fn get_field_by_offset<T>(&self, offset: usize) -> T
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let instance_oop: InstanceOop = self.0.into();
        instance_oop.get_field_by_offset(offset)
    }

    pub fn get_field_by_basic_type_and_offset(&self, ty: BasicType, offset: usize) -> JValue {
        match ty {
            BasicType::Boolean => self.get_field_by_offset::<JBoolean>(offset).into(),
            BasicType::Char => self.get_field_by_offset::<JChar>(offset).into(),
            BasicType::Float => self.get_field_by_offset::<JFloat>(offset).into(),
            BasicType::Double => self.get_field_by_offset::<JDouble>(offset).into(),
            BasicType::Byte => self.get_field_by_offset::<JByte>(offset).into(),
            BasicType::Short => self.get_field_by_offset::<JShort>(offset).into(),
            BasicType::Int => self.get_field_by_offset::<JInt>(offset).into(),
            BasicType::Long => self.get_field_by_offset::<JLong>(offset).into(),
            BasicType::Object => self.get_field_by_offset::<JObject>(offset).into(),
            BasicType::Array => self.get_field_by_offset::<JObject>(offset).into(),
        }
    }

    pub fn hash_code(&self) -> JInt {
        self.0.identity_hash()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JClass(JObject);
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JThrowable(JObject);
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JString(JObject);
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JArray(ArrayOop);

impl JArray {
    pub fn new(mut oop: Oop, class_id: ClassId, len: usize) -> Self {
        oop.class = class_id;
        oop.mark = MarkWord::default();
        let array_oop: ArrayOop = oop.into();
        array_oop.set_len(len);
        JArray(array_oop)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get<T>(&self, i: usize) -> T
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        self.0.element_at(i)
    }

    pub fn set<T>(&self, i: usize, v: T)
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        self.0.set_element_at(i, v)
    }

    pub fn null() -> JArray {
        JArray(ArrayOop::empty())
    }

    pub fn as_slice<T>(&self) -> &[T]
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        self.0.as_slice()
    }

    pub fn as_mut_slice<T>(&self) -> &mut [T]
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        self.0.as_mut_slice()
    }

    pub fn copy_from<T>(&self, src: &[T])
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        self.0.copy_from(src)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<JObject> for JArray {
    fn from(o: JObject) -> Self {
        JArray(o.0.into())
    }
}

impl From<JArray> for JObject {
    fn from(o: JArray) -> Self {
        JObject(o.0.oop())
    }
}

impl From<JArray> for JValue {
    fn from(o: JArray) -> Self {
        JValue::Object(o.into())
    }
}

impl From<JObject> for JValue {
    fn from(o: JObject) -> Self {
        JValue::Object(o)
    }
}

impl From<JBoolean> for JValue {
    fn from(o: JBoolean) -> Self {
        JValue::Boolean(o)
    }
}

impl From<JChar> for JValue {
    fn from(o: JChar) -> Self {
        JValue::Char(o)
    }
}

impl From<JFloat> for JValue {
    fn from(o: JFloat) -> Self {
        JValue::Float(o)
    }
}

impl From<JDouble> for JValue {
    fn from(o: JDouble) -> Self {
        JValue::Double(o)
    }
}

impl From<JByte> for JValue {
    fn from(o: JByte) -> Self {
        JValue::Byte(o)
    }
}

impl From<JShort> for JValue {
    fn from(o: JShort) -> Self {
        JValue::Short(o)
    }
}

impl From<JInt> for JValue {
    fn from(o: JInt) -> Self {
        JValue::Int(o)
    }
}

impl From<JLong> for JValue {
    fn from(o: JLong) -> Self {
        JValue::Long(o)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
#[repr(C)]
pub enum JValue {
    Boolean(JBoolean),
    Char(JChar),
    Float(JFloat),
    Double(JDouble),
    Byte(JByte),
    Short(JShort),
    Int(JInt),
    Long(JLong),
    Object(JObject),
}

impl JValue {
    pub fn as_jfloat(&self) -> JFloat {
        match self {
            JValue::Float(a) => *a,
            _ => unreachable!(),
        }
    }

    pub fn as_jarray(&self) -> JArray {
        match self {
            JValue::Object(a) => (*a).into(),
            _ => unreachable!(),
        }
    }

    pub fn as_jobject(&self) -> JObject {
        match self {
            JValue::Object(a) => *a,
            _ => unreachable!(),
        }
    }

    pub fn as_jbool(&self) -> bool {
        match self {
            JValue::Boolean(b) => *b == 1,
            _ => unreachable!(),
        }
    }

    pub fn as_jdouble(&self) -> JDouble {
        match self {
            JValue::Double(a) => *a,
            _ => unreachable!(),
        }
    }

    pub fn as_jint(&self) -> JInt {
        match self {
            JValue::Int(a) => *a,
            _ => unreachable!(),
        }
    }

    pub fn as_jlong(&self) -> JLong {
        match self {
            JValue::Long(a) => *a,
            _ => unreachable!(),
        }
    }

    pub fn is_null(&self) -> bool {
        assert!(self.is_reference_type());
        match self {
            JValue::Object(o) => o.is_null(),
            _ => unreachable!(),
        }
    }

    pub fn is_reference_type(&self) -> bool {
        matches!(self, JValue::Object(_))
    }

    pub fn class_id(&self) -> ClassId {
        assert!(self.is_reference_type());
        match self {
            JValue::Object(o) => o.class_id(),
            _ => unreachable!(),
        }
    }

    pub fn is_category1(&self) -> bool {
        !self.is_category2()
    }

    pub fn is_category2(&self) -> bool {
        matches!(self, JValue::Double(_) | JValue::Long(_))
    }
}

impl Default for JValue {
    fn default() -> Self {
        JValue::Boolean(0)
    }
}
