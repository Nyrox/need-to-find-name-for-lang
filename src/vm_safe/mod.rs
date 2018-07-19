
pub mod instruction_set;
pub mod codegen;
use self::codegen::{Instruction};

use std::mem::transmute;

const STACK_MAX: usize = 256;
const VAR_TABLE_MAX: usize = 256;

pub struct Machine {
	module: codegen::Module,
	stack: [i64; STACK_MAX],
	stack_top: usize,
	var_table: [i64; VAR_TABLE_MAX],
}

impl Machine {
	pub fn new(module: codegen::Module) -> Self {
		Machine {
			module,
			stack: [0; STACK_MAX],
			stack_top: 0,
			var_table: [0; STACK_MAX],
		}
	}

	pub fn execute(&mut self) {
		let mut isp = 0;

		macro_rules! impl_binary_op {
			($pType: ty, $op: tt) => {
				let right: $pType = self.pop();
				let left: $pType = self.pop();
				self.push(left $op right);
			}
		}

		loop {
			match self.module.instructions[isp] {
				Instruction::CONST_I32(i) => {
					let c = self.module.constants[i as usize] as i32;
					self.push::<i32>(c);
				}
				Instruction::VAR_ASSIGN(i) => {
					self.pop_to_var(i);
				}
				Instruction::VAR_LOOKUP(i) => {
					self.push_var(i);
				}
				Instruction::ADD_I32 => { impl_binary_op!(i32, +); },
				Instruction::SUB_I32 => { impl_binary_op!(i32, -); },
				Instruction::DIV_I32 => { impl_binary_op!(i32, /); }
				Instruction::MUL_I32 => { impl_binary_op!(i32, *); },
				_ => panic!("Missing impl")
			}
			isp += 1;
			if isp == self.module.instructions.len() { break; }
		}
	}

	pub fn print_stack(&mut self) {
		println!("[Stack]: ");
		for i in self.stack.iter() {
			print!("{}, ", i);
		}
		println!("")
	}

	pub fn print_vars(&mut self) {
		println!("[Vars]: ");
		for i in self.var_table.iter() {
			print!("{:?}, ", i);
		}
		println!("");
	}

	pub fn pop_to_var(&mut self, var_i: i16) {
		let val = self.pop::<i64>();

		unsafe {
			::std::ptr::copy::<i64>(&val, transmute(&self.var_table[var_i as usize]), 1);
		}
	}

	pub fn push_var(&mut self, var_i: i16) {
		unsafe {
			::std::ptr::copy::<i64>(transmute(&self.var_table[var_i as usize]), transmute(&self.stack[self.stack_top]), 1);
			self.stack_top += 1;
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