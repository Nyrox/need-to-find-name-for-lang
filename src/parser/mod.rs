mod atoms;
mod expressions;

mod combine_prelude {
    use combine;
    pub use combine::error::{ParseError, ParseResult};
    pub use combine::parser::char::*;
    pub use combine::parser::choice::*;
    pub use combine::parser::sequence::*;
    pub use combine::stream::state::State;
    pub use combine::stream::{Positioned, Stream};
    pub use combine::{attempt, between, choice, many, many1, parser, sep_by, Parser};

    #[macro_export]
    macro_rules! lang_item {
        ($id: ident, $t: ty, $e: expr) => {
            pub fn $id<I>() -> impl Parser<Input = I, Output = $t>
            where
                I: Stream<Item = char>,
                I::Error: ParseError<I::Item, I::Range, I::Position>,
            {
                $e
            }
        };
    }
    pub use lang_item;
}

use self::atoms::*;
use self::combine_prelude::*;

use crate::repr;
use crate::repr::untyped::{self, Ast};
use std::io::Read;

pub mod prelude {}

/*

*/

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


/// Main entry point for the parser
pub fn parse_lang(input: &str) -> untyped::Ast {
    let id = identifier().easy_parse(State::new(input));
    let val = literal().easy_parse(State::new("123"));
    let type_annotation = type_annotation().easy_parse(State::new("(  )"));
    let fndecl = function_declaration().easy_parse(State::new("fn  foo() -> i64 {}"));

    println!("{:?}", id);
    println!("{:?}", val);
    println!("{:?}", type_annotation);
    println!("{:?}", fndecl);

    Ast {
        declarations: Vec::new(),
    }
}
