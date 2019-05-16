use super::{atoms::*, combine_prelude::*};
use super::expressions::*;

use crate::repr::{
	self,
	untyped::{self, Ast}
};

lang_item!(statement, untyped::Statement, {
    choice! (
		attempt(var_decl()),
		block_statement()
	)
});

lang_item!(block_statement, untyped::Statement, {
	block().map(|b| untyped::Statement::BlockStatement(b))
});

lang_item!(var_decl, untyped::Statement, {
	string("let")
		.and(whitespace())
		.with(identifier())
		.and(char(':').with(type_annotation()))
		.map(|(ident, type_a)| untyped::Statement::VariableDeclaration(ident, type_a))
});
