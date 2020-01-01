use cesu8::from_java_cesu8;
use enum_methods::EnumIsA;
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

#[derive(Debug, EnumIsA)]
pub(crate) enum ConstPoolInfo {
    ConstantClassInfo {
        name_index: u16,
    },
    ConstantFieldrefInfo {
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

impl ConstPoolInfo {}

pub(crate) fn parse_const_pool_info(buf: &[u8]) -> IResult<&[u8], ConstPoolInfo> {
    use ConstPoolInfo::*;

    let (left, pool_tag) = be_u8(buf)?;
    match pool_tag {
        CONSTANT_CLASS => {
            let (left, name_index) = be_u16(left)?;
            Ok((left, ConstantClassInfo { name_index }))
        }
        CONSTANT_FIELDREF | CONSTANT_METHODREF | CONSTANT_INTERFACE_METHODREF => {
            let (left, class_index) = be_u16(left)?;
            let (left, name_and_type_index) = be_u16(left)?;
            Ok((
                left,
                ConstantFieldrefInfo {
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

#[cfg(test)]
mod tests {
    use crate::class_parser::constant_pool::parse_const_pool_info;
    use insta::assert_debug_snapshot;
    use nom::multi::many_m_n;

    #[test]
    fn test_parse_const_pool_info() {
        let buf = [
            10, 0, 6, 0, 15, 9, 0, 16, 0, 17, 8, 0, 18, 10, 0, 19, 0, 20, 7, 0, 21, 7, 0, 22, 1, 0,
            6, 60, 105, 110, 105, 116, 62, 1, 0, 3, 40, 41, 86, 1, 0, 4, 67, 111, 100, 101, 1, 0,
            15, 76, 105, 110, 101, 78, 117, 109, 98, 101, 114, 84, 97, 98, 108, 101, 1, 0, 4, 109,
            97, 105, 110, 1, 0, 22, 40, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83,
            116, 114, 105, 110, 103, 59, 41, 86, 1, 0, 10, 83, 111, 117, 114, 99, 101, 70, 105,
            108, 101, 1, 0, 9, 77, 97, 105, 110, 46, 106, 97, 118, 97, 12, 0, 7, 0, 8, 7, 0, 23,
            12, 0, 24, 0, 25, 1, 0, 5, 72, 101, 108, 108, 111, 7, 0, 26, 12, 0, 27, 0, 28, 1, 0, 9,
            77, 97, 105, 110, 47, 77, 97, 105, 110, 1, 0, 16, 106, 97, 118, 97, 47, 108, 97, 110,
            103, 47, 79, 98, 106, 101, 99, 116, 1, 0, 16, 106, 97, 118, 97, 47, 108, 97, 110, 103,
            47, 83, 121, 115, 116, 101, 109, 1, 0, 3, 111, 117, 116, 1, 0, 21, 76, 106, 97, 118,
            97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101, 97, 109, 59, 1, 0, 19,
            106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101, 97, 109,
            1, 0, 7, 112, 114, 105, 110, 116, 108, 110, 1, 0, 21, 40, 76, 106, 97, 118, 97, 47,
            108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 41, 86,
        ];
        let parser = many_m_n(28, 28, parse_const_pool_info);
        assert_debug_snapshot!(parser(&buf).expect("parse const pool"));
    }
}
