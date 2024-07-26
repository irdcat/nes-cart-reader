use std::{collections::HashMap, error, fmt};

use super::instructions::Instruction;

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
    // TODO: Use BTreeMap instead of HashMap
    pub instructions: HashMap<u16, Instruction>,
}

impl PrgData {
    pub fn parse(_prg_data: Vec<u8>) -> Result<PrgData, InvalidPrgDataError> {
        Ok(PrgData {
            instructions: HashMap::new(),
        })
    }
}
