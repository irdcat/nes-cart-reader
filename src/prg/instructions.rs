use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Index {
    X,
    Y,
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stringified = match self {
            Index::X => "X",
            Index::Y => "Y",
        };
        write!(f, "{}", stringified)
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Operand {
    Immediate { value: u8 },
    ZeroPage { address: u8 },
    ZeroPageIndexed { address: u8, index: Index },
    Absolute { address: u16 },
    AbsoluteIndexed { address: u16, index: Index },
    Indirect { address: u16 },
    PreIndexedIndirect { address: u8 },
    PostIndexedIndirect { address: u8 },
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operand::*;
        match self {
            Immediate { value } => write!(f, "#${:02X}", value),
            ZeroPage { address } => write!(f, "${:02X}", address),
            ZeroPageIndexed { address, index } => write!(f, "${:02X}, {}", address, index),
            Absolute { address } => write!(f, "${:04X}", address),
            AbsoluteIndexed { address, index } => write!(f, "${:04X}, {}", address, index),
            Indirect { address } => write!(f, "(${:04X})", address),
            PreIndexedIndirect { address } => write!(f, "(${:02X}, X)", address),
            PostIndexedIndirect { address } => write!(f, "(${:02X}), Y", address),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Mnemonic {
    Lda,
    Ldx,
    Ldy,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,

    Dec,
    Dex,
    Dey,
    Inc,
    Inx,
    Iny,

    Clc,
    Cld,
    Cli,
    Clv,
    Sec,
    Sed,
    Sei,

    Pha,
    Php,
    Pla,
    Plp,

    Jmp,
    Jsr,
    Rts,

    Adc,
    Sbc,

    And,
    Eor,
    Ora,

    Bcc,
    Bcs,
    Beq,
    Bmi,
    Bne,
    Bpl,
    Bvc,
    Bvs,

    Asl,
    Lsr,
    Rol,
    Ror,

    Cmp,
    Cpx,
    Cpy,

    Brk,
    Rti,

    Nop,
    Bit,

    Alr,
    Anc,
    Xaa,
    Arr,
    Dcp,
    Isc,
    Las,
    Lax,
    Lxa,
    Rla,
    Rra,
    Sax,
    Axs,
    Ahx,
    Shx,
    Shy,
    Slo,
    Sre,
    Tas,
    Stp,
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Mnemonic::*;
        let mnemonic_string = match self {
            Lda => "LDA",
            Ldx => "LDX",
            Ldy => "LDY",
            Sta => "STA",
            Stx => "STX",
            Sty => "STY",
            Tax => "TAX",
            Tay => "TAY",
            Tsx => "TSX",
            Txa => "TXA",
            Txs => "TXS",
            Tya => "TYA",
            Dec => "DEC",
            Dex => "DEX",
            Dey => "DEY",
            Inc => "INC",
            Inx => "INX",
            Iny => "INY",
            Clc => "CLC",
            Cld => "CLD",
            Cli => "CLI",
            Clv => "CLV",
            Sec => "SEC",
            Sed => "SED",
            Sei => "SEI",
            Pha => "PHA",
            Php => "PHP",
            Pla => "PLA",
            Plp => "PLP",
            Jmp => "JMP",
            Jsr => "JSR",
            Rts => "RTS",
            Adc => "ADC",
            Sbc => "SBC",
            And => "AND",
            Eor => "EOR",
            Ora => "ORA",
            Bcc => "BCC",
            Bcs => "BCS",
            Beq => "BEQ",
            Bmi => "BMI",
            Bne => "BNE",
            Bpl => "BPL",
            Bvc => "BVC",
            Bvs => "BVS",
            Asl => "ASL",
            Lsr => "LSR",
            Rol => "ROL",
            Ror => "ROR",
            Cmp => "CMP",
            Cpx => "CPX",
            Cpy => "CPY",
            Brk => "BRK",
            Rti => "RTI",
            Nop => "NOP",
            Bit => "BIT",
            Alr => "ALR",
            Anc => "ANC",
            Xaa => "XAA",
            Arr => "ARR",
            Dcp => "DCP",
            Isc => "ISC",
            Las => "LAS",
            Lax => "LAX",
            Lxa => "LXA",
            Rla => "RLA",
            Rra => "RRA",
            Sax => "SAX",
            Axs => "AXS",
            Ahx => "AHX",
            Shx => "SHX",
            Shy => "SHY",
            Slo => "SLO",
            Sre => "SRE",
            Tas => "TAS",
            Stp => "STP",
        };
        write!(f, "{}", mnemonic_string)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operand: Option<Operand>,
    pub bytes: Vec<u8>,
}
