extern crate num;
extern crate num_derive;
extern crate lazy_static;
extern crate strum_macros;

#[macro_use]
mod console_colors;
mod addressing;
mod opcodes;
mod instructions;
mod program;
mod debug;

use std::io::{self, Write};
use std::fs::File;
use std::{env, process};
use std::u16;

use crate::program::Program;
use crate::opcodes::{INSTRUCTION_DATA, Opcode};
use crate::addressing::ADDRESS_FUNCS;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
//use fltk::{app::*, window::*, button::*, frame::*};

// =======================================================================

fn get_input_args(input: &String) -> Vec<String> {
	let trimmed = input.trim_start().trim_end().trim_matches('\n').to_lowercase();
	let result = trimmed.as_str();
	
	let mut vec = Vec::<String>::new();
	for st in result.split_whitespace() {
		vec.push(String::from(st));
	}

	vec
}

// =======================================================================

fn load_program_file(filename: &String) -> Program {
	let mut file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
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
	program.origin = origin;
	program
}

fn run_program(program: &mut Program, debug_mode: bool) -> bool {
	println!("Running program from ${:x}", program.origin);
	program.program_counter = program.origin;
	program.flag_break = false;
	loop {
		let addr = program.program_counter;

		let byte = program.get_memory(program.program_counter);
		program.advance_counter();
		let opcode_test = FromPrimitive::from_u8(byte);
		let opcode: Opcode;
		match opcode_test {
			None => {
				eprintln!("{}Error:{} Invalid opcode (${:x})", con_red!(), con_reset!(), byte);
				return false;
			},
			Some(oc) => {
				opcode = oc;
			}
		}
		
		let instr_data = &INSTRUCTION_DATA[&opcode];
		let addr_func = ADDRESS_FUNCS[&instr_data.amode];
		addr_func(program);

		debug::print_instruction(program, addr, byte, &opcode, instr_data);

		let contains = program.breakpoints.contains(&addr);
		if debug_mode && (contains || program.broken) {
			if contains {
				println!("BREAKPOINT at ${:x}", addr);
			}
			
			program.broken = true;
			let mut input = String::new();
			loop {
				input.clear();
				io::stdout().flush().unwrap();
				io::stdin().read_line(&mut input).unwrap();

				let cmd_args = get_input_args(&input);

				match cmd_args[0].as_str() {
					"continue" => {
						program.broken = false;
						break;
					},

					"step" => {
						break;
					},

					"stop" => {
						return true;
					}

					"memory" | "mem" => {
						debug::print_memory(&program, &cmd_args);
					},

					_ => {
						eprintln!("{}Error:{} Invalid command", con_red!(), con_reset!());
					},
				}
			}
		}

		(instr_data.func)(program, &instr_data.amode);

		debug::print_status(program);

		if program.flag_break {
			println!("BREAK at ${:x}", addr);
			return true;
		}
	}
}

fn print_help() {
	println!("\n{0}load {1}[filename]    {2}Load a program file
{0}breakpoint {1}[address]    {2}Set breakpoint at address
{0}run    {2}Run program
{0}debug    {2}Run program in debug mode, stopping at breakpoints
{0}memory {1}[from] [range]    {2}Prints contents of memory at address and [range] addresses afterward
{0}gui    {2}Launch GUI (Not yet implemented)
{0}help    {2}Print this help text
{0}exit    {2}Exit fe6502\n", con_green!(), con_yellow!(), con_reset!());
}

fn print_help_debugger() {
	
}

// =======================================================================

fn main() {
	let mut program = Program::new();

	// Argument checks
	let args: Vec<String> = env::args().collect();
	let mut i = 1;
	while i < args.len() {
		match args[i].as_str() {
			"-l" => { // Load file
				program = load_program_file(&args[i + 1]);
				i += 1;
			},

			"-g" => { // Launch with GUI
				eprintln!("{}Error:{} GUI not yet supported", con_red!(), con_reset!());
				process::exit(1);
			},

			_ => {
				println!("Unknown parameter \"{}\"", args[i]);
			},
		}

		i += 1;
	}

	let mut input = String::new();
	loop {
		print!("{}fe6502{}> ", con_green!(), con_reset!());
		input.clear();
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input).unwrap();

		let cmd_args = get_input_args(&input);
		match cmd_args[0].as_str() {
			"load" => {
				program = load_program_file(&cmd_args[1]);
			},

			"breakpoint" | "bkpt" => {
				let addr_str = &cmd_args[1];
				let addr = u16::from_str_radix(addr_str.trim_start_matches('$'), 16).unwrap();
				program.add_breakpoint(addr);
				println!("Breakpoint set at {}${:x}{}", con_red!(), addr, con_reset!());
			},

			"run" => {
				run_program(&mut program, false);
			},

			"debug" | "db" | "dbg" => {
				run_program(&mut program, true);
			},

			"memory" | "mem" => {
				debug::print_memory(&program, &cmd_args);
			},

			"gui" => {
				eprintln!("{}Error:{} GUI not yet supported", con_red!(), con_reset!());
			},

			"help" => {
				print_help();
			},

			"exit" => {
				break;
			},

			_ => {
				eprintln!("{}Error:{} Invalid command", con_red!(), con_reset!());
			}
		}
	}
}
