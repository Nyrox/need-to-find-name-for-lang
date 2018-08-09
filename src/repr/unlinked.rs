use super::instruction_set;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Module {
    pub instructions: Vec<instruction_set::Instruction>,
    pub constants: Vec<i64>,
    pub variable_slots: HashMap<String, i16>,
    // List of symbols that haven't yet been defined
    pub unresolved_symbols: Vec<(String, i16)>,
    // List of symbols this module defines
    pub symbols: HashMap<String, i16>
}