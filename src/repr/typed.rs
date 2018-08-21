use std::collections::HashMap;
use repr::{self, Type};

use std::fmt::Debug;
use std::mem::transmute;

#[derive(Debug, Default)]
pub struct Ast {
	pub functions: HashMap<String, FunctionDefintion>,
	pub idents: HashMap<String, Variable>,
}

impl Ast {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_expr: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct FunctionDefintion {
	pub identifier: String,
	pub statements: Vec<Statement>,
	pub return_type: Type,
	pub parameters: Vec<(String, Type)>
}

#[derive(Debug)]
pub enum Statement {
	VariableAssignment(i16, Expression),
	ReturnStatement(Expression),
	ExpressionStatement(Expression),
	PrintStatement(Expression),
}

#[derive(Debug)]
pub enum Expression {
	BinaryOperation(Box<Expression>, repr::BinaryOperation, Box<Expression>, repr::Type),
	Constant(Box<ConstantValue>),
	FunctionCall(String, Type, Vec<Expression>),
	VariableLookup(i16, Type),
	Block(Block),
}

impl Expression {
	pub fn get_type(&self) -> repr::Type {
		match self {
			Expression::BinaryOperation(_,_,_, t) => *t,
			Expression::Constant(c) => c.get_type(),
			Expression::FunctionCall(_, t, _) => *t,
			Expression::VariableLookup(_, t) => *t,
			Expression::Block(block) => {
				if let Some(e) = &block.return_expr {
					return e.get_type();
				}
				return Type::UNIT;
			}
		}
	}
}

pub trait ConstantValue: Debug {
	fn get_type(&self) -> Type;
	fn cast_to_register(&self) -> i64;
}

impl ConstantValue for i32 {
	fn get_type(&self) -> Type {
		Type::INTEGER_32
	}
	fn cast_to_register(&self) -> i64 {
		*self as i64
	}
}

impl ConstantValue for f32 {
	fn get_type(&self) -> Type {
		Type::FLOAT_32
	}
	fn cast_to_register(&self) -> i64 {
		println!("{}", *self);
		unsafe { transmute::<f32, i32>(*self) as i64 }
	}
}

#[derive(Debug, Clone)]
pub struct Variable {
	pub identifier: String,
	pub v_type: Type,
}
