use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::multi::{many0, many1};
use nom::IResult;

fn reference_type(buf: &str) -> IResult<&str, String> {
    let (buf, _) = tag("L")(buf)?;
    let (buf, class_name) = many1(none_of(";["))(buf)?;
    let (buf, _) = tag(";")(buf)?;
    Ok((
        buf,
        format!("L{};", class_name.into_iter().collect::<String>()),
    ))
}

fn base_type(buf: &str) -> IResult<&str, String> {
    let (buf, d) = alt((
        tag("B"),
        tag("C"),
        tag("D"),
        tag("F"),
        tag("I"),
        tag("J"),
        tag("S"),
        tag("Z"),
    ))(buf)?;
    Ok((buf, d.to_string()))
}

fn array_type(buf: &str) -> IResult<&str, String> {
    let (buf, array) = many1(tag("["))(buf)?;
    let (buf, ty) = alt((base_type, reference_type))(buf)?;
    Ok((buf, format!("{}{}", array.join(""), ty)))
}

pub fn field_descriptor(buf: &str) -> IResult<&str, String> {
    alt((base_type, reference_type, array_type))(buf)
}

fn void_descriptor(buf: &str) -> IResult<&str, String> {
    let (buf, v) = tag("V")(buf)?;
    Ok((buf, v.to_string()))
}

pub fn method_descriptor(buf: &str) -> IResult<&str, (Vec<String>, String)> {
    let (buf, _) = tag("(")(buf)?;
    let (buf, parameters) = many0(field_descriptor)(buf)?;
    let (buf, _) = tag(")")(buf)?;
    let (buf, return_descriptor) = alt((void_descriptor, field_descriptor))(buf)?;
    Ok((buf, (parameters, return_descriptor)))
}

#[cfg(test)]
mod tests {
    use super::{field_descriptor, method_descriptor};

    #[test]
    fn test_descriptor() {
        let (_, field) = field_descriptor("I").unwrap();
        assert_eq!(field, "I");
        let (_, field) = field_descriptor("Ljava/lang/Thread;").unwrap();
        assert_eq!(field, "java/lang/Thread");
        let (_, method) = method_descriptor("(IDLjava/lang/Thread;)Ljava/lang/Object;").unwrap();
        assert_eq!(
            method,
            (
                vec![
                    "I".to_string(),
                    "D".to_string(),
                    "java/lang/Thread".to_string()
                ],
                "java/lang/Object".to_string()
            )
        );
        let (_, method) =
            method_descriptor("(IDLjava/lang/Thread;[[[Ljava/lang/Thread;)Ljava/lang/Object;")
                .unwrap();
        assert_eq!(
            method,
            (
                vec![
                    "I".to_string(),
                    "D".to_string(),
                    "java/lang/Thread".to_string(),
                    "[[[java/lang/Thread".to_string(),
                ],
                "java/lang/Object".to_string()
            )
        )
    }
}
