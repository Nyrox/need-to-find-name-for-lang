use super::instruction_set;
use std::collections::HashMap;


#[derive(Debug, Default)]
pub struct Module {
    pub instructions: Vec<instruction_set::Instruction>,
    pub constants: Vec<i64>,
    pub variable_slots: HashMap<String, i16>,
    pub entry: i16,
}