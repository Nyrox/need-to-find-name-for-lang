use super::instruction_set::{self, Instruction};
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Default)]
pub struct Module {
    pub instructions: Vec<instruction_set::Instruction>,
    pub constants: Vec<i64>,
    pub variable_slots: HashMap<String, i16>,
    // List of symbols that haven't yet been defined
    pub unresolved_symbols: Vec<(String, i16)>,
    // List of symbols this module defines
    pub symbols: HashMap<String, i16>,
}

impl Module {
    pub fn dump_assembly<T>(&self, buffer: &mut T)
    where
        T: Write,
    {
        // Invert symbol table
        let inverse_symbol_table: HashMap<i16, String> = self
            .symbols
            .iter()
            .map(|(s, i)| (i.clone(), s.clone()))
            .collect();

        write!(buffer, "Constant buffer:\n");
        write!(buffer, "{:?}\n\n", self.constants);

        write!(buffer, "Variable Slots:\n");
        write!(buffer, "{:?}\n\n", self.variable_slots);

        write!(buffer, "Instruction block:\n\n");

        for (i, e) in self.instructions.iter().enumerate() {
            if let Some(symbol) = inverse_symbol_table.get(&(i as i16)) {
                if symbol.starts_with("jmp") {
                    write!(buffer, "{}\n", symbol);
                } else {
                    write!(buffer, "\n{}\n", symbol);
                }
            }

            match e {
                Instruction::CALL(_) => {
                    for (name, index) in self.unresolved_symbols.iter() {
                        if *index == i as i16 {
                            write!(buffer, "\tCALL {}\n", name);
                        }
                    }
                }
                Instruction::COND_JMP(_) => {
                    for (name, index) in self.unresolved_symbols.iter() {
                        if *index == i as i16 {
                            write!(buffer, "\tCOND_JMP {}\n", name);
                        }
                    }
                }
                _ => {
                    write!(buffer, "\t{:?}\n", e);
                }
            }
        }
    }
}
