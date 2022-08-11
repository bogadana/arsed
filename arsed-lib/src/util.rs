use arrayvec::ArrayVec;

pub fn utf16_lit<const N: usize>(string: &str) -> [u16; N] {
    match string
        .encode_utf16()
        .collect::<ArrayVec<u16, N>>()
        .into_inner()
    {
        Ok(bytes) => bytes,
        Err(mut bytes) => {
            while !bytes.is_full() {
                bytes.push(0);
            }
            bytes.into_inner().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_literal() {}
}
