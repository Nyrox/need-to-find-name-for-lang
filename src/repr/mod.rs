pub mod untyped;
pub mod typed;
pub mod unlinked;
pub mod linked;
pub mod instruction_set;


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
    Unit,
    Integer32, Integer64,
    Float32, Float64,
}