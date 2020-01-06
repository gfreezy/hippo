use nom::error::ParseError;
use nom::multi::many_m_n;
use nom::{IResult, InputLength, InputTake, ToUsize};

pub fn length_many<I, O, N, E, F, G>(f: F, g: G) -> impl Fn(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + InputLength + InputTake + PartialEq,
    N: Copy + ToUsize,
    F: Fn(I) -> IResult<I, N, E>,
    G: Fn(I) -> IResult<I, O, E> + Copy,
    E: ParseError<I>,
{
    move |buf| {
        let (buf, num) = f(buf)?;
        many_m_n(num.to_usize(), num.to_usize(), g)(buf)
    }
}
