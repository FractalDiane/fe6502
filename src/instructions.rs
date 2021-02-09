// instructions.rs
#![allow(non_snake_case)]

use crate::program::Program;
use crate::addressing::AddressMode;
use crate::addressing::make_u16;

fn branch(program: &mut Program) {
	program.program_counter = program.program_counter.wrapping_add(program.rel_address as u16);
}

// =============================================================

pub fn ADC(program: &mut Program, _amode: &AddressMode) {
	let prev = program.reg_a;
	let result: u8;
	if !program.flag_decimal {
		result = program.reg_a.wrapping_add(program.fetched_byte + program.flag_carry as u8);
	}
	else {
		result = program.reg_a.wrapping_add(program.fetched_byte + program.flag_carry as u8);
	}

	program.reg_a = result;

	program.flag_zero = program.reg_a == 0;
	program.flag_negative = (program.reg_a as i8) < 0;
	program.flag_carry = result < prev;
	program.flag_overflow = (result as i8).signum() != (prev as i8).signum();
}

pub fn AND(program: &mut Program, _amode: &AddressMode) {
	let result = program.reg_a & program.fetched_byte;
	program.reg_a = result;

	program.flag_zero = result == 0;
	program.flag_negative = (result as i8) < 0;
}

pub fn ASL(program: &mut Program, amode: &AddressMode) {
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

pub fn BCC(program: &mut Program, _amode: &AddressMode) {
	if !program.flag_carry {
		branch(program);
	}
}

pub fn BCS(program: &mut Program, _amode: &AddressMode) {
	if program.flag_carry {
		branch(program);
	}
}

pub fn BEQ(program: &mut Program, _amode: &AddressMode) {
	if program.flag_zero {
		branch(program);
	}
}

pub fn BIT(program: &mut Program, _amode: &AddressMode) {
	let operand = program.fetched_byte;
	let bit6 = (operand >> 6) & 1;
	let bit7 = (operand >> 7) & 1;

	program.stack_pointer ^= (!bit6 ^ program.stack_pointer) & (1u8 << 6);
	program.stack_pointer ^= (!bit7 ^ program.stack_pointer) & (1u8 << 7);
	program.flag_zero = program.reg_a & operand == 0;
}

pub fn BMI(program: &mut Program, _amode: &AddressMode) {
	if program.flag_negative {
		branch(program);
	}
}

pub fn BNE(program: &mut Program, _amode: &AddressMode) {
	if !program.flag_zero {
		branch(program);
	}
}

pub fn BPL(program: &mut Program, _amode: &AddressMode) {
	if !program.flag_negative {
		branch(program);
	}
}

pub fn BRK(program: &mut Program, _amode: &AddressMode) {
	program.brk();
	program.flag_break = true;
}

pub fn BVC(program: &mut Program, _amode: &AddressMode) {
	if !program.flag_overflow {
		branch(program);
	}
}

pub fn BVS(program: &mut Program, _amode: &AddressMode) {
	if program.flag_overflow {
		branch(program);
	}
}

pub fn CLC(program: &mut Program, _amode: &AddressMode) {
	program.flag_carry = false;
}

pub fn CLD(program: &mut Program, _amode: &AddressMode) {
	program.flag_decimal = false;
}

pub fn CLI(program: &mut Program, _amode: &AddressMode) {
	program.flag_interrupt = false;
}

pub fn CLV(program: &mut Program, _amode: &AddressMode) {
	program.flag_overflow = false;
}

pub fn CMP(program: &mut Program, _amode: &AddressMode) {
	program.flag_zero = program.reg_a == program.fetched_byte;
	program.flag_carry = program.reg_a > program.fetched_byte || program.reg_a == program.fetched_byte;
	program.flag_negative = ((program.reg_a - program.fetched_byte) >> 7) == 1;
}

pub fn CPX(program: &mut Program, _amode: &AddressMode) {
	program.flag_zero = program.reg_x == program.fetched_byte;
	program.flag_carry = program.reg_x > program.fetched_byte || program.reg_x == program.fetched_byte;
	program.flag_negative = ((program.reg_x - program.fetched_byte) >> 7) == 1;
}

pub fn CPY(program: &mut Program, _amode: &AddressMode) {
	program.flag_zero = program.reg_y == program.fetched_byte;
	program.flag_carry = program.reg_y > program.fetched_byte || program.reg_y == program.fetched_byte;
	program.flag_negative = ((program.reg_y - program.fetched_byte) >> 7) == 1;
}

pub fn DEC(program: &mut Program, _amode: &AddressMode) {
	let result = program.get_memory(program.abs_address) - 1;
	program.set_memory(program.abs_address, result);

	program.flag_negative = (result as i8) < 0;
	program.flag_zero = result == 0;
}

pub fn DEX(program: &mut Program, _amode: &AddressMode) {
	program.reg_x -= 1;
	
	program.flag_negative = (program.reg_x as i8) < 0;
	program.flag_zero = program.reg_x == 0;
}

pub fn DEY(program: &mut Program, _amode: &AddressMode) {
	program.reg_y -= 1;
	
	program.flag_negative = (program.reg_y as i8) < 0;
	program.flag_zero = program.reg_y == 0;
}

pub fn EOR(program: &mut Program, _amode: &AddressMode) {
	let result = program.reg_a ^ program.fetched_byte;
	program.reg_a = result;
	
	program.flag_zero = result == 0;
	program.flag_negative = (result as i8) < 0;
}

pub fn INC(program: &mut Program, _amode: &AddressMode) {
	let result = program.get_memory(program.abs_address) + 1;
	program.set_memory(program.abs_address, result);

	program.flag_negative = (result as i8) < 0;
	program.flag_zero = result == 0;
}

pub fn INX(program: &mut Program, _amode: &AddressMode) {
	program.reg_x += 1;

	program.flag_negative = (program.reg_x as i8) < 0;
	program.flag_zero = program.reg_x == 0;
}

pub fn INY(program: &mut Program, _amode: &AddressMode) {
	program.reg_y += 1;

	program.flag_negative = (program.reg_y as i8) < 0;
	program.flag_zero = program.reg_y == 0;
}

pub fn JMP(program: &mut Program, _amode: &AddressMode) {
	program.program_counter = program.abs_address;
	// Replicate indirect addressing bug here later
}

pub fn JSR(program: &mut Program, _amode: &AddressMode) {
	program.stack_push((program.program_counter + 2) as u8);
	program.stack_push((program.program_counter + 3) as u8);
	program.program_counter = program.abs_address;
}

pub fn LDA(program: &mut Program, _amode: &AddressMode) {
	program.reg_a = program.fetched_byte;

	program.flag_zero = program.fetched_byte == 0;
	program.flag_negative = (program.fetched_byte as i8) < 0;
}

pub fn LDX(program: &mut Program, _amode: &AddressMode) {
	program.reg_x = program.fetched_byte;

	program.flag_zero = program.fetched_byte == 0;
	program.flag_negative = (program.fetched_byte as i8) < 0;
}

pub fn LDY(program: &mut Program, _amode: &AddressMode) {
	program.reg_y = program.fetched_byte;

	program.flag_zero = program.fetched_byte == 0;
	program.flag_negative = (program.fetched_byte as i8) < 0;
}

pub fn LSR(program: &mut Program, amode: &AddressMode) {
	let (result, prev) = match amode {
		AddressMode::Accumulator => {
			let p = program.reg_a;
			program.reg_a >>= 1;
			(program.reg_a, p)
		},
		_ => {
			let res = program.fetched_byte >> 1;
			program.set_memory(program.abs_address, res);
			(res, program.fetched_byte)
		}
	};

	program.flag_zero = result == 0;
	program.flag_negative = false;
	program.flag_carry = result > prev;
}

pub fn NOP(_program: &mut Program, _amode: &AddressMode) {

}

pub fn ORA(program: &mut Program, _amode: &AddressMode) {
	let result = program.reg_a | program.fetched_byte;
	program.reg_a = result;
	
	program.flag_zero = result == 0;
	program.flag_negative = (result as i8) < 0;
}

pub fn PHA(program: &mut Program, _amode: &AddressMode) {
	program.stack_push(program.reg_a);
}

pub fn PHP(program: &mut Program, _amode: &AddressMode) {
	let mut result = 0u8;
	result |= program.flag_carry as u8;
	result |= (program.flag_zero as u8) << 1;
	result |= (program.flag_interrupt as u8) << 2;
	result |= (program.flag_decimal as u8) << 3;
	result |= (program.flag_break as u8) << 4;
	result |= (program.flag_overflow as u8) << 6;
	result |= (program.flag_negative as u8) << 7;

	program.stack_push(result);
}

pub fn PLA(program: &mut Program, _amode: &AddressMode) {
	program.reg_a = program.stack_pull();
}

pub fn PLP(program: &mut Program, _amode: &AddressMode) {
	let status = program.stack_pull();

	program.flag_carry = (status & 1) == 1;
	program.flag_zero = ((status >> 1) & 1) == 1;
	program.flag_interrupt = ((status >> 2) & 1) == 1;
	program.flag_decimal = ((status >> 3) & 1) == 1;
	program.flag_break = ((status >> 4) & 1) == 1;
	program.flag_overflow = ((status >> 6) & 1) == 1;
	program.flag_negative = ((status >> 7) & 1) == 1;
}

pub fn ROL(program: &mut Program, amode: &AddressMode) {
	let (result, prev) = match amode {
		AddressMode::Accumulator => {
			let p = program.reg_a;
			program.reg_a = program.reg_a.rotate_left(1);
			(program.reg_a, p)
		},
		_ => {
			let res = program.fetched_byte.rotate_left(1);
			program.set_memory(program.abs_address, res);
			(res, program.fetched_byte)
		}
	};

	program.flag_zero = result == 0;
	program.flag_negative = (result as i8) < 0;
	program.flag_carry = result < prev;
}

pub fn ROR(program: &mut Program, amode: &AddressMode) {
	let (result, prev) = match amode {
		AddressMode::Accumulator => {
			let p = program.reg_a;
			program.reg_a = program.reg_a.rotate_right(1);
			(program.reg_a, p)
		},
		_ => {
			let res = program.fetched_byte.rotate_right(1);
			program.set_memory(program.abs_address, res);
			(res, program.fetched_byte)
		}
	};

	program.flag_zero = result == 0;
	program.flag_negative = (result as i8) < 0;
	program.flag_carry = result > prev;
}

pub fn RTI(program: &mut Program, _amode: &AddressMode) {
	// todo
}

pub fn RTS(program: &mut Program, _amode: &AddressMode) {
	let hi = program.stack_pull();
	let lo = program.stack_pull();
	program.program_counter = make_u16(lo, hi);
}

pub fn SBC(program: &mut Program, _amode: &AddressMode) {
	let prev = program.reg_a;
	let result: u8;
	if !program.flag_decimal {
		result = program.reg_a.wrapping_sub(program.fetched_byte + !program.flag_carry as u8);
	}
	else {
		result = program.reg_a.wrapping_sub(program.fetched_byte + !program.flag_carry as u8);
	}

	program.reg_a = result;

	program.flag_zero = program.reg_a == 0;
	program.flag_negative = (program.reg_a as i8) < 0;
	program.flag_carry = result < prev;
	program.flag_overflow = (result as i8).signum() != (prev as i8).signum();
}

pub fn SEC(program: &mut Program, _amode: &AddressMode) {
	program.flag_carry = true;
}

pub fn SED(program: &mut Program, _amode: &AddressMode) {
	program.flag_decimal = true;
}

pub fn SEI(program: &mut Program, _amode: &AddressMode) {
	program.flag_interrupt = true;
}

pub fn STA(program: &mut Program, _amode: &AddressMode) {
	program.set_memory(program.abs_address, program.reg_a);
}

pub fn STX(program: &mut Program, _amode: &AddressMode) {
	program.set_memory(program.abs_address, program.reg_x);
}

pub fn STY(program: &mut Program, _amode: &AddressMode) {
	program.set_memory(program.abs_address, program.reg_y);
}

pub fn TAX(program: &mut Program, _amode: &AddressMode) {
	program.reg_x = program.reg_a;

	program.flag_negative = (program.reg_x as i8) < 0;
	program.flag_zero = program.reg_x == 0;
}

pub fn TAY(program: &mut Program, _amode: &AddressMode) {
	program.reg_y = program.reg_a;

	program.flag_negative = (program.reg_y as i8) < 0;
	program.flag_zero = program.reg_y == 0;
}

pub fn TSX(program: &mut Program, _amode: &AddressMode) {
	program.reg_x = program.stack_pointer;

	program.flag_negative = (program.reg_x as i8) < 0;
	program.flag_zero = program.reg_x == 0;
}

pub fn TXA(program: &mut Program, _amode: &AddressMode) {
	program.reg_a = program.reg_x;

	program.flag_negative = (program.reg_a as i8) < 0;
	program.flag_zero = program.reg_a == 0;
}

pub fn TXS(program: &mut Program, _amode: &AddressMode) {
	program.stack_pointer = program.reg_x;
}

pub fn TYA(program: &mut Program, _amode: &AddressMode) {
	program.reg_a = program.reg_y;

	program.flag_negative = (program.reg_a as i8) < 0;
	program.flag_zero = program.reg_a == 0;
}
