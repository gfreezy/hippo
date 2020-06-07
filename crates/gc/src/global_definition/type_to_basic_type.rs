use crate::global_definition::{
    BasicType, JBoolean, JByte, JChar, JDouble, JFloat, JInt, JLong, JShort,
};
use std::marker::PhantomData;

pub struct TypeToBasicType<T> {
    phantom: PhantomData<T>,
}

pub fn type_to_basic_type<T>() -> BasicType
where
    TypeToBasicType<T>: Into<BasicType>,
{
    TypeToBasicType {
        phantom: PhantomData::<T>,
    }
    .into()
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
