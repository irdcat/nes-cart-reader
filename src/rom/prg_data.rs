use std::{collections::{BTreeMap, HashMap}, error, fmt};

use super::instructions::{Instruction, Mnemonic, Operand};

const PRG_BANK_SIZE: usize = 0x4000;
const PRG_LOWEST_START_ADDRESS: u16 = 0x6000;

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidPrgDataError;

impl fmt::Display for InvalidPrgDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid PRG ROM data")
    }
}

impl error::Error for InvalidPrgDataError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrgData {
    pub instructions: BTreeMap<u16, Instruction>,
    pub labels: HashMap<u16, String>,
}

impl PrgData {
    pub fn parse(prg_data: Vec<u8>) -> Result<PrgData, InvalidPrgDataError> {
        use InterruptRoutineType::*;
        if prg_data.len() % PRG_BANK_SIZE != 0 {
            return Err(InvalidPrgDataError)
        }

        let mut instructions = BTreeMap::<u16, Instruction>::new();
        let mut labels = HashMap::<u16, String>::new();

        let nmi_isr_location = PrgData::resolve_isr_location(&prg_data, NMI);
        let irq_isr_location = PrgData::resolve_isr_location(&prg_data, IRQ);

        if !PrgData::is_isr_unused(nmi_isr_location) {
            instructions.append(&mut PrgData::disassemble_interrupt_until_rti(nmi_isr_location, &prg_data));
            labels.insert(nmi_isr_location, "nmi".to_string());
        }
        if !PrgData::is_isr_unused(irq_isr_location) {
            instructions.append(&mut PrgData::disassemble_interrupt_until_rti(irq_isr_location, &prg_data));
            labels.insert(irq_isr_location, "irq".to_string());
        }

        Ok(PrgData { instructions, labels })
    }

    fn disassemble_interrupt_until_rti(start_addr: u16, prg_data: &Vec<u8>) -> BTreeMap<u16, Instruction> {
        use crate::rom::instructions::Mnemonic::*;

        let mut instructions = BTreeMap::<u16, Instruction>::new();
        let mut context = ExecutionContext::new(start_addr, prg_data);
        loop {
            let address = context.get_current_pc();
            let instruction = context.execute_instruction();
            if address < PRG_LOWEST_START_ADDRESS {
                continue;
            }
            let relative_address = address - PRG_LOWEST_START_ADDRESS;
            let is_rti = instruction.mnemonic == Rti;
            instructions.insert(relative_address, instruction);
            if is_rti {
                break;
            }
        }
        instructions
    }

    fn disassemble_isr_until_infinite_loop(_start_addr: u16, _prg_data: &Vec<u8>) -> BTreeMap<u16, Instruction> {

        // TODO: Implement instruction decoding with infinite loop detection
        BTreeMap::new()
    }

    fn resolve_isr_location(prg_data: &Vec<u8>, kind: InterruptRoutineType) -> u16 {
        use InterruptRoutineType::*;
        let read16 = |addr: u16, data: &Vec<u8>| -> u16 {
            (data[addr as usize] as u16) | ((data[(addr + 1) as usize] as u16) << 8) 
        };
        // This so far assumes that we're dealing with NROM cartridges
        let vector_location = match kind {
            NMI => prg_data.len() - 6,
            RESET => prg_data.len() - 4,
            IRQ => prg_data.len() - 2,
        };
        read16(vector_location as u16, prg_data)
    }

    fn is_isr_unused(addr: u16) -> bool {
        addr < PRG_LOWEST_START_ADDRESS
    }
}

enum InterruptRoutineType {
    NMI,
    IRQ,
    RESET
}

struct ExecutionContext<'a> {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: u8, // TODO: Change to struct type
    pc: u16,

    ram: Vec<u8>,
    prg: &'a Vec<u8>,
}

