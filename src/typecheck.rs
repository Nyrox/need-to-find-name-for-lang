use ast::*;
use typed_ast::*;

pub fn check(ast: Expr) -> TypedExpr {
	check_expr(ast)
}

fn check_expr(expr: Expr) -> TypedExpr {
	match expr {
		Expr::BinaryOp(l, o, r) => {
			if l.is_constant() && r.is_constant() {
				return TypedExpr::TypedBinaryOp(Box::new(TypedBinaryOp {
					rType: Type::INTEGER_32,
					left: TypedExpr::TypedConstant(TypedConstant::INTEGER_32(l.unwrap_constant().unwrap_int() as i32)),
					right: TypedExpr::TypedConstant(TypedConstant::INTEGER_32(r.unwrap_constant().unwrap_int() as i32)),
					op: o
				}));
			}
			else {
				panic!();
			}
		}
		_ => panic!("Internal compiler error: Missing Impl")
	}
}