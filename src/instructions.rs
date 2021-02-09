// instructions.rs
#![allow(non_snake_case)]

use crate::program::Program;
use crate::addressing::AddressMode;

fn branch(program: &mut Program) {
	program.program_counter = program.program_counter.wrapping_add(program.rel_address as u16);
}

// =============================================================

pub fn AND(program: &mut Program, _amode: AddressMode) {
	let result = program.reg_a & program.fetched_byte;

	program.reg_a = result;
	program.flag_zero = result == 0;
	program.flag_negative = (result as i8) < 0;
}

pub fn ASL(program: &mut Program, amode: AddressMode) {
	let (result, prev) = match amode {
		AddressMode::Accumulator => {
			let p = program.reg_a;
			program.reg_a <<= 1;
			(program.reg_a, p)
		},
		_ => {
			let res = program.fetched_byte << 1;
			program.set_memory(program.abs_address, res);
			(res, program.fetched_byte)
		}
	};

	program.flag_zero = result == 0;
	program.flag_negative = (result as i8) < 0;
	program.flag_carry = result < prev;
}

pub fn BCC(program: &mut Program) {
	if !program.flag_carry {
		branch(program);
	}
}

pub fn BCS(program: &mut Program) {
	if program.flag_carry {
		branch(program);
	}
}

pub fn BEQ(program: &mut Program) {
	if program.flag_zero {
		branch(program);
	}
}

pub fn BIT(program: &mut Program) {
	let operand = program.fetched_byte;
	let bit6 = (operand >> 6) & 1;
	let bit7 = (operand >> 7) & 1;

	program.stack_pointer ^= (!bit6 ^ program.stack_pointer) & (1u8 << 6);
	program.stack_pointer ^= (!bit7 ^ program.stack_pointer) & (1u8 << 7);
	program.flag_zero = program.reg_a & operand == 0;
}

pub fn BMI(program: &mut Program) {
	if program.flag_negative {
		branch(program);
	}
}

pub fn BNE(program: &mut Program) {
	if !program.flag_zero {
		branch(program);
	}
}

pub fn BPL(program: &mut Program) {
	if !program.flag_negative {
		branch(program);
	}
}
