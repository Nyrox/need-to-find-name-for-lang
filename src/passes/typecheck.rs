use repr::{self, untyped, typed};


pub fn pass(ast: untyped::Ast) -> typed::Ast {
    let mut typed_ast = repr::typed::Ast::new();
    
    for declaration in ast.declarations {
        check_top_level_declaration(&mut typed_ast, &declaration);
    }

    return typed_ast;
}

fn check_top_level_declaration(ast: &mut typed::Ast, declaration: &untyped::TopLevelDeclaration) {
    match declaration {
        untyped::TopLevelDeclaration::FunctionDeclaration(ident, block, rType) => {
            let statements = block.iter().filter_map(|s| check_statement(ast, s)).collect();

            ast.functions.insert(ident.clone(), typed::FunctionDefintion {
                identifier: ident.clone(),
                statements: statements,
                return_type: *rType,
            });
        }
    }
}

fn check_statement(ast: &mut typed::Ast, statement: &untyped::Statement) -> Option<typed::Statement> {
    match statement {
        untyped::Statement::VariableDeclaration(s, t) => {
            ast.idents.insert(s.clone(), typed::Variable {
                identifier: s.clone(),
                v_type: *t,
            });
            return None;
        },
        untyped::Statement::VariableAssignment(s, e) => {
            return Some(
                typed::Statement::VariableAssignment(typed::Variable {
                    identifier: s.clone(),
                    v_type: ast.idents.get(s).unwrap().v_type,
                }, check_expression(ast, e))
            )
        },
        untyped::Statement::ReturnStatement(e) => {
            return Some(
                typed::Statement::ReturnStatement(check_expression(ast, e))
            );
        }
        _ => panic!("ICE: Missing imp for {:?}", statement)
    }
}

fn check_expression(ast: &mut typed::Ast, expression: &untyped::Expression) -> typed::Expression {
    match expression {
        untyped::Expression::BinaryOperation(l, o, r) => {
            let left = check_expression(ast, l);
            let right = check_expression(ast, r);
            let _type = left.get_type();

            assert!(left.get_type() == right.get_type());

            return typed::Expression::BinaryOperation(
                Box::new(left), *o, Box::new(right),
                _type
            );
        },
        untyped::Expression::Constant(value) => {
            return match value {
                repr::Value::Integer(i) => typed::Expression::Constant(Box::new(*i as i32)),
                repr::Value::Decimal(f) => typed::Expression::Constant(Box::new(*f as f32))
            }
        },
        untyped::Expression::FunctionCall(ident) => {
            match ast.functions.get(ident) {
                Some(f) => typed::Expression::FunctionCall(ident.clone(), f.return_type),
                None => panic!("{} is not a function\n\n{:?}", ident, ast.functions),
            }
        },
        untyped::Expression::VariableLookup(ident) => {
            let var = ast.idents.get(ident).expect(&format!("Unrecognized identifier: {:?}", ident));
            return typed::Expression::VariableLookup(var.clone());
        }
        _ => panic!("ICE: Missing impl: {:?}", expression)
    }
}