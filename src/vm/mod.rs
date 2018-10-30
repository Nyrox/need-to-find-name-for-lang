use repr::{self, linked, instruction_set};
use repr::instruction_set::Instruction;

use std::mem::transmute;

const STACK_MAX: usize = 256;
const VAR_TABLE_MAX: usize = 256;

pub struct Machine {
	module: linked::Module,
	stack: [i64; STACK_MAX],
	stack_top: usize,
	var_table: [i64; VAR_TABLE_MAX],
	callstack: Vec<usize>,
}

impl Machine {
	pub fn new(module: linked::Module) -> Self {
		Machine {
			module,
			stack: [0; STACK_MAX],
			stack_top: 0,
			var_table: [0; STACK_MAX],
			callstack: Vec::new(),
		}
	}

	pub fn execute(&mut self) {
		let mut isp = self.module.entry as usize;
		let len = self.module.instructions.len();

		macro_rules! impl_binary_op {
			($pType: ty, $op: tt) => {{
				let right: $pType = self.pop();
				let left: $pType = self.pop();
				self.push(left $op right);
			}}
		}

		macro_rules! impl_cast {
			($fr: ty, $to: ty) => {{
				let val: $fr = self.pop();
				self.push(val as $to);
			}};
		}

		loop {
			match self.module.instructions[isp] {
				Instruction::CONST_I32(i) => {
					let c = self.module.constants[i as usize] as i32;
					self.push::<i32>(c);
				},
				Instruction::CONST_F32(i) => {
					let c = self.get_constant::<f32>(i);
					self.push(c);
				}
				Instruction::VAR_ASSIGN(i) => {
					self.pop_to_var(i);
				}
				Instruction::VAR_LOOKUP(i) => {
					self.push_var(i);
				}
				Instruction::ADD_I32 => { impl_binary_op!(i32, +) },
				Instruction::SUB_I32 => { impl_binary_op!(i32, -) },
				Instruction::DIV_I32 => { impl_binary_op!(i32, /) },
				Instruction::MUL_I32 => { impl_binary_op!(i32, *) },
				Instruction::LESS_I32 => { impl_binary_op!(i32, <) },
				Instruction::ADD_F32 => { impl_binary_op!(f32, +) },
				Instruction::SUB_F32 => { impl_binary_op!(f32, -) },
				Instruction::DIV_F32 => { impl_binary_op!(f32, /) },
				Instruction::MUL_F32 => { impl_binary_op!(f32, *) },

				Instruction::CAST_I32_F32 => { impl_cast!(i32, f32) },
				Instruction::CAST_F32_I32 => { impl_cast!(f32, i32) },
				Instruction::RETURN => {
					let ret = self.pop::<i64>();
					if self.callstack.len() == 0 { return; }
					let nIsp = self.callstack.pop();
					isp = nIsp.unwrap();
					self.push(ret);
					continue;
				},
				Instruction::CALL(nIsp) => {
					self.callstack.push(isp + 1);
					isp = nIsp as usize;
					continue;
				},
				Instruction::COND_JMP(nIsp) => {
					if self.pop::<i64>() != 0 {
						isp = nIsp as usize;
					}
				}
				Instruction::POP_STACK => {
					self.pop::<i64>();
				},
				Instruction::PUSH_VOID => {
					self.push::<i64>(0);
				}
				Instruction::PRINT(_type) => {
					match _type {
						repr::Type::INTEGER_32 => println!("Runtime Print: {}", self.pop::<i32>()),
						repr::Type::INTEGER_64 => println!("Runtime Print: {}", self.pop::<i64>()),
						repr::Type::FLOAT_32 => println!("Runtime Print: {}", self.pop::<f32>()),
						repr::Type::FLOAT_64 => println!("Runtime Print: {}", self.pop::<f64>()),
						repr::Type::UNIT => { self.pop::<i64>(); println!("Runtime Print: ()"); },
					};
				}
				_ => panic!("IRE [Missing Impl]: {:?}", self.module.instructions[isp])
			}
			isp += 1;
			if isp >= self.module.instructions.len() { break; }
		}
	}

	pub fn debug_log(&self, what: &str) {
		if cfg!(debug_assertions) == true {

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

	pub fn get_constant<T>(&self, i: i16) -> T {

		unsafe {
			let v = ::std::mem::zeroed::<T>();
			::std::ptr::copy::<T>(transmute(&self.module.constants[i as usize]), transmute(&v), 1);
			return v;
		}
	}
}
