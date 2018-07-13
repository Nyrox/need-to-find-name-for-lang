pub mod grammar;
pub mod ast;
pub mod codegen;
pub mod typed_ast;
pub mod typecheck;

fn main() {
	let _result = grammar::AstParser::new()
		.parse(r"5 + 2").unwrap();
	
	println!("{:#?}", _result);

	let typed_ast = typecheck::check(_result);
    let module = codegen::gen(typed_ast);

	println!("{:?}", module);
}