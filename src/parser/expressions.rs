use super::atoms::*;
use super::combine_prelude::*;
use super::statements::*;

use crate::repr::{
    self,
    untyped::{self, Ast},
};

lang_item!(block, untyped::Block, {
	let statements = many::<Vec<untyped::Statement>, _> (statement());

	let skip_spaces = || spaces().silent();
	let lex_char = 	|c| char(c).skip(skip_spaces());

	between(lex_char('{'), lex_char('}'), statements.map(|s| untyped::Block::empty()))
});
