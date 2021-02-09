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
use opcodes::{Opcode, ADDRESS_MODES};
use addressing::ADDRESS_FUNCS;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		process::exit(1);
	}

	let mut file = File::open(&args[1])
					.expect(format!("Failed to open file {}", &args[1]).as_str());

	let mut program = Program::new();

	let origin = file.read_u16::<LittleEndian>().unwrap();
	loop {
		let byte = file.read_u8();
		match byte {
			Ok(byte) => {
				let opcode: Opcode = FromPrimitive::from_u8(byte).unwrap();
				let amode = ADDRESS_MODES[&opcode];
				let addr_func = ADDRESS_FUNCS[&amode];
				addr_func(&file, &mut program);
			},

			Err(_) => { break; }
		}
		
	}
}
