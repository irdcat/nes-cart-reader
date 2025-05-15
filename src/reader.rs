use std::{error, fmt};

use super::{
    chr::data::{ChrData, InvalidChrDataError},
    header::data::{HeaderData, InvalidHeaderError},
    prg::data::{InvalidPrgDataError, PrgData},
};

#[derive(Debug, PartialEq)]
pub struct RomReaderResult {
    pub header: HeaderData,
    pub chr_data: ChrData,
    pub prg_data: PrgData,
}

#[derive(Debug, PartialEq)]
pub enum RomReaderError {
    Header(InvalidHeaderError),
    ChrData(InvalidChrDataError),
    PrgData(InvalidPrgDataError),
}

impl fmt::Display for RomReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RomReaderError::Header(e) => e.fmt(f),
            RomReaderError::ChrData(e) => e.fmt(f),
            RomReaderError::PrgData(e) => e.fmt(f),
        }
    }
}

impl error::Error for RomReaderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            RomReaderError::Header(ref e) => Some(e),
            RomReaderError::ChrData(ref e) => Some(e),
            RomReaderError::PrgData(ref e) => Some(e),
        }
    }
}

impl From<InvalidHeaderError> for RomReaderError {
    fn from(value: InvalidHeaderError) -> Self {
        RomReaderError::Header(value)
    }
}

impl From<InvalidChrDataError> for RomReaderError {
    fn from(value: InvalidChrDataError) -> Self {
        RomReaderError::ChrData(value)
    }
}

impl From<InvalidPrgDataError> for RomReaderError {
    fn from(value: InvalidPrgDataError) -> Self {
        RomReaderError::PrgData(value)
    }
}

pub struct RomReader;

impl RomReader {
    pub fn read(data: Vec<u8>) -> Result<RomReaderResult, RomReaderError> {
        const HEADER_SIZE_BYTES: usize = 16;
        let header_bytes: &[u8; HEADER_SIZE_BYTES] = &data[0..HEADER_SIZE_BYTES]
            .try_into()
            .expect("Slice with incorrect lenght!");
        let header_parse_result = HeaderData::parse(header_bytes);
        if let Err(e) = header_parse_result {
            return Err(RomReaderError::from(e));
        }
        let header = header_parse_result.unwrap();
        let prg_rom_start = HEADER_SIZE_BYTES + (if header.trainer_present { 512 } else { 0 });
        let prg_rom_bytes =
            data[prg_rom_start..prg_rom_start + header.prg_rom_size as usize].to_vec();
        let prg_data_parse_result = PrgData::parse(prg_rom_bytes);
        if let Err(e) = prg_data_parse_result {
            return Err(RomReaderError::from(e));
        }
        let prg_data = prg_data_parse_result.unwrap();
        let chr_rom_start = prg_rom_start + header.prg_rom_size as usize;
        let chr_rom_bytes =
            data[chr_rom_start..chr_rom_start + header.chr_rom_size as usize].to_vec();
        let chr_data_parse_result = ChrData::parse(chr_rom_bytes);
        if let Err(e) = chr_data_parse_result {
            return Err(RomReaderError::from(e));
        }
        let chr_data = chr_data_parse_result.unwrap();

        Ok(RomReaderResult {
            header,
            chr_data,
            prg_data,
        })
    }
}
