// program.rs
use std::{fs::File, u8};
use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;

use crate::opcodes::{Opcode};

pub struct Program {
	pub program_counter: u16,
	pub reg_a: u8,
	pub reg_x: u8,
	pub reg_y: u8,
	pub stack_pointer: u8,

	pub flag_negative: bool,
	pub flag_overflow: bool,
	pub flag_decimal: bool,
	pub flag_interrupt: bool,
	pub flag_zero: bool,
	pub flag_carry: bool,
	pub flag_break: bool,

	pub fetched_byte: u8,
	pub abs_address: u16,
	pub rel_address: i8,

	pub memory: Vec<u8>,
}


impl Program {
	pub fn new() -> Self {
		Program{
			program_counter: 0,
			reg_a: 0,
			reg_x: 0,
			reg_y: 0,
			stack_pointer: 0xff,

			fetched_byte: 0,
			abs_address: 0,
			rel_address: 0,

			flag_negative: false,
			flag_overflow: false,
			flag_decimal: false,
			flag_interrupt: false,
			flag_zero: false,
			flag_carry: false,
			flag_break: false,

			memory: vec![0; 65535],
		}
	}

	pub fn get_memory(&self, address: u16) -> u8 {
		self.memory[address as usize]
	}

	pub fn set_memory(&mut self, address: u16, value: u8) {
		self.memory[address as usize] = value;
	}

	pub fn advance_counter(&mut self) {
		self.program_counter += 1;
	}

	pub fn brk(&self) {
		println!("BREAK");
	}

	pub fn stack_push(&mut self, value: u8) {
		self.memory[self.stack_pointer as usize] = value;
		self.stack_pointer = self.stack_pointer.wrapping_sub(1);
	}

	pub fn stack_pull(&mut self) -> u8 {
		let ptr = self.stack_pointer;
		self.stack_pointer = self.stack_pointer.wrapping_add(1);
		self.memory[ptr as usize]
	}

	/*pub fn fetch_byte(&mut self) {
		let byte = self.file.read_u8().unwrap();
		self.fetched_byte = byte;
		self.program_counter += 1;
	}*/
}
