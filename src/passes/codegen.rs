use repr::{self, BinaryOperation, Type, typed, unlinked, instruction_set};
use repr::typed::{Statement, Expression};
use repr::instruction_set::Instruction;

/*
TODO: Implement a smarter variable slot assignment
*/
static mut VAR_COUNT: i16 = 0;

pub fn pass(ast: &typed::Ast) -> unlinked::Module {
    let mut module = unlinked::Module::default();

    for (ident, function) in ast.functions.iter() {
        module.symbols.insert(ident.clone(), module.instructions.len() as i16);
    
        for statement in function.statements.iter() {
            generate_statement(&mut module, statement);
        }
    }

    return module;
}

fn generate_instruction(module: &mut unlinked::Module, instruction: Instruction) {
    module.instructions.push(instruction);
}

fn generate_statement(module: &mut unlinked::Module, statement: &Statement) {
    match statement {
        Statement::VariableAssignment(var, expr) => {
            generate_expression(module, expr);

            generate_instruction(module, Instruction::VAR_ASSIGN(unsafe { VAR_COUNT }));
            module.variable_slots.insert(var.identifier.clone(), unsafe { VAR_COUNT });
            unsafe { VAR_COUNT += 1};
        },
        Statement::ReturnStatement(expr) => {
            generate_expression(module, expr);
            generate_instruction(module, Instruction::RETURN);
        }
        _ => panic!("ICE [Missing Impl]: {:?}", statement)
    }
}

fn generate_expression(module: &mut unlinked::Module, expression: &Expression) {
    match expression {
        Expression::BinaryOperation(left, op, right, return_type) => {
            generate_expression(module, left);
            generate_expression(module, right);

            generate_instruction(module, match (op, return_type) {
                (BinaryOperation::Add, Type::INTEGER_32) => Instruction::ADD_I32,
                (BinaryOperation::Sub, Type::INTEGER_32) => Instruction::SUB_I32,
                (BinaryOperation::Mul, Type::INTEGER_32) => Instruction::MUL_I32,
                (BinaryOperation::Div, Type::INTEGER_32) => Instruction::DIV_I32,
                (BinaryOperation::Add, Type::FLOAT_32) => Instruction::ADD_F32,
                (BinaryOperation::Sub, Type::FLOAT_32) => Instruction::SUB_F32,
                (BinaryOperation::Mul, Type::FLOAT_32) => Instruction::MUL_F32,
                (BinaryOperation::Div, Type::FLOAT_32) => Instruction::DIV_F32,

                _ => panic!("ICE [Missing impl] for binary op: {:?}", (op, return_type))
            });
        },
        Expression::Constant(constant) => {
            generate_instruction(module, match constant.get_type() {
                Type::INTEGER_32 => Instruction::CONST_I32(module.constants.len() as i16),
                Type::FLOAT_32 => Instruction::CONST_F32(module.constants.len() as i16),
                _ => panic!()
            });
            module.constants.push(constant.cast_to_register());
        },
        Expression::VariableLookup(v) => {
            generate_instruction(module, Instruction::VAR_LOOKUP(*module.variable_slots.get(&v.identifier).unwrap()));
        },
        Expression::FunctionCall(ident, return_type) => {
            generate_instruction(module, Instruction::CALL(0));
            module.unresolved_symbols.push((ident.clone(), module.instructions.len() as i16 - 1));
        },
        _ => panic!("ICE [Missing Impl]: {:?}", expression)
    }
}