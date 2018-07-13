use ast::*;


#[derive(Clone, Debug, Copy)]
pub enum Type {
    INTEGER_32, INTEGER_64,
    FLOAT_32, FLOAT_64,
}

#[derive(Clone, Debug, Copy)]
pub enum TypedConstant {
    INTEGER_32(i32),
    INTEGER_64(i64),
    FLOAT_32(f32),
    FLOAT_64(f64),
}

#[derive(Clone, Debug)]
pub struct TypedBinaryOp {
    pub rType: Type,
    pub left: TypedExpr,
    pub right: TypedExpr,
    pub op: BinaryOp,
}

#[derive(Clone, Debug)]
pub enum TypedExpr {
    TypedBinaryOp(Box<TypedBinaryOp>),
    TypedConstant(TypedConstant),
}