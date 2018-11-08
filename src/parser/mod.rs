use combine;
use combine::error::{ParseError, ParseResult};
use combine::parser::char::*;
use combine::parser::choice::*;
use combine::parser::sequence::*;
use combine::stream::state::State;
use combine::stream::{Positioned, Stream};
use combine::{attempt, between, choice, many, many1, parser, sep_by, Parser};

use crate::repr;
use crate::repr::untyped::{self, Ast};
use std::io::Read;

pub mod prelude {}

macro_rules! lang_item {
    ($id: ident, $t: ty, $e: expr) => {
        fn $id<I>() -> impl Parser<Input = I, Output = $t>
        where
            I: Stream<Item = char>,
            I::Error: ParseError<I::Item, I::Range, I::Position>,
        {
            $e
        }
    };
}

/*

*/
lang_item!(whitespace, (), { spaces() });

lang_item!(whitespace_opt, Option<()>, { optional(whitespace()) });

lang_item!(identifier, String, {
    letter()
        .or(char('_'))
        .and(many::<String, _>(alpha_num().or(char('_'))))
        .map(|(l, d)| format!("{}{}", l, d))
});

lang_item!(literal, repr::Value, {
    many1::<String, _>(digit())
        .and(optional(char('.').and(many1::<String, _>(digit()))))
        .map(|l| match l.1 {
            Some((d, s)) => repr::Value::Decimal(format!("{}{}{}", l.0, d, s).parse().unwrap()),
            None => repr::Value::Integer(l.0.parse().unwrap()),
        })
});

lang_item!(unit_type, repr::Type, {
    char('(')
        .and(spaces())
        .and(char(')'))
        .map(|_| repr::Type::UNIT)
});

lang_item!(type_annotation, repr::Type, {
    choice!(
        attempt(string("f32")).map(|_| repr::Type::FLOAT_32),
        string("f64").map(|_| repr::Type::FLOAT_64),
        attempt(string("i32")).map(|_| repr::Type::INTEGER_32),
        string("i64").map(|_| repr::Type::INTEGER_64),
        unit_type()
    )
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
