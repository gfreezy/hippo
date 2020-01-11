use crate::class_parser::attribute_info::{parse_attribute_info, AttributeInfo};
use nom::multi::many_m_n;
use nom::number::complete::be_u16;
use nom::IResult;

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

pub fn parse_method_info(buf: &[u8]) -> IResult<&[u8], MethodInfo> {
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
        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use nom::multi::many_m_n;

    #[test]
    fn test_parse_method_info() {
        let buf = [
            0, 1, 0, 7, 0, 8, 0, 1, 0, 9, 0, 0, 0, 29, 0, 1, 0, 1, 0, 0, 0, 5, 42, 183, 0, 1, 177,
            0, 0, 0, 1, 0, 10, 0, 0, 0, 6, 0, 1, 0, 0, 0, 3, 0, 9, 0, 11, 0, 12, 0, 1, 0, 9, 0, 0,
            0, 37, 0, 2, 0, 1, 0, 0, 0, 9, 178, 0, 2, 18, 3, 182, 0, 4, 177, 0, 0, 0, 1, 0, 10, 0,
            0, 0, 10, 0, 2, 0, 0, 0, 5, 0, 8, 0, 6,
        ];
        let parser = many_m_n(2, 2, parse_method_info);
        assert_debug_snapshot!(parser(&buf).unwrap());
    }
}
