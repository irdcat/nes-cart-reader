use std::collections::HashMap;

use super::instructions::Instruction;

#[derive(PartialEq)]
pub struct PrgData {
    pub instructions: HashMap<u16, Instruction>,
}
