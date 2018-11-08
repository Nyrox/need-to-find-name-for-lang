use repr::instruction_set::Instruction;
use repr::{self, linked, unlinked};

pub fn pass(mut module: repr::unlinked::Module) -> repr::linked::Module {
    for (label, isp) in module.unresolved_symbols {
        match module.symbols.get(&label) {
            Some(symbol) => match module.instructions[isp as usize] {
                Instruction::CALL(_) => {
                    module.instructions[isp as usize] = Instruction::CALL(*symbol);
                }
                Instruction::COND_JMP(_) => {
                    module.instructions[isp as usize] = Instruction::COND_JMP(*symbol)
                }
                _ => unimplemented!(),
            },
            None => panic!("Symbol not found in module: {}", label),
        }
    }

    return repr::linked::Module {
        instructions: module.instructions,
        constants: module.constants,
        entry: *module.symbols.get("main").expect("no main"),
    };
}
