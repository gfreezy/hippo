#[macro_use]
pub mod predefined_attribute;

use crate::class_parser::attribute_info::predefined_attribute::{
    parse_predefined_attribute, PredefinedAttribute,
};
use crate::class_parser::constant_pool::ConstPoolInfo;
use nom::number::complete::{be_u16, be_u32};
use nom::IResult;

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute: PredefinedAttribute,
}

pub fn parse_attribute_info<'a>(
    const_pools: &Vec<ConstPoolInfo>,
    buf: &'a [u8],
) -> IResult<&'a [u8], AttributeInfo> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, _length) = be_u32(buf)?;
    let attr_name = const_pools[attribute_name_index as usize - 1].as_constant_utf8_info();
    let (buf, attr) = parse_predefined_attribute(attr_name, const_pools, buf)?;
    Ok((
        buf,
        AttributeInfo {
            attribute_name_index,
            attribute: attr,
        },
    ))
}
