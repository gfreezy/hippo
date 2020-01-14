#[macro_use]
pub mod predefined_attribute;

use crate::class_parser::attribute_info::predefined_attribute::{
    parse_predefined_attribute, PredefinedAttribute,
};
use crate::class_parser::constant_pool::ConstPool;
use nom::number::complete::{be_u16, be_u32};
use nom::IResult;

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute: PredefinedAttribute,
}

pub fn parse_attribute_info<'a>(
    const_pool: &ConstPool,
    buf: &'a [u8],
) -> IResult<&'a [u8], AttributeInfo> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, _length) = be_u32(buf)?;
    let attr_name = const_pool.get_utf8_string_at(attribute_name_index);
    let (buf, attr) = parse_predefined_attribute(attr_name, const_pool, buf)?;
    Ok((
        buf,
        AttributeInfo {
            attribute_name_index,
            attribute: attr,
        },
    ))
}
