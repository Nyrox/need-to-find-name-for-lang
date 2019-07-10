use rand;

use repr::{BinaryOperation, Type, typed, unlinked};
use repr::typed::{Statement, Expression, Block};
use repr::instruction_set::Instruction;

/*
TODO: Implement a smarter variable slot assignment
*/
// static mut VAR_COUNT: i16 = 0;

pub fn pass(ast: &typed::Ast) -> unlinked::Module {
    let mut module = unlinked::Module::default();

    for (ident, function) in ast.functions.iter() {
        module.symbols.insert(ident.clone(), module.instructions.len() as i16);

        for (i, _e) in function.parameters.iter().enumerate() {
            generate_instruction(&mut module, Instruction::VarAssign(i as i16));
        }

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
        Statement::VariableAssignment(stack_offset, expr) => {
            generate_expression(module, expr);

            generate_instruction(module, Instruction::VarAssign(*stack_offset));
        },
        Statement::ReturnStatement(expr) => {
            generate_expression(module, expr);
            generate_instruction(module, Instruction::Return);
        },
        Statement::ExpressionStatement(expr) => {
            generate_expression(module, expr);
            generate_instruction(module, Instruction::PopStack);
        },
        Statement::PrintStatement(expr) => {
            generate_expression(module, expr);
            generate_instruction(module, Instruction::Print(expr.get_type()));
        }
    }
}

fn generate_expression(module: &mut unlinked::Module, expression: &Expression) {
    match expression {
        Expression::BinaryOperation(left, op, right, return_type) => {
            generate_expression(module, left);
            generate_expression(module, right);

            generate_instruction(module, match (op, return_type) {
                (BinaryOperation::Add, Type::Integer32) => Instruction::AddI32,
                (BinaryOperation::Sub, Type::Integer32) => Instruction::SubI32,
                (BinaryOperation::Mul, Type::Integer32) => Instruction::MulI32,
                (BinaryOperation::Div, Type::Integer32) => Instruction::DivI32,
                (BinaryOperation::Less, Type::Integer32) => Instruction::LessI32,
                (BinaryOperation::Add, Type::Float32) => Instruction::AddF32,
                (BinaryOperation::Sub, Type::Float32) => Instruction::SubF32,
                (BinaryOperation::Mul, Type::Float32) => Instruction::MulF32,
                (BinaryOperation::Div, Type::Float32) => Instruction::DivF32,
                (BinaryOperation::Less, Type::Float32) => Instruction::LessF32,

                _ => panic!("ICE [Missing impl] for binary op: {:?}", (op, return_type))
            });
        },
        Expression::Constant(constant) => {
            generate_instruction(module, match constant.get_type() {
                Type::Integer32 => Instruction::ConstI32(module.constants.len() as i16),
                Type::Float32 => Instruction::ConstF32(module.constants.len() as i16),
                _ => panic!()
            });
            module.constants.push(constant.cast_to_register());
        },
        Expression::VariableLookup(slot, _) => {
            generate_instruction(module, Instruction::VarLookup(*slot));
        },
        Expression::FunctionCall(ident, _return_type, params) => {
            // push arguments
            for p in params.into_iter().rev() {
                generate_expression(module, p);
            }

            generate_instruction(module, Instruction::Call(0));
            module.unresolved_symbols.push((ident.clone(), module.instructions.len() as i16 - 1));
        },
        Expression::Block(block) => {
            generate_block(module, block, true);
        },
        Expression::Conditional(cond, ref consequent, ref alternate) => {
            generate_expression(module, cond);

            let cond_ident = rand::random::<u16>() as i16;
            let cond_ident_fmt = format!("jmp_{}_else", cond_ident);

            generate_instruction(module, Instruction::CondJmp(0));
            module.unresolved_symbols.push((cond_ident_fmt.clone(), module.instructions.len() as i16 - 1));

            generate_block(module, consequent, true);

            module.symbols.insert(cond_ident_fmt, module.instructions.len() as i16);

            generate_block(module, alternate, false);
        }
    }
}

fn generate_block(module: &mut unlinked::Module, block: &Block, force_return: bool) {
    for s in block.statements.iter() {
        generate_statement(module, s);
    }

    if let Some(expr) = &block.return_expr {
        generate_expression(module, expr);
    }
    else if force_return {
        generate_instruction(module, Instruction::PushVoid);
    }
}
