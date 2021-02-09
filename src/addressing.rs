// addressing.rs

use crate::program::Program;

use std::collections::HashMap;
use std::fs::File;

use byteorder::ReadBytesExt;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, Hash)]
pub enum AddressMode {
	Implied,
	Accumulator,
	Relative,
	Immediate,
	Zeropage,
	ZeropageX,
	ZeropageY,
	Absolute,
	AbsoluteX,
	AbsoluteY,
	Indirect,
	IndirectX,
	IndirectY,
}

type AddrFunc = fn(&File, &mut Program) -> ();

lazy_static! {
	pub static ref ADDRESS_FUNCS: HashMap<AddressMode, AddrFunc> = {
		let mut map = HashMap::new();

		map.insert(AddressMode::Implied, addr_relative_immediate as AddrFunc);
		map.insert(AddressMode::Accumulator, addr_relative_immediate as AddrFunc);
		map.insert(AddressMode::Absolute, addr_absolute as AddrFunc);

		map
	};
}

fn make_u16(lo: u8, hi: u8) -> u16 {
	((hi as u16) << 8) | (lo as u16)
}

fn fetch_byte(file: &File, program: &mut Program) -> u8 {
	let byte = file.read_u8().unwrap();
	program.advance_counter();
	byte
}

// =============================================================

pub fn addr_relative_immediate(file: &File, program: &mut Program) {
	let byte = fetch_byte(file, program);
	program.fetched_byte = byte;
}

pub fn addr_zeropage(file: &File, program: &mut Program) {
	let addr = fetch_byte(file, program);
	program.abs_address = addr as u16;
	program.fetched_byte = program.get_memory(addr as u16);
}

pub fn addr_zeropage_x(file: &File, program: &mut Program) {
	let addr = fetch_byte(file, program);
	program.abs_address = addr as u16;
	program.fetched_byte = program.get_memory(addr as u16 + program.reg_x as u16);
}

pub fn addr_zeropage_y(file: &File, program: &mut Program) {
	let addr = fetch_byte(file, program);
	program.abs_address = addr as u16;
	program.fetched_byte = program.get_memory(addr as u16 + program.reg_y as u16);
}

pub fn addr_absolute(file: &File, program: &mut Program) {
	let lo = fetch_byte(file, program);
	let hi = fetch_byte(file, program);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr);
}

pub fn addr_absolute_x(file: &File, program: &mut Program) {
	let lo = fetch_byte(file, program);
	let hi = fetch_byte(file, program);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr + program.reg_x as u16);
}

pub fn addr_absolute_y(file: &File, program: &mut Program) {
	let lo = fetch_byte(file, program);
	let hi = fetch_byte(file, program);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr + program.reg_y as u16);
}

pub fn addr_indirect(file: &File, program: &mut Program) {
	let byte = fetch_byte(file, program);
	let lo = program.get_memory(byte as u16);
	let hi = program.get_memory(byte as u16 + 1);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr);
}

pub fn addr_x_indirect(file: &File, program: &mut Program) {
	let byte = fetch_byte(file, program);
	let lo = program.get_memory(byte as u16 + program.reg_x as u16);
	let hi = program.get_memory(byte as u16 + 1 + program.reg_x as u16);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr);
}

pub fn addr_indirect_y(file: &File, program: &mut Program) {
	let byte = fetch_byte(file, program);
	let lo = program.get_memory(byte as u16);
	let hi = program.get_memory(byte as u16 + 1);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr + program.reg_y as u16);
}
