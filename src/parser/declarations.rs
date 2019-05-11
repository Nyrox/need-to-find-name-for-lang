use super::atoms::*;
use super::combine_prelude::*;

use crate::repr::{
    self,
    untyped::{self, Ast},
};

lang_item!(top_level_declaration, untyped::TopLevelDeclaration, {
    choice!(function_declaration())
});

lang_item!(function_declaration, untyped::TopLevelDeclaration, {
    string("fn")
        .and(whitespace())
        .with(identifier())
        .and(
            char('(')
                .and(whitespace_opt())
                .and(char(')'))
                .and(whitespace_opt())
                .with(
                    optional(string("->").and(whitespace_opt()).with(type_annotation())).map(|r| {
                        match r {
                            Some(t) => t,
                            None => repr::Type::UNIT,
                        }
                    }),
                ),
        )
        .map(|(ident, rType)| {
            println!("{:?}, {:?}", ident, rType);

            repr::untyped::TopLevelDeclaration::FunctionDeclaration(
                ident,
                repr::untyped::Block::empty(),
                rType,
                Vec::new(),
            )
        })
});
