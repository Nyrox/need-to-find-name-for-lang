use super::{atoms::*, combine_prelude::*};
use super::expressions::*;

use crate::repr::{
	self,
	untyped::{self, Ast}
};

lang_item!(statement, untyped::Statement, {
    choice! (
		attempt(block().map(|b| untyped::Statement::BlockStatement(b)))
	)
});
