mod predefined_attribute;
use nom::eof;
use nom::multi::length_data;
use nom::number::complete::{be_u16, be_u32};
use nom::IResult;

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub info: Vec<u8>,
}

pub fn parse_attribute_info(buf: &[u8]) -> IResult<&[u8], AttributeInfo> {
    let (left, attribute_name_index) = be_u16(buf)?;
    let (left, info) = length_data(be_u32)(left)?;
    Ok((
        left,
        AttributeInfo {
            attribute_name_index,
            info: info.to_vec(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::class_parser::attribute_info::parse_attribute_info;
    use insta::assert_debug_snapshot;
    use nom::multi::many_m_n;

    #[test]
    fn test_parse_attribute_info() {
        let buf = [0, 13, 0, 0, 0, 2, 0, 14];
        let parser = many_m_n(1, 1, parse_attribute_info);
        assert_debug_snapshot!(parser(&buf).unwrap());
    }
}