impl <'a> ExecutionContext<'a> {
    pub fn new(start_addr: u16, prg_data: &'a Vec<u8>) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            p: 0,
            pc: start_addr,

            ram: Vec::with_capacity(0x800),
            prg: prg_data
        }
    }

    pub fn get_current_pc(&self) -> u16 {
        self.pc
    }

    pub fn execute_instruction(&mut self) -> Instruction {
        let mut mnemonic: Option<Mnemonic> = None;
        let mut operand: Option<Operand> = None;
        let mut bytes = Vec::<u8>::new();

        let opcode = self.fetch_immediate8();

        match opcode {
            0x00 => todo!(),
            0x01 => todo!(),
            0x02 => todo!(),
            0x03 => todo!(),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => todo!(),
            0x07 => todo!(),
            0x08 => todo!(),
            0x09 => todo!(),
            0x0A => todo!(),
            0x0B => todo!(),
            0x0C => todo!(),
            0x0D => todo!(),
            0x0E => todo!(),
            0x0F => todo!(),
            0x10 => todo!(),
            0x11 => todo!(),
            0x12 => todo!(),
            0x13 => todo!(),
            0x14 => todo!(),
            0x15 => todo!(),
            0x16 => todo!(),
            0x17 => todo!(),
            0x18 => todo!(),
            0x19 => todo!(),
            0x1A => todo!(),
            0x1B => todo!(),
            0x1C => todo!(),
            0x1D => todo!(),
            0x1E => todo!(),
            0x1F => todo!(),
            0x20 => todo!(),
            0x21 => todo!(),
            0x22 => todo!(),
            0x23 => todo!(),
            0x24 => todo!(),
            0x25 => todo!(),
            0x26 => todo!(),
            0x27 => todo!(),
            0x28 => todo!(),
            0x29 => todo!(),
            0x2A => todo!(),
            0x2B => todo!(),
            0x2C => todo!(),
            0x2D => todo!(),
            0x2E => todo!(),
            0x2F => todo!(),
            0x30 => todo!(),
            0x31 => todo!(),
            0x32 => todo!(),
            0x33 => todo!(),
            0x34 => todo!(),
            0x35 => todo!(),
            0x36 => todo!(),
            0x37 => todo!(),
            0x38 => todo!(),
            0x39 => todo!(),
            0x3A => todo!(),
            0x3B => todo!(),
            0x3C => todo!(),
            0x3D => todo!(),
            0x3E => todo!(),
            0x3F => todo!(),
            0x40 => todo!(),
            0x41 => todo!(),
            0x42 => todo!(),
            0x43 => todo!(),
            0x44 => todo!(),
            0x45 => todo!(),
            0x46 => todo!(),
            0x47 => todo!(),
            0x48 => todo!(),
            0x49 => todo!(),
            0x4A => todo!(),
            0x4B => todo!(),
            0x4C => todo!(),
            0x4D => todo!(),
            0x4E => todo!(),
            0x4F => todo!(),
            0x50 => todo!(),
            0x51 => todo!(),
            0x52 => todo!(),
            0x53 => todo!(),
            0x54 => todo!(),
            0x55 => todo!(),
            0x56 => todo!(),
            0x57 => todo!(),
            0x58 => todo!(),
            0x59 => todo!(),
            0x5A => todo!(),
            0x5B => todo!(),
            0x5C => todo!(),
            0x5D => todo!(),
            0x5E => todo!(),
            0x5F => todo!(),
            0x60 => todo!(),
            0x61 => todo!(),
            0x62 => todo!(),
            0x63 => todo!(),
            0x64 => todo!(),
            0x65 => todo!(),
            0x66 => todo!(),
            0x67 => todo!(),
            0x68 => todo!(),
            0x69 => todo!(),
            0x6A => todo!(),
            0x6B => todo!(),
            0x6C => todo!(),
            0x6D => todo!(),
            0x6E => todo!(),
            0x6F => todo!(),
            0x70 => todo!(),
            0x71 => todo!(),
            0x72 => todo!(),
            0x73 => todo!(),
            0x74 => todo!(),
            0x75 => todo!(),
            0x76 => todo!(),
            0x77 => todo!(),
            0x78 => todo!(),
            0x79 => todo!(),
            0x7A => todo!(),
            0x7B => todo!(),
            0x7C => todo!(),
            0x7D => todo!(),
            0x7E => todo!(),
            0x7F => todo!(),
            0x80 => todo!(),
            0x81 => todo!(),
            0x82 => todo!(),
            0x83 => todo!(),
            0x84 => todo!(),
            0x85 => todo!(),
            0x86 => todo!(),
            0x87 => todo!(),
            0x88 => todo!(),
            0x89 => todo!(),
            0x8A => todo!(),
            0x8B => todo!(),
            0x8C => todo!(),
            0x8D => todo!(),
            0x8E => todo!(),
            0x8F => todo!(),
            0x90 => todo!(),
            0x91 => todo!(),
            0x92 => todo!(),
            0x93 => todo!(),
            0x94 => todo!(),
            0x95 => todo!(),
            0x96 => todo!(),
            0x97 => todo!(),
            0x98 => todo!(),
            0x99 => todo!(),
            0x9A => todo!(),
            0x9B => todo!(),
            0x9C => todo!(),
            0x9D => todo!(),
            0x9E => todo!(),
            0x9F => todo!(),
            0xA0 => todo!(),
            0xA1 => todo!(),
            0xA2 => todo!(),
            0xA3 => todo!(),
            0xA4 => todo!(),
            0xA5 => todo!(),
            0xA6 => todo!(),
            0xA7 => todo!(),
            0xA8 => todo!(),
            0xA9 => todo!(),
            0xAA => todo!(),
            0xAB => todo!(),
            0xAC => todo!(),
            0xAD => todo!(),
            0xAE => todo!(),
            0xAF => todo!(),
            0xB0 => todo!(),
            0xB1 => todo!(),
            0xB2 => todo!(),
            0xB3 => todo!(),
            0xB4 => todo!(),
            0xB5 => todo!(),
            0xB6 => todo!(),
            0xB7 => todo!(),
            0xB8 => todo!(),
            0xB9 => todo!(),
            0xBA => todo!(),
            0xBB => todo!(),
            0xBC => todo!(),
            0xBD => todo!(),
            0xBE => todo!(),
            0xBF => todo!(),
            0xC0 => todo!(),
            0xC1 => todo!(),
            0xC2 => todo!(),
            0xC3 => todo!(),
            0xC4 => todo!(),
            0xC5 => todo!(),
            0xC6 => todo!(),
            0xC7 => todo!(),
            0xC8 => todo!(),
            0xC9 => todo!(),
            0xCA => todo!(),
            0xCB => todo!(),
            0xCC => todo!(),
            0xCD => todo!(),
            0xCE => todo!(),
            0xCF => todo!(),
            0xD0 => todo!(),
            0xD1 => todo!(),
            0xD2 => todo!(),
            0xD3 => todo!(),
            0xD4 => todo!(),
            0xD5 => todo!(),
            0xD6 => todo!(),
            0xD7 => todo!(),
            0xD8 => todo!(),
            0xD9 => todo!(),
            0xDA => todo!(),
            0xDB => todo!(),
            0xDC => todo!(),
            0xDD => todo!(),
            0xDE => todo!(),
            0xDF => todo!(),
            0xE0 => todo!(),
            0xE1 => todo!(),
            0xE2 => todo!(),
            0xE3 => todo!(),
            0xE4 => todo!(),
            0xE5 => todo!(),
            0xE6 => todo!(),
            0xE7 => todo!(),
            0xE8 => todo!(),
            0xE9 => todo!(),
            0xEA => todo!(),
            0xEB => todo!(),
            0xEC => todo!(),
            0xED => todo!(),
            0xEE => todo!(),
            0xEF => todo!(),
            0xF0 => todo!(),
            0xF1 => todo!(),
            0xF2 => todo!(),
            0xF3 => todo!(),
            0xF4 => todo!(),
            0xF5 => todo!(),
            0xF6 => todo!(),
            0xF7 => todo!(),
            0xF8 => todo!(),
            0xF9 => todo!(),
            0xFA => todo!(),
            0xFB => todo!(),
            0xFC => todo!(),
            0xFD => todo!(),
            0xFE => todo!(),
            0xFF => todo!(),
        }

        if mnemonic.is_none() {
            panic!("Unresolved mnemonic during instruction execution!");
        }
        Instruction {
            mnemonic: mnemonic.unwrap(),
            operand: operand,
            bytes: bytes
        }
    }

    fn read8(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.ram[addr as usize % self.ram.len()]
        } else if addr < 0x4000 {
            // PPU Registers
            0
        } else if addr < 0x4018 {
            // IO Registers
            0
        } else if addr < 0x6000 {
            // Unused space
            0
        } else if addr < 0x8000 {
            // PRG RAM
            0
        } else {
            self.prg[addr as usize % self.prg.len()]
        }
    }

    fn write8(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 {
            let ram_size = self.ram.len();
            self.ram[addr as usize % ram_size] = value;
        } else if addr < 0x4000 {
            // PPU Registers
        } else if addr < 0x4018 {
            // IO Registers
        } else if addr < 0x6000 {
            // Unused space
        } else if addr < 0x8000 {
            // PRG RAM
        } else {
            // PRG ROM - writes forbidden
        }
    }

    fn read16(&self, addr: u16) -> u16 {
        self.read8(addr) as u16 | ((self.read8(addr + 1) as u16) << 8)
    }

    fn write16(&mut self, addr: u16, value: u16) {
        self.write8(addr, (value & 0xFF) as u8);
        self.write8(addr + 1, (value >> 8) as u8);
    }

    fn fetch_immediate8(&mut self) -> u8 {
        let value = self.read8(self.pc);
        self.pc += 1;
        value
    }

    fn fetch_immediate16(&mut self) -> u16 {
        let value = self.read16(self.pc);
        self.pc += 2;
        value
    }
}