use cesu8::from_java_cesu8;
use enum_methods::{EnumAsGetters, EnumIsA};
use nom::multi::length_data;
use nom::number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, be_u8};
use nom::IResult;

const CONSTANT_CLASS: u8 = 7;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_INTERFACE_METHODREF: u8 = 11;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_FLOAT: u8 = 4;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_NAME_AND_TYPE: u8 = 12;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_METHOD_HANDLE: u8 = 15;
const CONSTANT_METHOD_TYPE: u8 = 16;
const CONSTANT_INVOKE_DYNAMIC: u8 = 18;

#[derive(Debug)]
pub struct ConstPool {
    infos: Vec<ConstPoolInfo>,
}

impl ConstPool {
    pub fn new(const_pool_infos: Vec<ConstPoolInfo>) -> Self {
        ConstPool {
            infos: const_pool_infos,
        }
    }

    pub fn get_const_pool_info_at(&self, index: u16) -> &ConstPoolInfo {
        &self.infos[index as usize - 1]
    }

    pub fn get_utf8_string_at(&self, index: u16) -> &String {
        self.get_const_pool_info_at(index).as_constant_utf8_info()
    }

    pub fn is_class_at(&self, index: u16) -> bool {
        self.get_const_pool_info_at(index).is_constant_class_info()
    }

    pub fn get_class_name_at(&self, index: u16) -> &String {
        match self.get_const_pool_info_at(index) {
            ConstPoolInfo::ConstantClassInfo { name_index } => self.get_utf8_string_at(*name_index),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, EnumIsA, EnumAsGetters)]
pub enum ConstPoolInfo {
    ConstantClassInfo {
        name_index: u16,
    },
    ConstantFieldRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantMethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantInterfaceMethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantStringInfo {
        string_index: u16,
    },
    ConstantIntegerInfo(i32),
    ConstantFloatInfo(f32),
    ConstantLongInfo(i64),
    ConstantDoubleInfo(f64),
    ConstantNameAndTypeInfo {
        name_index: u16,
        descriptor_index: u16,
    },
    ConstantUtf8Info(String),
    ConstantMethodHandleInfo {
        reference_kind: u8,
        reference_index: u16,
    },
    ConstantMethodTypeInfo {
        descriptor_index: u16,
    },
    ConstantInvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

pub(crate) fn parse_const_pool_info(buf: &[u8]) -> IResult<&[u8], ConstPoolInfo> {
    use ConstPoolInfo::*;

    let (left, pool_tag) = be_u8(buf)?;
    match pool_tag {
        CONSTANT_CLASS => {
            let (left, name_index) = be_u16(left)?;
            Ok((left, ConstantClassInfo { name_index }))
        }
        CONSTANT_FIELDREF => {
            let (left, class_index) = be_u16(left)?;
            let (left, name_and_type_index) = be_u16(left)?;
            Ok((
                left,
                ConstantFieldRefInfo {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        CONSTANT_METHODREF => {
            let (left, class_index) = be_u16(left)?;
            let (left, name_and_type_index) = be_u16(left)?;
            Ok((
                left,
                ConstantMethodRefInfo {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        CONSTANT_INTERFACE_METHODREF => {
            let (left, class_index) = be_u16(left)?;
            let (left, name_and_type_index) = be_u16(left)?;
            Ok((
                left,
                ConstantInterfaceMethodRefInfo {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        CONSTANT_STRING => {
            let (left, string_index) = be_u16(left)?;
            Ok((left, ConstantStringInfo { string_index }))
        }
        CONSTANT_INTEGER => {
            let (left, i) = be_i32(left)?;
            Ok((left, ConstantIntegerInfo(i)))
        }
        CONSTANT_FLOAT => {
            let (left, f) = be_f32(left)?;
            Ok((left, ConstantFloatInfo(f)))
        }
        CONSTANT_LONG => {
            let (left, i) = be_i64(left)?;
            Ok((left, ConstantLongInfo(i)))
        }
        CONSTANT_DOUBLE => {
            let (left, i) = be_f64(left)?;
            Ok((left, ConstantDoubleInfo(i)))
        }
        CONSTANT_NAME_AND_TYPE => {
            let (left, name_index) = be_u16(left)?;
            let (left, descriptor_index) = be_u16(left)?;
            Ok((
                left,
                ConstantNameAndTypeInfo {
                    name_index,
                    descriptor_index,
                },
            ))
        }
        CONSTANT_UTF8 => {
            let (left, bytes) = length_data(be_u16)(left)?;
            let s = from_java_cesu8(bytes).expect("invalid utf-8 string");
            Ok((left, ConstantUtf8Info(s.to_string())))
        }
        CONSTANT_METHOD_HANDLE => {
            let (left, reference_kind) = be_u8(left)?;
            let (left, reference_index) = be_u16(left)?;
            Ok((
                left,
                ConstantMethodHandleInfo {
                    reference_kind,
                    reference_index,
                },
            ))
        }
        CONSTANT_METHOD_TYPE => {
            let (left, descriptor_index) = be_u16(left)?;
            Ok((left, ConstantMethodTypeInfo { descriptor_index }))
        }
        CONSTANT_INVOKE_DYNAMIC => {
            let (left, bootstrap_method_attr_index) = be_u16(left)?;
            let (left, name_and_type_index) = be_u16(left)?;
            Ok((
                left,
                ConstantInvokeDynamicInfo {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
            ))
        }
        _ => unreachable!(),
    }
}
