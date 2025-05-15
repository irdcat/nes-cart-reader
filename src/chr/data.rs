use std::{error, fmt};

const CHR_BANK_SIZE: usize = 0x2000;
const PATTERN_TABLES_PER_BANK: usize = 2;
const PATTERN_TABLE_SIZE_IN_BYTES: usize = 0x1000;
const TILES_PER_PATTERN_TABLE: usize = 256;
const TILE_SIZE_IN_BYTES: usize = 16;
const TILE_PATTERN_ROWS: usize = 8;

pub const TILES_PER_ROW: usize = 16;
pub const TILE_WIDTH_IN_PIXELS: usize = 8;
pub const TILE_ROWS: usize = 16;
pub const TILE_HEIGHT_IN_PIXELS: usize = 8;
pub const BITS_PER_PIXEL: usize = 2;

pub const TILE_PATTERN_WIDTH_IN_PIXELS: usize = TILES_PER_ROW * TILE_WIDTH_IN_PIXELS;
pub const TILE_PATTERN_HEIGHT_IN_PIXELS: usize = TILE_ROWS * TILE_HEIGHT_IN_PIXELS;

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidChrDataError;

impl fmt::Display for InvalidChrDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid CHR ROM data")
    }
}

impl error::Error for InvalidChrDataError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tile {
    pub pattern: [u16; TILE_PATTERN_ROWS],
}

pub type PatternTable = [Tile; TILES_PER_PATTERN_TABLE];

#[derive(PartialEq, Debug, Clone)]
pub struct ChrData {
    pub pattern_tables: Vec<PatternTable>,
}

impl ChrData {
    pub fn parse(chr_data: Vec<u8>) -> Result<ChrData, InvalidChrDataError> {
        if chr_data.len() % CHR_BANK_SIZE != 0 {
            return Err(InvalidChrDataError);
        }
        let chr_rom_banks = chr_data.len() / CHR_BANK_SIZE;

        let mut tables: Vec<[Tile; TILES_PER_PATTERN_TABLE]> = Vec::new();
        let pattern_table_count = chr_rom_banks * PATTERN_TABLES_PER_BANK;
        for pattern_table_id in 0..pattern_table_count {
            let tiles = ChrData::get_tiles_from_pattern_table(pattern_table_id, &chr_data);
            tables.push(tiles);
        }

        Ok(ChrData {
            pattern_tables: tables,
        })
    }

    fn interleave_pattern_bytes(lsb: u8, msb: u8) -> u16 {
        let mut pattern = (lsb as u16) | (msb as u16) << 8;
        pattern = (pattern & 0xF00F) | ((pattern & 0x0F00) >> 4) | ((pattern & 0x00F0) << 4);
        pattern = (pattern & 0xC3C3) | ((pattern & 0x3030) >> 2) | ((pattern & 0x0C0C) << 2);
        pattern = (pattern & 0x9999) | ((pattern & 0x4444) >> 1) | ((pattern & 0x2222) << 1);
        pattern
    }

    fn get_tiles_from_pattern_table(
        pattern_table_id: usize,
        chr_data: &[u8],
    ) -> [Tile; TILES_PER_PATTERN_TABLE] {
        let mut tiles: [Tile; TILES_PER_PATTERN_TABLE] = [Tile {
            pattern: [0; TILE_PATTERN_ROWS],
        }; TILES_PER_PATTERN_TABLE];
        for (tile_number, tile) in tiles.iter_mut().enumerate().take(TILES_PER_PATTERN_TABLE) {
            let offset =
                PATTERN_TABLE_SIZE_IN_BYTES * pattern_table_id + tile_number * TILE_SIZE_IN_BYTES;
            let mut tile_pattern: [u16; TILE_PATTERN_ROWS] = [0; TILE_PATTERN_ROWS];
            for row in 0..TILE_PATTERN_ROWS {
                let tile_lsb = chr_data[offset + row];
                let tile_msb = chr_data[offset + row + TILE_PATTERN_ROWS];
                tile_pattern[row] = ChrData::interleave_pattern_bytes(tile_lsb, tile_msb);
            }
            *tile = Tile {
                pattern: tile_pattern,
            };
        }
        tiles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_invalid_chr_data() {
        let invalid_chr_data: Vec<u8> = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ]
        .to_vec();
        assert_eq!(ChrData::parse(invalid_chr_data), Err(InvalidChrDataError));
    }

    #[test]
    fn parse_valid_chr_data() {
        let mut valid_chr_data: Vec<u8> = [0; 0x2000].to_vec();
        let valid_tile_data: [u8; 16] = [
            0x41, 0xC2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x16, 0x21,
            0x42, 0x87,
        ];
        for i in 0..valid_tile_data.len() {
            valid_chr_data[i] = valid_tile_data[i];
        }

        let expected_tile = Tile {
            pattern: [
                0x1003, 0x500C, 0x1030, 0x10C0, 0x0328, 0x0C02, 0x3008, 0xC02A,
            ],
        };
        let result = ChrData::parse(valid_chr_data);
        assert!(result.is_ok());
        let parsed_chr_data = result.unwrap();
        let parsed_tile = parsed_chr_data.pattern_tables[0][0];
        assert_eq!(parsed_tile, expected_tile);
    }
}
