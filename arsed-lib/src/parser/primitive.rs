use nom::{bytes::complete::take, IResult};

pub fn u8_bool(i: &[u8]) -> IResult<&[u8], bool> {
    let (i, bool) = take(1usize)(i)?;

    Ok((i, bool[0] != 0))
}