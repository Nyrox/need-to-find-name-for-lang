use ast::*;
use std::fmt::Debug;
use std::mem::transmute;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Type {
    INTEGER_32, INTEGER_64,
    FLOAT_32, FLOAT_64,
}

#[derive(Clone, Debug)]
pub struct TypedVar {
	pub vType: Type,
	pub ident: String
}

pub trait ConstValue: Debug {
	fn get_type(&self) -> Type;
	fn cast_to_register(&self) -> i64;
}

impl ConstValue for i32 {
	fn get_type(&self) -> Type {
		Type::INTEGER_32
	}
	fn cast_to_register(&self) -> i64 {
		*self as i64
	}
}

impl ConstValue for f32 {
	fn get_type(&self) -> Type {
		Type::FLOAT_32
	}
	fn cast_to_register(&self) -> i64 {
		unsafe { transmute::<f32, i32>(*self) as i64 }
	}
}

#[derive(Debug)]
pub struct TypedBinaryOp {
    pub rType: Type,
    pub left: TypedExpr,
    pub right: TypedExpr,
    pub op: BinaryOp,
}

impl TypedBinaryOp {
	pub fn get_type(&self) -> Type {
		return self.rType;
	}
}

#[derive(Debug)]
pub enum TypedStatement {
	VarDecl(TypedVar),
    VarAssign(TypedVar, TypedExpr),
    ExpressionStatement(TypedExpr),
}

#[derive(Debug)]
pub enum TypedExpr {
    TypedBinaryOp(Box<TypedBinaryOp>),
    TypedConstant(Box<ConstValue>),
	TypedVarLookup(TypedVar)
}

impl TypedExpr {
	pub fn is_constant(&self) -> bool {
		match self {
			TypedExpr::TypedConstant(_) => true,
			_ => false
		}
	}

	pub fn get_type(&self) -> Type {
		match self {
			TypedExpr::TypedConstant(c) => c.get_type(),
			TypedExpr::TypedBinaryOp(op) => op.get_type(),
			TypedExpr::TypedVarLookup(var) => var.vType
		}
	}
}
