// auto-generated: "lalrpop 0.15.2"
// sha256: 24657bcb4bc7e820f7267438cd451c78e19be7d7bab2578298f3b9edd294f
use ast;
use typed_ast::{self, Type};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Ast {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ast;
    use typed_ast::{self, Type};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Vec<ast::Statement>),
        Variant2(ast::Value),
        Variant3(ast::Expr),
        Variant4(String),
        Variant5(ast::Statement),
        Variant6(::std::vec::Vec<ast::Statement>),
        Variant7(typed_ast::Type),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 15, 16,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, -16, -16, -16, -16, -16, 0, -16, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 17, 18, 0, 0, 19, 0, 0, 0, 0, 0, 0,
        // State 4
        0, -6, 20, -6, -6, 21, 0, -6, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, -17, -17, -17, -17, 0, -17, 22, 0, 0, 0, 0, 0,
        // State 6
        -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14, -14,
        // State 7
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 15, 16,
        // State 8
        0, -9, -9, -9, -9, -9, 0, -9, 0, 0, 0, 0, 0, 0,
        // State 9
        -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, -11, -11, -11,
        // State 10
        -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12, -12,
        // State 11
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 16,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0,
        // State 13
        0, -10, -10, -10, -10, -10, -10, -10, -10, 0, 0, 0, 0, 0,
        // State 14
        0, -3, -3, -3, -3, -3, 0, -3, 0, 0, 0, 0, 0, 0,
        // State 15
        0, -2, -2, -2, -2, -2, 0, -2, 0, 0, 0, 0, 0, 0,
        // State 16
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 16,
        // State 17
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 16,
        // State 18
        -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13, -13,
        // State 19
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 16,
        // State 20
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 16,
        // State 21
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 16,
        // State 22
        -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15, -15,
        // State 23
        0, 32, 0, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -17, -17, -17, -17, -17, 0, -17, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -4, 20, -4, -4, 21, 0, -4, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -5, 20, -5, -5, 21, 0, -5, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -7, -7, -7, -7, -7, 0, -7, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -8, -8, -8, -8, -8, 0, -8, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 17, 18, 0, 0, 34, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -18, -18, -18, -18, -18, 0, -18, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 33
        -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20, -20,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0,
        // State 36
        -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, -21, -21,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -22,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        -14,
        // State 7
        -1,
        // State 8
        0,
        // State 9
        -11,
        // State 10
        -12,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        -13,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -15,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        -20,
        // State 34
        0,
        // State 35
        0,
        // State 36
        -21,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        2, 3, 4, 5, 6, 7, 8, 9, 0, 10, 11, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 3, 4, 5, 6, 23, 0, 9, 0, 10, 11, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 3, 24, 5, 25, 0, 0, 9, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 3, 0, 27, 25, 0, 0, 9, 0, 0, 0, 0,
        // State 17
        0, 3, 0, 28, 25, 0, 0, 9, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 3, 0, 0, 25, 0, 0, 29, 0, 0, 0, 0,
        // State 20
        0, 3, 0, 0, 25, 0, 0, 30, 0, 0, 0, 0,
        // State 21
        0, 3, 31, 5, 25, 0, 0, 9, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""=""###,
            r###""i32""###,
            r###""let""###,
            r###"r#"\\w[\\w\\d]*"#"###,
            r###"_decimal"###,
            r###"_integer"###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct AstParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl AstParser {
        pub fn new() -> AstParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            AstParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Vec<ast::Statement>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(e),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token(1, _) if true => 0,
                    Token(2, _) if true => 1,
                    Token(3, _) if true => 2,
                    Token(4, _) if true => 3,
                    Token(5, _) if true => 4,
                    Token(6, _) if true => 5,
                    Token(7, _) if true => 6,
                    Token(8, _) if true => 7,
                    Token(9, _) if true => 8,
                    Token(10, _) if true => 9,
                    Token(11, _) if true => 10,
                    Token(0, _) if true => 11,
                    Token(12, _) if true => 12,
                    Token(13, _) if true => 13,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 14 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                Token(13, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<ast::Statement>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                // __Ast = Ast => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 12 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Expr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Value, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, typed_ast::Type, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Ast = Statement+ => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ConstantValue = _integer => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ConstantValue = _decimal => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = Expr, "+", Factor => ActionFn(8);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = Expr, "-", Factor => ActionFn(9);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = Factor => ActionFn(10);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Factor = Factor, "*", Term => ActionFn(11);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Factor = Factor, "/", Term => ActionFn(12);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Factor = Term => ActionFn(13);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Ident = r#"\\w[\\w\\d]*"# => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Statement = VariableAssignment => ActionFn(2);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 5)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Statement = VariableDeclaration => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 5)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Statement = Expr, ";" => ActionFn(4);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action4::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Statement+ = Statement => ActionFn(20);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(21);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 6)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = ConstantValue => ActionFn(14);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Ident => ActionFn(15);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", Expr, ")" => ActionFn(16);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 7)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Type = "i32" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // VariableAssignment = Ident, "=", Expr, ";" => ActionFn(5);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (4, __symbol, 9)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // VariableDeclaration = "let", Ident, ":", Type, ";" => ActionFn(6);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (5, __symbol, 10)
    }
}
pub use self::__parse__Ast::AstParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use ast;
    use typed_ast::{self, Type};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^((?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ\u{300}-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙ-ՙա-և\u{591}-\u{5bd}\u{5bf}-\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}-\u{5c7}א-תװ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿ-ۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ-ߺࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࢠ-ࢴࢶ-ࢽ\u{8d4}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}-\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ-ৼ\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}-\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}-\u{a51}ਖ਼-ੜਫ਼-ਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐ-ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b56}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ-ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ-ௐ\u{bd7}-\u{bd7}௦-௯\u{c00}-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೞ-ೞೠ-\u{ce3}೦-೯ೱ-ೲ\u{d00}-ഃഅ-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ\u{dca}-\u{dca}\u{dcf}-\u{dd4}\u{dd6}-\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-\u{eb9}\u{ebb}-ຽເ-ໄໆ-ໆ\u{ec8}-\u{ecd}໐-໙ໜ-ໟༀ-ༀ\u{f18}-\u{f19}༠-༩\u{f35}-\u{f35}\u{f37}-\u{f37}\u{f39}-\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}-\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-\u{1714}ᜠ-\u{1734}ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗ-ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ-ᪧ\u{1ab0}-\u{1abe}\u{1b00}-ᭋ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈ\u{1cd0}-\u{1cd2}\u{1cd4}-\u{1cf9}ᴀ-\u{1df9}\u{1dfb}-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ\u{20d0}-\u{20f0}ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ-ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻ-ꣻꣽ-ꣽ꤀-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}-\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}-\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨳\u{10a38}-\u{10a3a}\u{10a3f}-\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-\u{11046}𑁦-𑁯\u{1107f}-\u{110ba}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅐-\u{11173}𑅶-𑅶\u{11180}-𑇄\u{111ca}-\u{111cc}𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}-\u{1123e}𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133c}-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐\u{11357}-\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄-𑙄𑙐-𑙙𑚀-\u{116b7}𑛀-𑛉𑜀-𑜙\u{1171d}-\u{1172b}𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-\u{11a3e}\u{11a47}-\u{11a47}𑩐-𑪃𑪆-\u{11a99}𑫀-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}-\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾\u{16f8f}-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}-\u{1da75}\u{1da84}-\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-\u{1e94a}𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀\u{e0100}-\u{e01ef}])(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ\u{300}-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙ-ՙա-և\u{591}-\u{5bd}\u{5bf}-\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}-\u{5c7}א-תװ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿ-ۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ-ߺࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࢠ-ࢴࢶ-ࢽ\u{8d4}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}-\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ-ৼ\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}-\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}-\u{a51}ਖ਼-ੜਫ਼-ਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐ-ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b56}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ-ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ-ௐ\u{bd7}-\u{bd7}௦-௯\u{c00}-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೞ-ೞೠ-\u{ce3}೦-೯ೱ-ೲ\u{d00}-ഃഅ-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ\u{dca}-\u{dca}\u{dcf}-\u{dd4}\u{dd6}-\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-\u{eb9}\u{ebb}-ຽເ-ໄໆ-ໆ\u{ec8}-\u{ecd}໐-໙ໜ-ໟༀ-ༀ\u{f18}-\u{f19}༠-༩\u{f35}-\u{f35}\u{f37}-\u{f37}\u{f39}-\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}-\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-\u{1714}ᜠ-\u{1734}ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗ-ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ-ᪧ\u{1ab0}-\u{1abe}\u{1b00}-ᭋ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈ\u{1cd0}-\u{1cd2}\u{1cd4}-\u{1cf9}ᴀ-\u{1df9}\u{1dfb}-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ\u{20d0}-\u{20f0}ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ-ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻ-ꣻꣽ-ꣽ꤀-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}-\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}-\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨳\u{10a38}-\u{10a3a}\u{10a3f}-\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-\u{11046}𑁦-𑁯\u{1107f}-\u{110ba}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅐-\u{11173}𑅶-𑅶\u{11180}-𑇄\u{111ca}-\u{111cc}𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}-\u{1123e}𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133c}-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐\u{11357}-\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄-𑙄𑙐-𑙙𑚀-\u{116b7}𑛀-𑛉𑜀-𑜙\u{1171d}-\u{1172b}𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-\u{11a3e}\u{11a47}-\u{11a47}𑩐-𑪃𑪆-\u{11a99}𑫀-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}-\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾\u{16f8f}-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}-\u{1da75}\u{1da84}-\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-\u{1e94a}𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀\u{e0100}-\u{e01ef}])*)",
                "^((?u:\\())",
                "^((?u:\\)))",
                "^((?u:\\*))",
                "^((?u:\\+))",
                "^((?u:\\-))",
                "^((?u:/))",
                "^((?u::))",
                "^((?u:;))",
                "^((?u:=))",
                "^((?u:i32))",
                "^((?u:let))",
                "^((?u:[0-9])+(?u:.)(?u:[0-9])+)",
                "^((?u:[0-9])+)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ\u{300}-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙ-ՙա-և\u{591}-\u{5bd}\u{5bf}-\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}-\u{5c7}א-תװ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿ-ۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ-ߺࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࢠ-ࢴࢶ-ࢽ\u{8d4}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}-\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ-ৼ\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}-\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}-\u{a51}ਖ਼-ੜਫ਼-ਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐ-ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b56}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ-ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ-ௐ\u{bd7}-\u{bd7}௦-௯\u{c00}-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೞ-ೞೠ-\u{ce3}೦-೯ೱ-ೲ\u{d00}-ഃഅ-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ\u{dca}-\u{dca}\u{dcf}-\u{dd4}\u{dd6}-\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-\u{eb9}\u{ebb}-ຽເ-ໄໆ-ໆ\u{ec8}-\u{ecd}໐-໙ໜ-ໟༀ-ༀ\u{f18}-\u{f19}༠-༩\u{f35}-\u{f35}\u{f37}-\u{f37}\u{f39}-\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}-\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-\u{1714}ᜠ-\u{1734}ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗ-ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ-ᪧ\u{1ab0}-\u{1abe}\u{1b00}-ᭋ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈ\u{1cd0}-\u{1cd2}\u{1cd4}-\u{1cf9}ᴀ-\u{1df9}\u{1dfb}-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ\u{20d0}-\u{20f0}ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ-ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻ-ꣻꣽ-ꣽ꤀-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}-\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}-\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨳\u{10a38}-\u{10a3a}\u{10a3f}-\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-\u{11046}𑁦-𑁯\u{1107f}-\u{110ba}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅐-\u{11173}𑅶-𑅶\u{11180}-𑇄\u{111ca}-\u{111cc}𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}-\u{1123e}𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133c}-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐\u{11357}-\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄-𑙄𑙐-𑙙𑚀-\u{116b7}𑛀-𑛉𑜀-𑜙\u{1171d}-\u{1172b}𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-\u{11a3e}\u{11a47}-\u{11a47}𑩐-𑪃𑪆-\u{11a99}𑫀-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}-\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾\u{16f8f}-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}-\u{1da75}\u{1da84}-\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-\u{1e94a}𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀\u{e0100}-\u{e01ef}])(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ\u{300}-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙ-ՙա-և\u{591}-\u{5bd}\u{5bf}-\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}-\u{5c7}א-תװ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿ-ۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ-ߺࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࢠ-ࢴࢶ-ࢽ\u{8d4}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}-\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ-ৼ\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}-\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}-\u{a51}ਖ਼-ੜਫ਼-ਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐ-ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b56}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ-ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ-ௐ\u{bd7}-\u{bd7}௦-௯\u{c00}-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೞ-ೞೠ-\u{ce3}೦-೯ೱ-ೲ\u{d00}-ഃഅ-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ\u{dca}-\u{dca}\u{dcf}-\u{dd4}\u{dd6}-\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-\u{eb9}\u{ebb}-ຽເ-ໄໆ-ໆ\u{ec8}-\u{ecd}໐-໙ໜ-ໟༀ-ༀ\u{f18}-\u{f19}༠-༩\u{f35}-\u{f35}\u{f37}-\u{f37}\u{f39}-\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}-\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-\u{1714}ᜠ-\u{1734}ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗ-ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ-ᪧ\u{1ab0}-\u{1abe}\u{1b00}-ᭋ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈ\u{1cd0}-\u{1cd2}\u{1cd4}-\u{1cf9}ᴀ-\u{1df9}\u{1dfb}-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ\u{20d0}-\u{20f0}ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ-ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻ-ꣻꣽ-ꣽ꤀-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}-\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}-\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨳\u{10a38}-\u{10a3a}\u{10a3f}-\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-\u{11046}𑁦-𑁯\u{1107f}-\u{110ba}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅐-\u{11173}𑅶-𑅶\u{11180}-𑇄\u{111ca}-\u{111cc}𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}-\u{1123e}𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133c}-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐\u{11357}-\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄-𑙄𑙐-𑙙𑚀-\u{116b7}𑛀-𑛉𑜀-𑜙\u{1171d}-\u{1172b}𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-\u{11a3e}\u{11a47}-\u{11a47}𑩐-𑪃𑪆-\u{11a99}𑫀-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}-\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾\u{16f8f}-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}-\u{1da75}\u{1da84}-\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-\u{1e94a}𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀\u{e0100}-\u{e01ef}])*)").unwrap(),
                __regex::Regex::new("^((?u:\\())").unwrap(),
                __regex::Regex::new("^((?u:\\)))").unwrap(),
                __regex::Regex::new("^((?u:\\*))").unwrap(),
                __regex::Regex::new("^((?u:\\+))").unwrap(),
                __regex::Regex::new("^((?u:\\-))").unwrap(),
                __regex::Regex::new("^((?u:/))").unwrap(),
                __regex::Regex::new("^((?u::))").unwrap(),
                __regex::Regex::new("^((?u:;))").unwrap(),
                __regex::Regex::new("^((?u:=))").unwrap(),
                __regex::Regex::new("^((?u:i32))").unwrap(),
                __regex::Regex::new("^((?u:let))").unwrap(),
                __regex::Regex::new("^((?u:[0-9])+(?u:.)(?u:[0-9])+)").unwrap(),
                __regex::Regex::new("^((?u:[0-9])+)").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 14 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<ast::Statement>, usize),
) -> Vec<ast::Statement>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<ast::Statement>, usize),
) -> Vec<ast::Statement>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, e, _): (usize, ast::Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::Statement
{
    ast::Statement::ExpressionStatement(e)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, ast::Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::Statement
{
    ast::Statement::VarAssign(l, e)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, typed_ast::Type, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::Statement
{
    ast::Statement::VarDecl(l, t)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> typed_ast::Type
{
    Type::INTEGER_32
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, ast::Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, ast::Expr, usize),
) -> ast::Expr
{
    ast::Expr::BinaryOp(Box::new(l), ast::BinaryOp::Add, Box::new(r))
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, ast::Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, ast::Expr, usize),
) -> ast::Expr
{
    ast::Expr::BinaryOp(Box::new(l), ast::BinaryOp::Sub, Box::new(r))
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Expr, usize),
) -> ast::Expr
{
    (__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, ast::Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, ast::Expr, usize),
) -> ast::Expr
{
    ast::Expr::BinaryOp(Box::new(l), ast::BinaryOp::Mul, Box::new(r))
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, ast::Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, ast::Expr, usize),
) -> ast::Expr
{
    ast::Expr::BinaryOp(Box::new(l), ast::BinaryOp::Div, Box::new(r))
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Expr, usize),
) -> ast::Expr
{
    (__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Value, usize),
) -> ast::Expr
{
    ast::Expr::Constant(__0)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ast::Expr
{
    ast::Expr::VariableLookup(__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ast::Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::Expr
{
    (__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_owned()
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Value
{
    ast::Value::Integer(__0.parse().unwrap())
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Value
{
    ast::Value::Decimal(__0.parse().unwrap())
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ::std::vec::Vec<ast::Statement>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ast::Statement>, usize),
    (_, e, _): (usize, ast::Statement, usize),
) -> ::std::vec::Vec<ast::Statement>
{
    { let mut v = v; v.push(e); v }
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
