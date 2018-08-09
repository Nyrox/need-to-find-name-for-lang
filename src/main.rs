#![feature(ptr_offset_from)]
#![feature(asm)]
#![feature(nll)]


pub mod grammar;
// pub mod vm_safe;
pub mod repr;
pub mod passes;
pub mod vm;

extern "cdecl" fn foo(i: i32) {
	println!("This is a test: {}", i);
}

fn main() {
	let untyped_ast = grammar::AstParser::new()
		.parse(r"
		fn foo() -> i32 {
			return 10 / 2;
		}

		fn main() -> i32 {
			let y: i32;
			y = foo() + 3;
			return y * 2;
		}
	").expect("Failed to parse grammar.");
	
	println!("{:#?}", untyped_ast);
	let typed_ast = passes::typecheck::pass(untyped_ast);
	println!("{:#?}", typed_ast);
	let unlinked = passes::codegen::pass(&typed_ast);
	println!("{:?}", unlinked);
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
