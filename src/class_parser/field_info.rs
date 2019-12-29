use crate::class_parser::attribute_info::{parse_attribute_info, AttributeInfo};
use nom::multi::{length_data, many_m_n};
use nom::number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, be_u32, be_u8};
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
    let (left, attributes_count) = be_u16(left)?;
    let (left, attributes) = many_m_n(
        attributes_count as usize,
        attributes_count as usize,
        parse_attribute_info,
    )(left)?;
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
