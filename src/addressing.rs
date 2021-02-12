// addressing.rs

use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::program::Program;

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

type AddrFunc = fn(&mut Program) -> ();

lazy_static! {
	pub static ref ADDRESS_FUNCS: HashMap<AddressMode, AddrFunc> = {
		let mut map = HashMap::new();

		map.insert(AddressMode::Implied, addr_implied as AddrFunc);
		map.insert(AddressMode::Accumulator, addr_relative_immediate as AddrFunc);
		map.insert(AddressMode::Relative, addr_relative_immediate as AddrFunc);
		map.insert(AddressMode::Immediate, addr_relative_immediate as AddrFunc);
		map.insert(AddressMode::Zeropage, addr_zeropage as AddrFunc);
		map.insert(AddressMode::ZeropageX, addr_zeropage_x as AddrFunc);
		map.insert(AddressMode::ZeropageY, addr_zeropage_y as AddrFunc);
		map.insert(AddressMode::Absolute, addr_absolute as AddrFunc);
		map.insert(AddressMode::AbsoluteX, addr_absolute_x as AddrFunc);
		map.insert(AddressMode::AbsoluteY, addr_absolute_y as AddrFunc);
		map.insert(AddressMode::Indirect, addr_indirect as AddrFunc);
		map.insert(AddressMode::IndirectX, addr_x_indirect as AddrFunc);
		map.insert(AddressMode::IndirectY, addr_indirect_y as AddrFunc);

		map
	};
}

pub fn make_u16(lo: u8, hi: u8) -> u16 {
	((hi as u16) << 8) | (lo as u16)
}

fn fetch_byte(program: &mut Program) -> u8 {
	let byte = program.get_memory(program.program_counter);
	program.advance_counter();
	byte
}

// =============================================================

pub fn addr_implied(_program: &mut Program) {

}

pub fn addr_relative_immediate(program: &mut Program) {
	let byte = fetch_byte(program);
	program.fetched_byte = byte;
	program.rel_address = byte as i8;
}

pub fn addr_zeropage(program: &mut Program) {
	let addr = fetch_byte(program);
	program.abs_address = addr as u16;
	program.fetched_byte = program.get_memory(addr as u16);
}

pub fn addr_zeropage_x(program: &mut Program) {
	let addr = fetch_byte(program);
	program.abs_address = addr as u16;
	program.fetched_byte = program.get_memory(addr as u16 + program.reg_x as u16);
}

pub fn addr_zeropage_y(program: &mut Program) {
	let addr = fetch_byte(program);
	program.abs_address = addr as u16;
	program.fetched_byte = program.get_memory(addr as u16 + program.reg_y as u16);
}

pub fn addr_absolute(program: &mut Program) {
	let lo = fetch_byte(program);
	let hi = fetch_byte(program);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr);
}

pub fn addr_absolute_x(program: &mut Program) {
	let lo = fetch_byte(program);
	let hi = fetch_byte(program);
	let addr = make_u16(lo, hi);
	program.abs_address = addr + program.reg_x as u16;
	program.fetched_byte = program.get_memory(addr + program.reg_x as u16);
}

pub fn addr_absolute_y(program: &mut Program) {
	let lo = fetch_byte(program);
	let hi = fetch_byte(program);
	let addr = make_u16(lo, hi);
	program.abs_address = addr + program.reg_y as u16;
	program.fetched_byte = program.get_memory(addr + program.reg_y as u16);
}

pub fn addr_indirect(program: &mut Program) {
	let lo = fetch_byte(program);
	let hi = fetch_byte(program);
	let addr = make_u16(lo, hi);
	program.ind_address = addr;

	let lo_abs = program.get_memory(addr);
	
	// REPLICATE PAGE CHANGE BUG
	let hi_abs = if lo == 0xff { program.get_memory(addr + 1 - 0xff) } else { program.get_memory(addr + 1) };

	let addr_abs = make_u16(lo_abs, hi_abs);
	program.abs_address = addr_abs;
	program.fetched_byte = program.get_memory(addr_abs);
}

pub fn addr_x_indirect(program: &mut Program) {
	let byte = fetch_byte(program);
	program.ind_address = byte as u16;
	let lo = program.get_memory(byte as u16 + program.reg_x as u16);
	let hi = program.get_memory(byte as u16 + 1 + program.reg_x as u16);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr);
}

pub fn addr_indirect_y(program: &mut Program) {
	let byte = fetch_byte(program);
	program.ind_address = byte as u16;
	let lo = program.get_memory(byte as u16);
	let hi = program.get_memory(byte as u16 + 1);
	let addr = make_u16(lo, hi);
	program.abs_address = addr;
	program.fetched_byte = program.get_memory(addr + program.reg_y as u16);
}
