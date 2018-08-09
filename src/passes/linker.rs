use repr::{self, unlinked, linked};

pub fn pass(mut module: repr::unlinked::Module) -> repr::linked::Module {
    for (label, isp) in module.unresolved_symbols {
        match module.symbols.get(&label) {
            Some(symbol) => {
                module.instructions[isp as usize] = repr::instruction_set::Instruction::CALL(*symbol);
            }
            None => panic!("Symbol not found in module: {}", label)
        }
    }

    return repr::linked::Module {
        instructions: module.instructions,
        constants: module.constants,
        variable_slots: module.variable_slots,
        entry: *module.symbols.get("main").expect("no main"),
    };
}