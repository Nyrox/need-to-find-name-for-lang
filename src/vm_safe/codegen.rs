use typed_ast::*;
use ast::*;
use std::mem;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
	ADD_I32,
    SUB_I32,
    MUL_I32,
    DIV_I32,

    VAR_LOOKUP(i16),
    VAR_ASSIGN(i16),
	CONST_I32(i16),
}

#[derive(Debug, Default)]
pub struct Module {
	pub instructions: Vec<Instruction>,
	pub constants: Vec<i64>,
    pub vars: HashMap<String, i16>,
}

static mut VAR_COUNT: i16 = 0;

pub fn gen(ast: &Vec<TypedStatement>) -> Module {
    let mut module = Module::default();

    for i in ast {
        gen_statement(&mut module, i);
    }

    return module;
}

fn gen_statement(module: &mut Module, statement: &TypedStatement) {
    match statement {
        TypedStatement::VarAssign(i, e) => {
            assert!(i.vType == e.get_type());
            gen_expr(module, e);
            module.instructions.push(Instruction::VAR_ASSIGN(unsafe { VAR_COUNT }));
            module.vars.insert(i.ident.to_owned(), unsafe { VAR_COUNT });
            unsafe { VAR_COUNT += 1 };
        },
        _ => panic!("Missing impl")
    }
}

fn gen_expr(module: &mut Module, expr: &TypedExpr) {

    match expr {
        TypedExpr::TypedBinaryOp(op) => {
            gen_expr(module, &op.left);
            gen_expr(module, &op.right);

            module.instructions.push(match (op.op, op.rType) {
                (BinaryOp::Add, Type::INTEGER_32) => Instruction::ADD_I32,
                (BinaryOp::Sub, Type::INTEGER_32) => Instruction::SUB_I32,
                (BinaryOp::Mul, Type::INTEGER_32) => Instruction::MUL_I32,
                (BinaryOp::Div, Type::INTEGER_32) => Instruction::DIV_I32,
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
        },
        TypedExpr::TypedVarLookup(v) => {            
            module.instructions.push(Instruction::VAR_LOOKUP(*module.vars.get(&v.ident).unwrap()));
        }
    }
}
