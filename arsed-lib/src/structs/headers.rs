use nom_derive::NomLE;

use super::config::ResConfig;

#[repr(u16)]
#[derive(PartialEq, Debug, NomLE)]
pub enum ChunkType {
    ResNullType = 0x0000,
    ResStringPoolType = 0x0001,
    ResTableType = 0x0002,
    ResXmlType = 0x0003,

    //ResXmlFirstChunkType = 0x0100,
    ResXmlStartNamespaceType = 0x0100,
    ResXmlEndNamespaceType = 0x0101,
    ResXmlStartElementType = 0x0102,
    ResXmlEndElementType = 0x0103,
    ResXmlCdataType = 0x0104,
    ResXmlLastChunkType = 0x017f,
    ResXmlResourceMapType = 0x0180,

    ResTablePackageType = 0x0200,
    ResTableTypeType = 0x0201,
    ResTableTypeSpecType = 0x0202,
    ResTableLibraryType = 0x0203,
    ResTableOverlayableType = 0x0204,
    ResTableOverlayablePolicyType = 0x0205,
    ResTableStagedAliasType = 0x0206,
}

pub trait ExtendedHeader {
    fn get_base_header(&self) -> &ResChunkHeader;
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ResChunkHeader {
    pub res_type: ChunkType,
    pub header_size: u16,
    pub size: u32,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ResStringPoolHeader {
    pub base_header: ResChunkHeader,
    pub string_count: u32,
    pub style_count: u32,
    pub flags: u32,
    pub strings_start: u32,
    pub styles_start: u32,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ResPackageHeader {
    pub base_header: ResChunkHeader,
    pub id: u32,
    pub name: [u16; 128],
    pub type_strings_offset: u32,
    pub last_public_type_index: u32,
    pub key_strings_offset: u32,
    pub last_public_key_index: u32,
    pub type_id_offset: u32,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ResTypeSpecHeader {
    pub base_header: ResChunkHeader,
    pub id: u8,
    pub res0: u8,
    pub res1: u8,
    #[nom(AlignBefore(4))]
    pub entry_count: u32,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ResTypeHeader {
    pub base_header: ResChunkHeader,
    pub id: u8,
    pub flags: u8,
    pub reserved: u16,
    pub entry_count: u32,
    pub entries_start_offset: u32,
    pub config: ResConfig,

}

//TODO: make this a macro
impl ExtendedHeader for ResStringPoolHeader {
    fn get_base_header(&self) -> &ResChunkHeader {
        &self.base_header
    }
}

impl ExtendedHeader for ResTypeHeader {
    fn get_base_header(&self) -> &ResChunkHeader {
        &self.base_header
    }
}

impl ExtendedHeader for ResPackageHeader {
    fn get_base_header(&self) -> &ResChunkHeader {
        &self.base_header
    }
}

impl ExtendedHeader for ResTypeSpecHeader {
    fn get_base_header(&self) -> &ResChunkHeader {
        &self.base_header
    }
}
