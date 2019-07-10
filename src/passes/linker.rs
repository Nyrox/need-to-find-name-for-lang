use repr::{self};
use repr::instruction_set::Instruction;

pub fn pass(mut module: repr::unlinked::Module) -> repr::linked::Module {
    for (label, isp) in module.unresolved_symbols {
        match module.symbols.get(&label) {
            Some(symbol) => {
                match module.instructions[isp as usize] {
                    Instruction::Call(_) => {
                        module.instructions[isp as usize] = Instruction::Call(*symbol);
                    },
                    Instruction::CondJmp(_) => {
                        module.instructions[isp as usize] = Instruction::CondJmp(*symbol)
                    }
                    _ => unimplemented!()
                }
            }
            None => panic!("Symbol not found in module: {}", label)
        }
    }

    return repr::linked::Module {
        instructions: module.instructions,
        constants: module.constants,
        entry: *module.symbols.get("main").expect("no main"),
    };
}
