#![feature(ptr_offset_from)]
#![feature(asm)]
#![feature(nll)]

extern crate rand;
extern crate stopwatch;
use stopwatch::{Stopwatch};
extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

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
	let sw_fs = Stopwatch::start_new();

	let mut source = String::new();
	fs::File::open("examples/scope.nrs")
		.expect("Failed to open source file")
		.read_to_string(&mut source).unwrap();

	println!("Loading script took: {}ms", sw_fs.elapsed_ms());

	let sw_ps = Stopwatch::start_new();
	let untyped_ast = grammar::AstParser::new()
		.parse(&source).expect("Failed to parse grammar.");
	println!("Parsing grammar took: {}ms", sw_ps.elapsed_ms());

	// println!("{:#?}", untyped_ast);
	
	let sw_tp = Stopwatch::start_new();
	let typed_ast = passes::typecheck::pass(untyped_ast);
	println!("Typechecking took: {}ms", sw_tp.elapsed_ms());

	let sw_cg = Stopwatch::start_new();
	let unlinked = passes::codegen::pass(&typed_ast);
	println!("Codegen took: {}ms", sw_cg.elapsed_ms());

	let mut dump_file = fs::File::create("dump.nsm").unwrap();
	unlinked.dump_assembly(&mut dump_file);

	let sw_ln = Stopwatch::start_new();
	let linked_module = passes::linker::pass(unlinked);
	println!("Linking took: {}ms", sw_ln.elapsed_ms());

	let sw_vm = Stopwatch::start_new();
	let mut vm = vm::Machine::new(linked_module);
	vm.execute();
	println!("Execution took: {}ms", sw_vm.elapsed_ms());

	// vm.print_stack();
	// vm.print_vars();

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

}
