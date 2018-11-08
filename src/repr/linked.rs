use super::instruction_set;

#[derive(Debug, Default)]
pub struct Module {
    pub instructions: Vec<instruction_set::Instruction>,
    pub constants: Vec<i64>,
    pub entry: i16,
}
