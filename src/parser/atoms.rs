use super::combine_prelude::*;

use crate::repr::{
    self,
    untyped::{self, Ast},
};

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
