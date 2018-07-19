use ast::*;
use typed_ast::*;

pub fn check(ast: &Expr) -> TypedExpr {
	check_expr(ast)
}

fn check_expr(expr: &Expr) -> TypedExpr {
	match expr {
		Expr::BinaryOp(l, o, r) => {
			let left = check_expr(l);
			let right = check_expr(r);
			
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
		}
		_ => panic!("Internal compiler error: Missing Impl")
	}
}
