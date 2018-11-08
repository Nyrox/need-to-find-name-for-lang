use repr;

#[derive(Debug, Clone)]
pub struct Ast {
    pub declarations: Vec<TopLevelDeclaration>,
}

impl Block {
    pub fn new(s: Vec<Statement>, e: Option<Box<Expression>>) -> Block {
        Block {
            statements: s,
            return_expr: e,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_expr: Option<Box<Expression>>,
}

impl Block {
    pub fn empty() -> Block {
        Block {
            statements: Vec::new(),
            return_expr: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    BinaryOperation(Box<Expression>, super::BinaryOperation, Box<Expression>),
    Constant(super::Value),
    VariableLookup(String),
    Cast(Box<Expression>, super::Type),
    FunctionCall(String, Vec<Expression>),
    Block(Block),
    Conditional(Box<Expression>, Block, Block),
}

#[derive(Clone, Debug)]
pub enum Statement {
    VariableDeclaration(String, super::Type),
    VariableAssignment(String, Expression),
    ExpressionStatement(Expression),
    ReturnStatement(Expression),
    PrintStatement(Expression),
    BlockStatement(Block),
}

#[derive(Clone, Debug)]
pub enum TopLevelDeclaration {
    FunctionDeclaration(String, Block, repr::Type, Vec<(String, repr::Type)>),
}

impl Expression {
    pub fn is_constant(&self) -> bool {
        match self {
            Expression::Constant(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_constant(&self) -> super::Value {
        match self {
            Expression::Constant(v) => *v,
            _ => panic!(),
        }
    }
}
