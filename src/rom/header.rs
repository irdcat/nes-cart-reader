use std::{str, fmt, error};

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidHeaderError;

impl fmt::Display for InvalidHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid header constant")
    }
}

impl error::Error for InvalidHeaderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(PartialEq, Debug)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
    SingleScreen
}

#[derive(PartialEq, Debug)]
pub struct RomHeader {
    pub prg_rom_banks: usize,
    pub chr_rom_banks: usize,
    pub mapper: u32,
    pub trainer_present: bool,
    pub mirroring: Mirroring,
    pub nes2: bool,
}

impl RomHeader {
    pub fn parse(header_data: &[u8; 16]) -> Result<RomHeader, InvalidHeaderError> {
        str::from_utf8(&header_data[0..4])
            .or_else(|_| -> Result<&str, InvalidHeaderError> {
                Err(InvalidHeaderError)
            })
            .and_then(|constant| -> Result<RomHeader, InvalidHeaderError> {
                if constant.eq("NES\x1A") {
                    Ok(RomHeader {
                        prg_rom_banks: header_data[4] as usize,
                        chr_rom_banks: header_data[5] as usize,
                        mapper: ((header_data[6] >> 4) | (header_data[7] & 0xF0)) as u32,
                        trainer_present: ((header_data[6] >> 3) & 1) > 0,
                        mirroring: RomHeader::nametable_layout(&header_data[6]),
                        nes2: ((header_data[7] >> 2) & 3) == 2
                    })
                } else {
                    Err(InvalidHeaderError)
                }
            })
    }

    fn nametable_layout(flags6: &u8) -> Mirroring {
        let nametable_flag = ((flags6 >> 2) & 2) | (flags6 & 1);
        match nametable_flag {
            0 => Mirroring::Vertical,
            1 => Mirroring::Horizontal,
            2 => Mirroring::SingleScreen,
            3 => Mirroring::FourScreen,
            _ => panic!("Nametable bits with higher value that 3. This should not happen!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_header() {
        let valid_header: [u8; 16] = [0x4E, 0x45, 0x53, 0x1A, 0x10, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = RomHeader::parse(&valid_header);

        assert_eq!(result, Ok(RomHeader{
            prg_rom_banks: 16,
            chr_rom_banks: 0,
            mapper: 1,
            trainer_present: false,
            mirroring: Mirroring::Horizontal,
            nes2: false
        }));
    }

    #[test]
    fn parse_invalid_header() {
        let invalid_header: [u8; 16] = [0x4E, 0x45, 0x53, 0x23, 0x10, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = RomHeader::parse(&invalid_header);

        assert_eq!(result, Err(InvalidHeaderError));
    }
}