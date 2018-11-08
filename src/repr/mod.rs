pub mod instruction_set;
pub mod linked;
pub mod typed;
pub mod unlinked;
pub mod untyped;

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    More,
    LessEqual,
    MoreEqual,
}

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Integer(i64),
    Decimal(f64),
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Type {
    UNIT,
    INTEGER_32,
    INTEGER_64,
    FLOAT_32,
    FLOAT_64,
}
