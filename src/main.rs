extern crate num;
extern crate num_derive;
extern crate lazy_static;

use std::fs::File;
use std::{env, process};
//use fltk::{app::*, window::*, button::*, frame::*};

mod addressing;
mod opcodes;
mod instructions;
mod program;
use byteorder::{LittleEndian, ReadBytesExt};
use program::Program;
use num::FromPrimitive;
use opcodes::{Opcode, INSTRUCTION_DATA};
use addressing::ADDRESS_FUNCS;

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
		let byte = program.get_memory(program.program_counter);
		let opcode: Opcode = FromPrimitive::from_u8(byte).unwrap();

		let instr_data = &INSTRUCTION_DATA[&opcode];
		let addr_func = ADDRESS_FUNCS[&instr_data.amode];
		addr_func(&mut program);

		(instr_data.func)(&mut program, &instr_data.amode);
		if program.flag_break {
			println!("BREAK at ${:x}", program.program_counter);
			return;
		}
	}
}
