pub mod chr;
pub mod header;

use self::header::{InvalidHeaderError, RomHeader};
use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub struct RomReaderResult {
    pub header: RomHeader,
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
}

impl fmt::Display for RomReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RomReaderError::InvalidHeader(e) => e.fmt(f),
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
        let header_data: &[u8; 16] = &params.data[0..16]
            .try_into()
            .expect("Slice with incorrect lenght!");
        let header_parse_result = RomHeader::parse(header_data);
        match header_parse_result {
            Ok(value) => Ok(RomReaderResult { header: value }),
            Err(e) => Err(RomReaderError::from(e)),
        }
    }
}
