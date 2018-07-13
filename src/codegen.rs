use typed_ast::*;
use ast::*;
use std::mem;

#[derive(Debug, Default, Clone)]
pub struct Module {
    pub instructions: Vec<u8>,
    pub constant: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    ADD_I32,

    CONST_I32
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
                (BinaryOp::Add, Type::INTEGER_32) => OpCode::ADD_I32 as u8,
                _ => panic!("Internal compiler error: Missing impl")
            });
        },
        TypedExpr::TypedConstant(c) => {
            match c {
                TypedConstant::INTEGER_32(i) => {
                    module.instructions.push(OpCode::CONST_I32 as u8);
                    module.instructions.push(module.constant.len() as u8);

                    let bytes: [u8; 4] = unsafe { mem::transmute(*i) };
                    for b in bytes.iter() { module.constant.push(*b); }
                },
                _ => panic!("Internal compiler error: Missing Impl")
            }
        }
    }
}
