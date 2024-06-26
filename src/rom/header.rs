use std::{error, fmt, str::{self, FromStr}};

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
    SingleScreen
}

impl ToString for Mirroring {
    fn to_string(&self) -> String {
        match *self {
            Mirroring::Vertical => "Vertical".to_string(),
            Mirroring::Horizontal => "Horizontal".to_string(),
            Mirroring::FourScreen => "Four Screen".to_string(),
            Mirroring::SingleScreen => "Single Screen".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TvSystem {
    NTSC,
    PAL,
    DualCompatible,
    Dendy
}

impl ToString for TvSystem {
    fn to_string(&self) -> String {
        match *self {
            TvSystem::NTSC => "NTSC".to_string(),
            TvSystem::PAL => "PAL".to_string(),
            TvSystem::DualCompatible => "Dual Compatible".to_string(),
            TvSystem::Dendy => "Dendy".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RomHeader {
    pub prg_rom_size: u64,
    pub chr_rom_size: u64,
    pub mapper: u32,
    pub submapper: u32,
    pub trainer_present: bool,
    pub mirroring: Mirroring,
    pub nes2: bool,
    pub prg_ram_size: u64,
    pub chr_ram_size: u64,
    pub prg_nvram_size: u64,
    pub chr_nvram_size: u64,
    pub tv_system: TvSystem,
    pub prg_ram_present: bool,
    pub has_bus_conflicts: bool
}

const PRG_ROM_BANK_SIZE: u64 = 16384;
const CHR_ROM_BANK_SIZE: u64 = 8192;
const PRG_RAM_BANK_SIZE: u64 = 8192;

impl RomHeader {
    pub fn parse(header_data: &[u8; 16]) -> Result<RomHeader, InvalidHeaderError> {
        str::from_utf8(&header_data[0..4])
            .or_else(|_| -> Result<&str, InvalidHeaderError> {
                Err(InvalidHeaderError)
            })
            .and_then(|constant| -> Result<RomHeader, InvalidHeaderError> {
                if constant.eq("NES\x1A") {
                    let ines2 = ((header_data[7] >> 2) & 3) == 2;
                    if ines2 {
                        RomHeader::parse_ines2_header(header_data)
                    } else {
                        RomHeader::parse_ines1_header(header_data)
                    }
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

    fn parse_ines1_header(header_data: &[u8; 16]) -> Result<RomHeader, InvalidHeaderError> {
        let flags6 = header_data[6];
        let flags7 = header_data[7];
        let flags8 = header_data[8];
        let flags9 = header_data[9];
        let flags10 = header_data[10];

        let tv = if (flags10 & 3) == 1 || (flags10 & 3) == 3 { 
            TvSystem::DualCompatible
        } else {
            if (flags9 & 1) == 0 { 
                TvSystem::NTSC
            } else {
                TvSystem::PAL
            }
        };

        Ok(RomHeader{
            prg_rom_size: header_data[4] as u64 * PRG_ROM_BANK_SIZE,
            chr_rom_size: header_data[5] as u64 * CHR_ROM_BANK_SIZE,
            mapper: ((flags6 >> 4) | (flags7 & 0xF0)) as u32,
            submapper: 0,
            trainer_present: ((flags6 >> 3) & 1) > 0,
            mirroring: RomHeader::nametable_layout(&flags6),
            nes2: false,
            prg_ram_size: (if flags8 == 0 { 1 } else { flags8 }) as u64 * PRG_RAM_BANK_SIZE,
            chr_ram_size: 0,
            prg_nvram_size: 0,
            chr_nvram_size: 0,
            tv_system: tv,
            prg_ram_present: ((flags10 >> 4) & 1) > 0,
            has_bus_conflicts: ((flags10 >> 5) & 1) > 0
        })
    }

    fn ines2_rom_size(bytes: u16, bank_size: u64) -> u64 {
        return if bytes & 0xF00 == 0xF00 {
            // Exponent-multiplier notation
            let multiplier: u64 = (bytes & 0x3).into();
            let exponent: u32 = (bytes & 0xFC).into();
            const BASE: u64 = 2;
            BASE.pow(exponent) * (multiplier * 2 + 1)
        } else {
            (bytes & 0xFFF) as u64 * bank_size
        }
    }

    fn ines2_ram_size(shift_count: u8) -> u64 {
        return if shift_count == 0 {
            0u64
        } else {
            64u64 << (shift_count as u64)
        }
    }

    fn parse_ines2_header(header_data: &[u8; 16]) -> Result<RomHeader, InvalidHeaderError> {
        let flags6 = header_data[6];
        let flags7 = header_data[7];
        let flags8 = header_data[8];
        let flags9 = header_data[9];
        let flags10 = header_data[10];
        let flags11 = header_data[11];
        let flags12 = header_data[12];

        let prg_rom_size_bytes = header_data[4] as u16 | ((flags9 as u16 & 0xF) << 8);
        let chr_rom_size_bytes = header_data[5] as u16 | ((flags9 as u16 & 0xF0) << 4);

        let tv = match flags12 & 0x3 {
            0 => TvSystem::NTSC,
            1 => TvSystem::PAL,
            2 => TvSystem::DualCompatible,
            3 => TvSystem::Dendy,
            _ => panic!("CPU/PPU timing bits with value higher than 3. This should not happen!")
        };

        Ok(RomHeader{
            prg_rom_size: RomHeader::ines2_rom_size(prg_rom_size_bytes, PRG_ROM_BANK_SIZE),
            chr_rom_size: RomHeader::ines2_rom_size(chr_rom_size_bytes, CHR_ROM_BANK_SIZE),
            mapper: ((flags6 >> 4) | (flags7 & 0xF0) | (flags8 << 4)) as u32,
            submapper: (flags8 >> 4) as u32,
            trainer_present: ((flags6 >> 3) & 1) > 0,
            mirroring: RomHeader::nametable_layout(&flags6),
            nes2: true,
            prg_ram_size: RomHeader::ines2_ram_size(flags10 & 0xF),
            chr_ram_size: RomHeader::ines2_ram_size(flags11 & 0xF),
            prg_nvram_size: RomHeader::ines2_ram_size(flags10 >> 4),
            chr_nvram_size: RomHeader::ines2_ram_size(flags11 >> 4),
            tv_system: tv,
            prg_ram_present: flags10 & 0xF > 0,
            has_bus_conflicts: false
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_ines1_header() {
        let valid_header: [u8; 16] = [
            0x4E, 0x45, 0x53, 0x1A, 0x10, 0x00, 0x11, 0x00, 
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = RomHeader::parse(&valid_header);

        assert_eq!(result, Ok(RomHeader{
            prg_rom_size: 16 * PRG_ROM_BANK_SIZE,
            chr_rom_size: 0,
            mapper: 1,
            trainer_present: false,
            mirroring: Mirroring::Horizontal,
            nes2: false,
            prg_ram_size: 1 * PRG_RAM_BANK_SIZE,
            tv_system: TvSystem::NTSC,
            prg_ram_present: false,
            has_bus_conflicts: false, 
            submapper: 0, 
            chr_ram_size: 0, 
            prg_nvram_size: 0, 
            chr_nvram_size: 0 
        }));
    }

    #[test]
    fn parse_invalid_ines1_header() {
        let invalid_header: [u8; 16] = [
            0x4E, 0x45, 0x53, 0x23, 0x10, 0x00, 0x11, 0x00, 
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = RomHeader::parse(&invalid_header);

        assert_eq!(result, Err(InvalidHeaderError));
    }

    #[test]
    fn parse_valid_ines2_header() {
        let valid_header: [u8; 16] = [
            0x4E, 0x45, 0x53, 0x1A, 0x01, 0x01, 0x00, 0x08,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = RomHeader::parse(&valid_header);

        assert_eq!(result, Ok(RomHeader{
            prg_rom_size: PRG_ROM_BANK_SIZE,
            chr_rom_size: CHR_ROM_BANK_SIZE, 
            mapper: 0, 
            submapper: 0, 
            trainer_present: false, 
            mirroring: Mirroring::Vertical, 
            nes2: true, 
            prg_ram_size: 0, 
            chr_ram_size: 0, 
            prg_nvram_size: 0, 
            chr_nvram_size: 0, 
            tv_system: TvSystem::NTSC, 
            prg_ram_present: false, 
            has_bus_conflicts: false 
        }));
    }

    #[test]
    fn parse_invalid_ines2_header() {
        let invalid_header: [u8; 16] = [
            0x4E, 0x45, 0x53, 0x23, 0x10, 0x00, 0x08, 0x08, 
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = RomHeader::parse(&invalid_header);

        assert_eq!(result, Err(InvalidHeaderError));
    }
}