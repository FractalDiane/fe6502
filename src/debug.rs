// debug.rs

use crate::addressing::AddressMode;
use crate::program::Program;
use crate::opcodes::{Opcode, InstructionData};

pub fn print_instruction(program: &mut Program, address: u16, byte: u8, opcode: &Opcode, instruction_data: &InstructionData) {
	let opcode_str = opcode.to_string();
	let mut string = format!("${:x}: {}{}{}", address, con_yellow!(), &opcode_str[0..3], con_green!());
	match instruction_data.amode {
		AddressMode::Accumulator => {
			string += " A";
		},
		AddressMode::Immediate => {
			string += format!(" #${:x}", program.fetched_byte).as_str();
		},
		AddressMode::Absolute | AddressMode::Zeropage => {
			string += format!(" ${:x}", program.abs_address).as_str();
		},
		AddressMode::AbsoluteX | AddressMode::ZeropageX => {
			string += format!(" ${:x},X", program.abs_address).as_str();
		},
		AddressMode::AbsoluteY | AddressMode::ZeropageY => {
			string += format!(" ${:x},Y", program.abs_address).as_str();
		},
		AddressMode::Relative => {
			string += format!(" ${:x}", program.program_counter.wrapping_add(program.rel_address as u16)).as_str();
		},
		AddressMode::Indirect => {
			string += format!(" (${:x})", program.ind_address).as_str();
		},
		AddressMode::IndirectX => {
			string += format!(" (${:x},X)", program.ind_address).as_str();
		},
		AddressMode::IndirectY => {
			string += format!(" (${:x}),Y", program.ind_address).as_str();
		},
		_ => {}
	}

	let mut string_2 = String::from(con_red!());

	match instruction_data.amode {
		AddressMode::Implied | AddressMode::Accumulator => {
			string_2 += format!(" (${:02x})", byte).as_str();
		},
		AddressMode::Immediate => {
			string_2 += format!(" (${:02x} ${:02x})", byte, program.fetched_byte).as_str();
		}
		AddressMode::Zeropage | AddressMode::ZeropageX | AddressMode::ZeropageY => {
			string_2 += format!(" (${:02x} ${:02x})", byte, program.abs_address).as_str();
		},
		AddressMode::Absolute | AddressMode::AbsoluteX | AddressMode::AbsoluteY => {
			string_2 += format!(" (${:02x} ${:02x} ${:02x})", byte, program.abs_address & 0xff, program.abs_address >> 8).as_str();
		},
		AddressMode::Indirect => {
			string_2 += format!(" (${:02x} ${:02x} ${:02x})", byte, program.ind_address & 0xff, program.ind_address >> 8).as_str();
		}
		AddressMode::IndirectX | AddressMode::IndirectY => {
			string_2 += format!(" (${:02x} ${:02x})", byte, program.ind_address).as_str();
		}
		AddressMode::Relative => {
			string_2 += format!(" (${:02x} ${:02x})", byte, program.rel_address).as_str();
		}
	}

	println!("{:32} {}{}", string, string_2, con_reset!());
}

pub fn print_status(program: &mut Program) {
	let symbols = [format!("{}{}{}", con_red!(), "-", con_reset!()), format!("{}{}{}", con_green!(), "+", con_reset!())];
	println!("A    X    Y     N V B D I Z C");
	println!("{:<5}{:<5}{:<5} {} {} {} {} {} {} {}\n",
		program.reg_a, program.reg_x, program.reg_y,
		symbols[program.flag_negative as usize], symbols[program.flag_overflow as usize],
		symbols[program.flag_break as usize], symbols[program.flag_decimal as usize],
		symbols[program.flag_interrupt as usize], symbols[program.flag_zero as usize],
		symbols[program.flag_carry as usize]
	);
}

pub fn print_memory(program: &Program, cmd_args: &Vec<String>) {
		let addr_str = &cmd_args[1];
		let addr = u16::from_str_radix(addr_str.trim_start_matches('$'), 16).unwrap();
		let len = if cmd_args.len() == 3 { cmd_args[2].parse::<usize>().unwrap() } else { 10 };
		for i in 0..len {
			let byte = program.get_memory(addr + i as u16);
			println!("${:04x}: ${:<02x}   {:<3}   {}", addr + i as u16, byte, byte, if byte >= 32 && byte <= 126 { byte as char } else { '\0' });
		}
}
