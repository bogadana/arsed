mod headers;

use nom::IResult;
use nom_derive::Parse;

use self::headers::{ChunkType, ExtendedHeader, ResChunkHeader, ResStringPoolHeader};

fn extended_header(i: &[u8]) -> IResult<&[u8], Box<dyn ExtendedHeader>> {
    let (i, base_header) = ResChunkHeader::parse(i)?;

    match base_header.res_type {
        ChunkType::ResStringPoolType => {
            let (i, header) = ResStringPoolHeader::parse(i)?;
            Ok((i, Box::new(header)))
        }
        _ => panic!("unsupported chunk type"),
    }
}

#[cfg(test)]
mod tests {
    use nom_derive::Parse;

    use crate::util::utf16_lit;

    use super::{headers::{ResPackageHeader, ResTypeSpecHeader}, *};

    #[test]
    fn parse_res_header() {
        let test_header = b"\x02\x00\x0C\x00\x40\x38\x0A\x00";

        let (rest, result) = ResChunkHeader::parse(test_header).unwrap();

        let expected = ResChunkHeader {
            res_type: ChunkType::ResTableType,
            header_size: 0xc,
            size: 0xa3840,
        };

        assert_eq!(result, expected);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn parse_string_pool_header() {
        let test_header = b"\x01\x00\x1C\x00\x38\x55\x03\x00\x99\x16\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x80\x5A\x00\x00\x00\x00\x00\x00";

        let (rest, result) = ResStringPoolHeader::parse(test_header).unwrap();

        let expected_base = ResChunkHeader {
            res_type: ChunkType::ResStringPoolType,
            header_size: 0x1c,
            size: 0x35538,
        };

        let expected = ResStringPoolHeader {
            base_header: expected_base,
            string_count: 0x1699,
            style_count: 0,
            flags: 0x100,
            strings_start: 0x5a80,
            styles_start: 0,
        };

        assert_eq!(result, expected);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn parse_package_header() {
        let test_header = b"\x00\x02\x20\x01\xFC\xE2\x06\x00\x7F\x00\x00\x00\x63\x00\x6F\x00\x6D\x00\x2E\x00\x62\x00\x6F\x00\x67\x00\x61\x00\x64\x00\x61\x00\x6E\x00\x61\x00\x2E\x00\x72\x00\x65\x00\x73\x00\x74\x00\x65\x00\x73\x00\x74\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x20\x01\x00\x00\x00\x00\x00\x00\x94\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

        let (rest, result) = ResPackageHeader::parse(test_header).unwrap();

        let expected_base = ResChunkHeader {
            res_type: ChunkType::ResTablePackageType,
            header_size: 0x120,
            size: 0x6e2fc,
        };

        let expected = ResPackageHeader {
            base_header: expected_base,
            id: 0x7f,
            name: utf16_lit("com.bogadana.restest"),
            type_strings_offset: 0x120,
            last_public_type_index: 0,
            key_strings_offset: 0x294,
            last_public_key_index: 0,
            type_id_offset: 0,
        };

        assert_eq!(result, expected);
        assert_eq!(rest.len(), 0);
    }
    
    #[test]
    fn parse_typespec_header() {
        let test_header = b"\x02\x02\x10\x00\x2C\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00";

        let (rest, result) = ResTypeSpecHeader::parse(test_header).unwrap();

        let expected_base = ResChunkHeader {
            res_type: ChunkType::ResTableTypeSpecType,
            header_size: 0x10,
            size: 0x2c,
        };

        let expected = ResTypeSpecHeader {
            base_header: expected_base,
            id: 0x1,
            res0: 0,
            res1: 0,
            entry_count: 0x7,
        };

        assert_eq!(result, expected);
        assert_eq!(rest.len(), 0);
    }
}
