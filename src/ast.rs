use typed_ast::{self, Type};

// We store constants in the largest possible format representable by the VM
#[derive(Clone, Debug, Copy)]
pub enum Value {
    Integer(i64),
    Decimal(f64),
}

impl Value {
    pub fn unwrap_int(&self) -> i64 {
        match self {
            Value::Integer(i) => *i,
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
pub enum Expr {
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    Constant(Value),
    VariableLookup(String),
    Cast(Box<Expr>, Type),
}

#[derive(Clone, Debug)]
pub enum Statement {
    VarDecl(String, typed_ast::Type),
    VarAssign(String, Expr),
    ExpressionStatement(Expr),
}

impl Expr {
    pub fn is_constant(&self) -> bool {
        match self {
            Expr::Constant(_) => true,
            _ => false
        }
    }

    pub fn unwrap_constant(&self) -> Value {
        match self {
            Expr::Constant(v) => *v,
            _ => panic!()
        }
    }
}