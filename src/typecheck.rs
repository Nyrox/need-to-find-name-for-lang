use ast::*;
use typed_ast::*;
use std::collections::HashMap;

fn lookup_type(t: &str) -> Option<Type> {
	match t {
		"i32" => Some(Type::INTEGER_32),
		_ => panic!("Unkown type: {}", t)
	}
}

#[derive(Debug, Default)]
struct Context {
	vars: HashMap<String, TypedVar>
}

impl Context {
	pub fn new() -> Context { Self::default() }

	pub fn lookup_ident(&self, ident: &str) -> Option<&TypedVar> {
		self.vars.get(ident)
	}

	pub fn insert_ident(&mut self, ident: &str, vType: Type) {
		self.vars.insert(ident.to_owned(), TypedVar { ident: ident.to_owned(), vType });
	}
}

pub fn check(ast: &Vec<Statement>) -> Vec<TypedStatement> {
	let mut typed_ast = Vec::new();
	let mut context = Context::new();

	for statement in ast {
		let _st = check_statement(&mut context, statement);
		match _st {
			Some(st) => typed_ast.push(st),
			None => {}
		}
	}

	return typed_ast;
}

fn check_statement(context: &mut Context, statement: &Statement) -> Option<TypedStatement> {
	match statement {
		Statement::VarDecl(s, t) => { 
			context.insert_ident(s, *t);
			None
		},
		Statement::VarAssign(s, e) => Some(TypedStatement::VarAssign(TypedVar {vType: context.lookup_ident(s).unwrap().vType, ident: s.clone()}, check_expr(context, e))),
		Statement::ExpressionStatement(e) => Some(TypedStatement::ExpressionStatement(check_expr(context, e))),
	}
}

fn check_expr(context: &Context, expr: &Expr) -> TypedExpr {
	match expr {
		Expr::BinaryOp(l, o, r) => {
			let left = check_expr(context, l);
			let right = check_expr(context, r);
			
			assert!(left.get_type() == right.get_type());

			return TypedExpr::TypedBinaryOp(Box::new(TypedBinaryOp {
				rType: left.get_type(),
				left,
				right,
				op: *o
			}));
		}
		Expr::Constant(val) => {
			return match val {
				Value::Integer(i) => TypedExpr::TypedConstant(Box::new(*i as i32)),
				Value::Decimal(f) => TypedExpr::TypedConstant(Box::new(*f as f32)),
			}	
		},
		Expr::VariableLookup(id) => {
			let var = context.lookup_ident(id).expect(&format!("Unrecognized identifier {}", id));
			return TypedExpr::TypedVarLookup(var.clone());
		},
		Expr::Cast(e, t) => {
			return TypedExpr::TypedCast(Box::new(check_expr(context, e)), *t);
		}
		_ => panic!("Internal compiler error: Missing Impl")
	}
}
