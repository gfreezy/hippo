use crate::gc::global_definition::{
    BasicType, JArray, JBoolean, JByte, JChar, JDouble, JFloat, JInt, JLong, JObject, JShort,
    JValue,
};

pub struct TypeToBasicType<T> {
    v: Option<T>,
}

pub fn type_to_basic_type<T>(v: Option<T>) -> BasicType
where
    TypeToBasicType<T>: Into<BasicType>,
{
    TypeToBasicType { v }.into()
}

pub fn size_of_java_type<T>(v: Option<T>) -> usize
where
    TypeToBasicType<T>: Into<BasicType>,
{
    type_to_basic_type(v).size_in_bytes()
}

impl From<TypeToBasicType<JBoolean>> for BasicType {
    fn from(_: TypeToBasicType<JBoolean>) -> Self {
        BasicType::Boolean
    }
}

impl From<TypeToBasicType<JByte>> for BasicType {
    fn from(_: TypeToBasicType<JByte>) -> Self {
        BasicType::Byte
    }
}

impl From<TypeToBasicType<JChar>> for BasicType {
    fn from(_: TypeToBasicType<JChar>) -> Self {
        BasicType::Char
    }
}

impl From<TypeToBasicType<JShort>> for BasicType {
    fn from(_: TypeToBasicType<JShort>) -> Self {
        BasicType::Short
    }
}
impl From<TypeToBasicType<JInt>> for BasicType {
    fn from(_: TypeToBasicType<JInt>) -> Self {
        BasicType::Int
    }
}

impl From<TypeToBasicType<JLong>> for BasicType {
    fn from(_: TypeToBasicType<JLong>) -> Self {
        BasicType::Long
    }
}

impl From<TypeToBasicType<JFloat>> for BasicType {
    fn from(_: TypeToBasicType<JFloat>) -> Self {
        BasicType::Float
    }
}

impl From<TypeToBasicType<JDouble>> for BasicType {
    fn from(_: TypeToBasicType<JDouble>) -> Self {
        BasicType::Double
    }
}

impl From<TypeToBasicType<JObject>> for BasicType {
    fn from(_: TypeToBasicType<JObject>) -> Self {
        BasicType::Object
    }
}

impl From<TypeToBasicType<JArray>> for BasicType {
    fn from(_: TypeToBasicType<JArray>) -> Self {
        BasicType::Array
    }
}

impl From<TypeToBasicType<JValue>> for BasicType {
    fn from(v: TypeToBasicType<JValue>) -> Self {
        match v.v {
            Some(JValue::Boolean(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Char(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Float(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Double(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Byte(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Short(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Int(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Long(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Object(inner)) => type_to_basic_type(Some(inner)),
            Some(JValue::Array(inner)) => type_to_basic_type(Some(inner)),
            None => unreachable!(),
        }
    }
}
