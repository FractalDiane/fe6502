// instructions.rs

use crate::opcodes::{AddressMode, Opcode};
use crate::program::Program;

pub fn ADC(program: &mut Program, operands: Vec<u8>, address_mode: AddressMode) {

}


pub fn AND(program: &mut Program, operands: Vec<u8>, address_mode: AddressMode) {
	match address_mode {
		AddressMode::Immediate => {
			program.set_a(program.get_a() & operands[0]);
		}

		AddressMode::Zeropage => {
			program.set_memory(operands[0] as u16, operands[1]);
		}

		AddressMode::ZeropageX => {
			program.set_memory((operands[0] + program.get_x()) as u16, operands[1]);
		}

		AddressMode::Absolute => {
		   
		}
	}
}