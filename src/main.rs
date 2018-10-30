#![feature(ptr_offset_from)]
#![feature(asm)]
#![feature(nll)]

extern crate rand;

pub mod grammar;
// pub mod vm_safe;
pub mod repr;
pub mod passes;
pub mod vm;

use std::fs;
use std::io::prelude::*;

extern "cdecl" fn foo(i: i32) {
	println!("This is a test: {}", i);
}

fn main() {
	let mut source = String::new();
	fs::File::open("examples/scope.nrs")
		.expect("Failed to open source file")
		.read_to_string(&mut source).unwrap();

	let untyped_ast = grammar::AstParser::new()
		.parse(&source).expect("Failed to parse grammar.");

	// println!("{:#?}", untyped_ast);
	let typed_ast = passes::typecheck::pass(untyped_ast);
	println!("{:#?}", typed_ast);
	let unlinked = passes::codegen::pass(&typed_ast);
	println!("{:?}", unlinked);

	let mut dump_file = fs::File::create("dump.nsm").unwrap();
	unlinked.dump_assembly(&mut dump_file);


	let linked_module = passes::linker::pass(unlinked);
	println!("{:?}", linked_module);

	let mut vm = vm::Machine::new(linked_module);
	vm.execute();
	vm.print_stack();

	// unsafe {
	// 	let fnp: extern "cdecl" fn(i32) = foo;

	// 	asm!(r"
	// 		mov rax, rdi
	// 		mov rdi, 5
	// 		call rax
	// 	"
	// 		:
	// 		: "{rdi}"(fnp)
	// 		:: "intel"
	// 	);
	// }

	vm.print_vars();
}
