
pub mod instruction_set;
pub mod codegen;
use self::codegen::{Instruction};

use std::mem::transmute;

const STACK_MAX: usize = 256;

pub struct Machine {
	module: codegen::Module,
	stack: [i64; STACK_MAX],
	stack_top: usize,
}

impl Machine {
	pub fn new(module: codegen::Module) -> Self {
		Machine {
			module,
			stack: [0; STACK_MAX],
			stack_top: 0,
		}
	}

	pub fn execute(&mut self) {
		let mut isp = 0;

		loop {
			match self.module.instructions[isp] {
				Instruction::CONST_I32(i) => {
					let c = self.module.constants[i as usize] as i32;
					self.push::<i32>(c);
				}
				Instruction::ADD_I32 => {
					let left: i32 = self.pop();
					let right: i32 = self.pop();
					self.push(left + right);
				}
			}
			isp += 1;
			if isp == self.module.instructions.len() { break; }
		}
	}

	pub fn print_stack(&mut self) {
		for i in self.stack.iter() {
			print!("{}, ", i);
		}
	}

	pub fn push<T>(&mut self, val: T) {
		assert!(::std::mem::size_of::<T>() <= ::std::mem::size_of::<i64>());
		
		unsafe {
			::std::ptr::copy::<T>(transmute(&val), transmute(&self.stack[self.stack_top]), 1);
		}
		self.stack_top += 1;
	}

	pub fn pop<T>(&mut self) -> T {
		assert!(::std::mem::size_of::<T>() <= ::std::mem::size_of::<i64>());		

		self.stack_top -= 1;
		unsafe {
			let v = ::std::mem::zeroed::<T>();
			::std::ptr::copy::<T>(transmute(&self.stack[self.stack_top]), transmute(&v), 1);
			return v;
		}
	}
}