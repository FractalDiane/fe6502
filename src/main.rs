extern crate num;
extern crate num_derive;
extern crate lazy_static;

use std::fs::File;
use std::{env, process};
//use fltk::{app::*, window::*, button::*, frame::*};

mod opcodes;
mod instructions;
mod program;
use program::Program;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		process::exit(1);
	}

	let mut file = File::open(&args[1])
					.expect(format!("Failed to open file {}", &args[1]).as_str());

	

	let mut program = Program::new(file);
}
