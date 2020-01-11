use crate::class_parser::attribute_info::{parse_attribute_info, AttributeInfo};
use crate::class_parser::constant_pool::ConstPoolInfo;
use crate::nom_utils::length_many;
use nom::multi::many_m_n;
use nom::number::complete::be_u16;
use nom::IResult;

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

pub fn parse_field_info(buf: &[u8]) -> IResult<&[u8], FieldInfo> {
    let (left, access_flags) = be_u16(buf)?;
    let (left, name_index) = be_u16(left)?;
    let (left, descriptor_index) = be_u16(left)?;
    let (left, attributes) = length_many(be_u16, parse_attribute_info)(left)?;
    Ok((
        left,
        FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        },
    ))
}
