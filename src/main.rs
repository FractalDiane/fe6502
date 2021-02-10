extern crate num;
extern crate num_derive;
extern crate lazy_static;
extern crate strum_macros;

use std::fs::File;
use std::{env, process};
//use fltk::{app::*, window::*, button::*, frame::*};

mod addressing;
mod opcodes;
mod instructions;
mod program;
mod console_colors;
use byteorder::{LittleEndian, ReadBytesExt};
use program::Program;
use num::FromPrimitive;
use opcodes::{Opcode, INSTRUCTION_DATA};
use addressing::{AddressMode, ADDRESS_FUNCS};

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		process::exit(1);
	}

	// Load program into memory
	let mut file = File::open(&args[1])
					.expect(format!("Failed to open file {}", &args[1]).as_str());

	let mut program = Program::new();

	let origin = file.read_u16::<LittleEndian>().unwrap();
	let mut i = 0;
	loop {
		match file.read_u8() {
			Ok(byte) => {
				program.set_memory(origin + i, byte);
				i += 1;
			},

			Err(_) => { break; }
		}
	}

	program.program_counter = origin;

	// Run program
	println!("Running program from ${:x}", origin);
	loop {
		let addr = program.program_counter;

		let byte = program.get_memory(program.program_counter);
		program.advance_counter();
		let opcode: Opcode = FromPrimitive::from_u8(byte).unwrap();
		
		let instr_data = &INSTRUCTION_DATA[&opcode];
		let addr_func = ADDRESS_FUNCS[&instr_data.amode];
		addr_func(&mut program);

		{
			let opcode_str = opcode.to_string();
			let mut string = format!("${:x}: {}{}{}", addr, con_yellow!(), &opcode_str[0..3], con_green!());
			match instr_data.amode {
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

			println!("{}{}", string, con_reset!());

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

		(instr_data.func)(&mut program, &instr_data.amode);
		if program.flag_break {
			println!("BREAK at ${:x}", addr);
			return;
		}
	}
}
