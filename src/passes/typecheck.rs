use repr::{self, untyped, typed};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Symbol {
    varstack_offset: i16,
    _type: repr::Type,
}

#[derive(Debug, Default)]
struct SymbolScope {
    symbols: HashMap<String, Symbol>,
    parent: Option<Box<SymbolScope>>,
    varstack_top: i16,
}

impl SymbolScope {
    pub fn new() -> SymbolScope {
        SymbolScope::default()
    }

    pub fn add_symbol(&mut self, ident: &str, _type: repr::Type) {
        self.symbols.insert(ident.to_owned(), Symbol {
            varstack_offset: self.varstack_top,
            _type
        });
        self.varstack_top += 1;
    }

    pub fn find(&self, ident: &str) -> Option<Symbol> {
        match self.symbols.get(ident) {
            Some(symbol) => return Some(*symbol),
            None => {
                if let Some(p) = &self.parent {
                    return p.find(ident);
                }
            }
        };

        return None;
    }
}

//
struct Context {
    typed_ast: typed::Ast,
    current_scope: SymbolScope
}

pub fn pass(mut untyped_ast: untyped::Ast) -> typed::Ast {
    let mut typed_ast = repr::typed::Ast::new();
    let mut context = Context { typed_ast, current_scope: SymbolScope::new() };

    for mut declaration in untyped_ast.declarations.iter_mut() {
        check_top_level_declaration(&mut context, &mut declaration);
    }

    return context.typed_ast;
}

fn check_top_level_declaration(context: &mut Context, declaration: &mut untyped::TopLevelDeclaration) {
    match declaration {
        untyped::TopLevelDeclaration::FunctionDeclaration(ident, block, rType, params) => {
            context.current_scope = SymbolScope::new();
            for (id, _type) in params.iter() {
                context.current_scope.add_symbol(id, *_type);
            }            

            // For functions we implicitly convert return expressions to return statements
            if let Some(expr) = &block.return_expr {
                block.statements.push(untyped::Statement::ReturnStatement(*expr.clone()));
            }

            let statements: Vec<_> = block.statements.iter().filter_map(|s| check_statement(context, s)).collect();



            println!("VarTable {}: {:#?}", ident, context.current_scope);

            context.typed_ast.functions.insert(ident.clone(), typed::FunctionDefintion {
                identifier: ident.clone(),
                statements: statements,
                return_type: *rType,
                parameters: params.clone()
            });
        }
    }
}

fn check_statement(context: &mut Context, statement: &untyped::Statement) -> Option<typed::Statement> {
    match statement {
        untyped::Statement::VariableDeclaration(s, t) => {
            context.current_scope.add_symbol(s, *t);
            return None;
        },
        untyped::Statement::VariableAssignment(s, e) => {
            return Some(
                typed::Statement::VariableAssignment(context.current_scope.find(s).unwrap().varstack_offset, check_expression(context, e))
            )
        },
        untyped::Statement::ReturnStatement(e) => {
            return Some(
                typed::Statement::ReturnStatement(check_expression(context, e))
            );
        },
        untyped::Statement::ExpressionStatement(e) => {
            return Some(
                typed::Statement::ExpressionStatement(check_expression(context, e))
            );
        },
        untyped::Statement::PrintStatement(e) => {
            return Some(
                typed::Statement::PrintStatement(check_expression(context, e))
            );
        }
        _ => panic!("ICE: Missing imp for {:?}", statement)
    }
}

fn check_block(context: &mut Context, block: &untyped::Block) -> typed::Block {
    typed::Block {
        statements: block.statements.iter().filter_map(|s| check_statement(context, s)).collect(),
        return_expr: block.return_expr.clone().map(|e| Box::new(check_expression(context, &e.clone()))),
    }
}

fn check_expression(context: &mut Context, expression: &untyped::Expression) -> typed::Expression {
    match expression {
        untyped::Expression::BinaryOperation(l, o, r) => {
            let left = check_expression(context, l);
            let right = check_expression(context, r);
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
        untyped::Expression::FunctionCall(ident, params) => {
            match context.typed_ast.functions.get(ident) {
                Some(f) => {
                    typed::Expression::FunctionCall(ident.clone(), f.return_type, params.iter().map(|e| check_expression(context, e)).collect())
                },
                None => panic!("{} is not a function\n\n{:?}", ident, context.typed_ast.functions),
            }
        },
        untyped::Expression::VariableLookup(ident) => {
            let var = context.current_scope.find(ident).unwrap();
            return typed::Expression::VariableLookup(var.varstack_offset, var._type);
        },
        untyped::Expression::Block(block) => {
            return typed::Expression::Block(typed::Block {
                statements: block.statements.iter().filter_map(|s| check_statement(context, s)).collect(),
                return_expr: block.return_expr.clone().map(|e| Box::new(check_expression(context, &*e))),
            });
        },
        untyped::Expression::Conditional(cond, consequent, alternate) => {
            return typed::Expression::Conditional(
                Box::new(check_expression(context, &cond.clone())),
                check_block(context, consequent),
                check_block(context, alternate),
            );
        },
        _ => panic!("ICE: Missing impl: {:?}", expression)
    }
}