mod header;
mod chr;

use std::{error, fmt};
use self::header::{InvalidHeaderError, RomHeader};

#[derive(Debug, PartialEq)]
pub struct RomReaderResult {
    pub header: RomHeader
}

pub struct RomReaderParams {
    pub data: Vec<u8>,
    pub origin: u16
}

#[derive(Debug, PartialEq)]
pub enum RomReaderError{
    InvalidHeader(InvalidHeaderError)
}

impl fmt::Display for RomReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RomReaderError::InvalidHeader(e) => e.fmt(f)
        }
    }
}

impl error::Error for RomReaderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            RomReaderError::InvalidHeader(ref e) => Some(e),
        }
    }
}

impl From<InvalidHeaderError> for RomReaderError {
    fn from(value: InvalidHeaderError) -> Self {
        RomReaderError::InvalidHeader(value)
    }
}

pub struct RomReader;

impl RomReader {
    pub fn read(params: RomReaderParams) -> Result<RomReaderResult, RomReaderError> {
        let header_data: &[u8; 16] = &params.data[0..16].try_into().expect("Slice with incorrect lenght!");
        let header_parse_result = RomHeader::parse(header_data);
        match header_parse_result {
            Ok(value) => Ok(RomReaderResult { header: value }),
            Err(e) => Err(RomReaderError::from(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::header::*;

    #[test]
    fn read_rom_with_valid_header() {
        let params = RomReaderParams{
            data: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x10, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
                ],
            origin: 0xC000    
        };
        let result = RomReader::read(params);
        assert_eq!(result, Ok(RomReaderResult{
            header: RomHeader{
                prg_rom_banks: 16,
                chr_rom_banks: 0,
                mapper: 1,
                trainer_present: false,
                mirroring: Mirroring::Horizontal,
                nes2: false
            }
        }));
    }

    #[test]
    fn read_rom_with_invalid_header() {
        let params = RomReaderParams{
            data: vec![
                0x4E, 0x45, 0x53, 0x23, 0x10, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
                ],
            origin: 0xC000    
        };
        let result = RomReader::read(params);
        assert_eq!(result, Err(RomReaderError::InvalidHeader(InvalidHeaderError)));
    }
}