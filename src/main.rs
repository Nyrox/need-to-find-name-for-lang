#![feature(ptr_offset_from)]

pub mod grammar;
pub mod ast;
pub mod typed_ast;
pub mod typecheck;

pub mod vm_safe;

fn main() {
	let _result = grammar::AstParser::new()
		.parse(r"5 + 2").unwrap();
	
	println!("{:#?}", _result);

	let typed_ast = typecheck::check(_result);
    let module = vm_safe::codegen::gen(typed_ast);

	println!("{:?}", module);

	let mut vm = vm_safe::Machine::new(module);
	vm.execute();
	vm.print_stack();
}
