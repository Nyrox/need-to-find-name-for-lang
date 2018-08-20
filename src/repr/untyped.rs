use repr;

#[derive(Debug, Clone)]
pub struct Ast {
    pub declarations: Vec<TopLevelDeclaration>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    BinaryOperation(Box<Expression>, super::BinaryOperation, Box<Expression>),
    Constant(super::Value),
    VariableLookup(String),
    Cast(Box<Expression>, super::Type),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Clone, Debug)]
pub enum Statement {
    VariableDeclaration(String, super::Type),
    VariableAssignment(String, Expression),
    ExpressionStatement(Expression),
    ReturnStatement(Expression),
    PrintStatement(Expression),
}

#[derive(Clone, Debug)]
pub enum TopLevelDeclaration {
    FunctionDeclaration(String, Vec<Statement>, repr::Type, Vec<(String, repr::Type)>)
}

impl Expression {
    pub fn is_constant(&self) -> bool {
        match self {
            Expression::Constant(_) => true,
            _ => false
        }
    }

    pub fn unwrap_constant(&self) -> super::Value {
        match self {
            Expression::Constant(v) => *v,
            _ => panic!()
        }
    }
}