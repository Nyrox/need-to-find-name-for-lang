//! Contains the handwritten parser
//! This library uses combine as a parser combinator to provide efficient
//! and easy parsing.

pub mod atoms;
pub mod declarations;
pub mod expressions;
pub mod statements;

/// Internal usage only, exposes combine types and the lang_item macro
#[doc(hidden)]
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
	pub use boxed_lang_item;
}

use self::atoms::*;
use self::combine_prelude::*;
use self::declarations::*;
use self::expressions::*;

use crate::repr;
use crate::repr::untyped::{self, Ast};
use std::io::Read;

pub mod prelude {}

/*

*/

/// Main entry point for the parser
pub fn parse_lang(input: &str) -> untyped::Ast {
    let id = identifier().easy_parse(State::new(input));
    let val = literal().easy_parse(State::new("123"));
    let type_annotation = type_annotation().easy_parse(State::new("(  )"));
    let fndecl = top_level_declaration().easy_parse(State::new("fn  foo() -> i64"));

    println!("{:?}", id);
    println!("{:?}", val);
    println!("{:?}", type_annotation);
    println!("{:?}", fndecl);

	let block = block().easy_parse(State::new("{\n}"));

    Ast {
        declarations: Vec::new(),
    }
}
