#![feature(ptr_offset_from)]

pub mod grammar;
pub mod ast;
pub mod typed_ast;
pub mod typecheck;

pub mod vm_safe;

fn main() {
	let _result = grammar::AstParser::new()
		.parse(r"let x: i32; x = (5 + 2 * 2 * 5) / 3; x = x * 2 - 3;");
	

	let typed_ast = typecheck::check(&_result.unwrap());
	println!("{:#?}", typed_ast);

	let module = vm_safe::codegen::gen(&typed_ast);

	println!("{:?}", module);

	let mut vm = vm_safe::Machine::new(module);
	vm.execute();
	vm.print_stack();
	vm.print_vars();
}
