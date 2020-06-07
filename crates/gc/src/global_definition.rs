pub(crate) mod type_to_basic_type;

use crate::address::Address;
use std::marker::PhantomData;
use std::mem::size_of;

pub const HEAP_WORD_SIZE: usize = size_of::<usize>();
pub const BYTES_PER_LONG: usize = size_of::<i64>();
pub const LOG_HEAP_WORD_SIZE: usize = 3;
pub const HEAP_WORDS_PER_LONG: usize = BYTES_PER_LONG / HEAP_WORD_SIZE;
pub const HEAP_OOP_SIZE: usize = 1;

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
    Void,
    Address,
    NarrowOop,
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

pub struct JObject(Address);
pub struct JClass(JObject);
pub struct JThrowable(JObject);
pub struct JString(JObject);
pub struct JArray(JObject);
pub struct JBooleanArray(JObject);
pub struct JByteArray(JObject);
pub struct JCharArray(JObject);
pub struct JShortArray(JObject);
pub struct JIntArray(JObject);
pub struct JLongArray(JObject);
pub struct JFloatArray(JObject);
pub struct JDoubleArray(JObject);
pub struct JObjectArray(JObject);
