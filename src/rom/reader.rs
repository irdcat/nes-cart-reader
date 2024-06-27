use std::{error, fmt};

use super::{chr::{ChrData, InvalidChrDataError}, header::{InvalidHeaderError, RomHeader}};

#[derive(Debug, PartialEq)]
pub struct RomReaderResult {
    pub header: RomHeader,
    pub chr_data: ChrData
}

// TODO: Remove it later
#[allow(dead_code)]
pub struct RomReaderParams {
    pub data: Vec<u8>,
    pub origin: u16,
}

#[derive(Debug, PartialEq)]
pub enum RomReaderError {
    InvalidHeader(InvalidHeaderError),
    InvalidChrData(InvalidChrDataError),
}

impl fmt::Display for RomReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RomReaderError::InvalidHeader(e) => e.fmt(f),
            RomReaderError::InvalidChrData(e) => e.fmt(f),
        }
    }
}

impl error::Error for RomReaderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            RomReaderError::InvalidHeader(ref e) => Some(e),
            RomReaderError::InvalidChrData(ref e) => Some(e),
        }
    }
}

impl From<InvalidHeaderError> for RomReaderError {
    fn from(value: InvalidHeaderError) -> Self {
        RomReaderError::InvalidHeader(value)
    }
}

impl From<InvalidChrDataError> for RomReaderError {
    fn from(value: InvalidChrDataError) -> Self {
        RomReaderError::InvalidChrData(value)
    }
}

pub struct RomReader;

impl RomReader {
    pub fn read(params: RomReaderParams) -> Result<RomReaderResult, RomReaderError> {
        const HEADER_SIZE_BYTES: usize = 16;
        
        let header_bytes: &[u8; HEADER_SIZE_BYTES] = &params.data[0..HEADER_SIZE_BYTES]
            .try_into()
            .expect("Slice with incorrect lenght!");
        let header_parse_result = RomHeader::parse(header_bytes);
        if let Err(e) = header_parse_result {
            return Err(RomReaderError::from(e));
        }
        let header = header_parse_result.unwrap();
        
        let prg_rom_start = HEADER_SIZE_BYTES + (if header.trainer_present { 512 } else { 0 });
        // TODO: Parsing PRG ROM

        let chr_rom_start = prg_rom_start + header.prg_rom_size as usize;
        let chr_rom_bytes = params.data[chr_rom_start..chr_rom_start + header.chr_rom_size as usize].to_vec();
        let chr_data_parse_result = ChrData::parse(chr_rom_bytes);
        if let Err(e) = chr_data_parse_result {
            return Err(RomReaderError::from(e));
        }
        let chr_data = chr_data_parse_result.unwrap();

        Ok(RomReaderResult{ 
            header, 
            chr_data 
        })
    }
}