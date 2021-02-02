// program.rs
use std::{fs::File, u8};
use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;

use crate::opcodes::{Opcode, OPERAND_COUNTS};


pub struct Program {
	file: File,
	program_counter: u16,
	reg_a: u8,
	reg_x: u8,
	reg_y: u8,
	stack_pointer: u8,

	flag_negative: bool,
	flag_overflow: bool,
	flag_decimal: bool,
	flag_interrupt: bool,
	flag_zero: bool,
	flag_carry: bool,

	memory: Vec<u8>,
}


impl Program {
	pub fn new(file: File) -> Self {
		Program{
			file,

			program_counter: 0,
			reg_a: 0,
			reg_x: 0,
			reg_y: 0,
			stack_pointer: 0xff,

			flag_negative: false,
			flag_overflow: false,
			flag_decimal: false,
			flag_interrupt: false,
			flag_zero: false,
			flag_carry: false,

			memory: vec![0; 65535],
		}
	}

	pub fn run(&mut self) {
		let origin = self.file.read_u16::<LittleEndian>().unwrap();
		self.program_counter = origin;
	}

	pub fn set_memory(&mut self, address: u16, value: u8) {
		self.memory[address as usize] = value;
	}

	pub fn get_a(&self) -> &u8 {
		&self.reg_a
	}

	pub fn set_a(&mut self, value: u8) {
		self.reg_a = value;
	}

	pub fn get_x(&self) -> &u8 {
		&self.reg_x
	}

	pub fn get_y(&self) -> &u8 {
		&self.reg_y
	}
}


impl Program {
	fn run_instruction(&mut self) {
		let opcode: Opcode = FromPrimitive::from_u8(self.fetch_byte()).unwrap();
		let mut operands = Vec::<u8>::new();
		for _i in 0..OPERAND_COUNTS[&opcode] {
			operands.push(self.fetch_byte());
		}
	}

	fn fetch_byte(&mut self) -> u8 {
		let byte = self.file.read_u8().unwrap();
		self.program_counter += 1;
		byte
	}
}
