use typed_ast::*;
use ast::*;
use std::mem;

#[derive(Debug)]
pub enum Instruction {
	ADD_I32,
	CONST_I32(i16)
}

#[derive(Debug, Default)]
pub struct Module {
	pub instructions: Vec<Instruction>,
	pub constants: Vec<i64>
}



pub fn gen(ast: TypedExpr) -> Module {
    let mut module = Module::default();

    gen_expr(&mut module, &ast);

    return module;
}

fn gen_expr(module: &mut Module, expr: &TypedExpr) {

    match expr {
        TypedExpr::TypedBinaryOp(op) => {
            gen_expr(module, &op.left);
            gen_expr(module, &op.right);

            module.instructions.push(match (op.op, op.rType) {
                (BinaryOp::Add, Type::INTEGER_32) => Instruction::ADD_I32,
                _ => panic!("Internal compiler error: Missing impl")
            });
        },
        TypedExpr::TypedConstant(c) => {
            let instruction = match c.get_type() {
                Type::INTEGER_32 => Instruction::CONST_I32(module.constants.len() as i16),
                _ => panic!()
            };
            module.instructions.push(instruction);
            module.constants.push(c.cast_to_register());
        }
    }
}
