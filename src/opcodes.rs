// opcodes.rs

use std::collections::HashMap;
use crate::num_derive::FromPrimitive;

use lazy_static::lazy_static;

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

#[allow(non_camel_case_types)]
#[derive(FromPrimitive, PartialEq)]
pub enum Instruction {
	ADC,
	AND,
	ASL,
	BCC,
	BCS,
	BEQ,
	BIT,
	BMI,
	BNE,
	BPL,
	BRK,
	BVC,
	BVS,
	CLC,
	CLD,
	CLI,
	CLV,
	CMP,
	CPX,
	CPY,
	DEC,
	DEX,
	DEY,
	EOR,
	INC,
	INX,
	INY,
	JMP,
	JSR,
	LDA,
	LDX,
	LDY,
	LSR,
	NOP,
	ORA,
	PHA,
	PHP,
	PLA,
	PLP,
	ROL,
	ROR,
	RTI,
	RTS,
	SBC,
	SEC,
	SED,
	SEI,
	STA,
	STX,
	STY,
	TAX,
	TAY,
	TSX,
	TXA,
	TXS,
	TYA,
}

#[allow(non_camel_case_types)]
#[derive(FromPrimitive, PartialEq, Eq, Hash)]
pub enum Opcode {
	ADC_imm = 0x69,
	ADC_zpg = 0x65,
	ADC_zpx = 0x75,
	ADC_abs = 0x6d,
	ADC_abx = 0x7d,
	ADC_aby = 0x79,
	ADC_idx = 0x61,
	ADC_idy = 0x71,

	AND_imm = 0x29,
	AND_zpg = 0x25,
	AND_zpx = 0x35,
	AND_abs = 0x2d,
	AND_abx = 0x3d,
	AND_aby = 0x39,
	AND_idx = 0x21,
	AND_idy = 0x31,

	ASL_acc = 0x0a,
	ASL_zpg = 0x06,
	ASL_zpx = 0x16,
	ASL_abs = 0x0e,
	ASL_abx = 0x1e,

	BCC_rel = 0x90,

	BCS_rel = 0xb0,

	BEQ_rel = 0xf0,

	BIT_zpg = 0x24,
	BIT_abs = 0x2c,

	BMI_rel = 0x30,

	BNE_rel = 0xd0,

	BPL_rel = 0x10,

	BRK_imp = 0x00,

	BVC_rel = 0x50,

	BVS_rel = 0x70,

	CLC_imp = 0x18,

	CLD_imp = 0xd8,

	CLI_imp = 0x58,

	CLV_imp = 0xb8,

	CMP_imm = 0xc9,
	CMP_zpg = 0xc5,
	CMP_zpx = 0xd5,
	CMP_abs = 0xcd,
	CMP_abx = 0xdd,
	CMP_aby = 0xd9,
	CMP_idx = 0xc1,
	CMP_idy = 0xd1,

	CPX_imm = 0xe0,
	CPX_zpg = 0xe4,
	CPX_abs = 0xec,

	CPY_imm = 0xc0,
	CPY_zpg = 0xc4,
	CPY_abs = 0xcc,
	
	DEC_zpg = 0xc6,
	DEC_zpx = 0xd6,
	DEC_abs = 0xce,
	DEC_abx = 0xde,

	DEX_imp = 0xca,

	DEY_imp = 0x88,

	EOR_imm = 0x49,
	EOR_zpg = 0x45,
	EOR_zpx = 0x55,
	EOR_abs = 0x4d,
	EOR_abx = 0x5d,
	EOR_aby = 0x59,
	EOR_idx = 0x41,
	EOR_idy = 0x51,

	INC_zpg = 0xe6,
	INC_zpx = 0xf6,
	INC_abs = 0xee,
	INC_abx = 0xfe,

	INX_imp = 0xe8,

	INY_imp = 0xc8,

	JMP_abs = 0x4c,
	JMP_ind = 0x6c,

	JSR_abs = 0x20,

	LDA_imm = 0xa9,
	LDA_zpg = 0xa5,
	LDA_zpx = 0xb5,
	LDA_abs = 0xad,
	LDA_abx = 0xbd,
	LDA_aby = 0xb9,
	LDA_idx = 0xa1,
	LDA_idy = 0xb1,

	LDX_imm = 0xa2,
	LDX_zpg = 0xa6,
	LDX_zpy = 0xb6,
	LDX_abs = 0xae,
	LDX_aby = 0xbe,

	LDY_imm = 0xa0,
	LDY_zpg = 0xa4,
	LDY_zpx = 0xb4,
	LDY_abs = 0xac,
	LDY_abx = 0xbc,

	LSR_acc = 0x4a,
	LSR_zpg = 0x46,
	LSR_zpx = 0x56,
	LSR_abs = 0x4e,
	LSR_abx = 0x5e,

	NOP_imp = 0xea,

	ORA_imm = 0x09,
	ORA_zpg = 0x05,
	ORA_zpx = 0x15,
	ORA_abs = 0x0d,
	ORA_abx = 0x1d,
	ORA_aby = 0x19,
	ORA_idx = 0x01,
	ORA_idy = 0x11,

	PHA_imp = 0x48,

	PHP_imp = 0x08,

	PLA_imp = 0x68,

	PLP_imp = 0x28,

	ROL_acc = 0x2a,
	ROL_zpg = 0x26,
	ROL_zpx = 0x36,
	ROL_abs = 0x2e,
	ROL_abx = 0x3e,

	ROR_acc = 0x6a,
	ROR_zpg = 0x66,
	ROR_zpx = 0x76,
	ROR_abs = 0x6e,
	ROR_abx = 0x7e,

	RTI_imp = 0x40,

	RTS_imp = 0x60,

	SBC_imm = 0xe9,
	SBC_zpg = 0xe5,
	SBC_zpx = 0xf5,
	SBC_abs = 0xed,
	SBC_abx = 0xfd,
	SBC_aby = 0xf9,
	SBC_idx = 0xe1,
	SBC_idy = 0xf1,

	SEC_imp = 0x38,

	SED_imp = 0xf8,

	SEI_imp = 0x78,

	STA_zpg = 0x85,
	STA_zpx = 0x95,
	STA_abs = 0x8d,
	STA_abx = 0x9d,
	STA_aby = 0x99,
	STA_idx = 0x81,
	STA_idy = 0x91,

	STX_zpg = 0x86,
	STX_zpx = 0x96,
	STX_abs = 0x8e,

	STY_zpg = 0x84,
	STY_zpx = 0x94,
	STY_abs = 0x8c,

	TAX_imp = 0xaa,
	
	TAY_imp = 0xa8,

	TSX_imp = 0xba,

	TXA_imp = 0x8a,

	TXS_imp = 0x9a,

	TYA_imp = 0x98,
}

lazy_static! {
	pub static ref OPERAND_COUNTS: HashMap<Opcode, u8> = {
		let mut map = HashMap::new();

		map.insert(Opcode::ADC_imm , 1);
		map.insert(Opcode::ADC_zpg , 1);
		map.insert(Opcode::ADC_zpx , 1);
		map.insert(Opcode::ADC_abs , 1);
		map.insert(Opcode::ADC_abx , 1);
		map.insert(Opcode::ADC_aby , 1);
		map.insert(Opcode::ADC_idx , 1);
		map.insert(Opcode::ADC_idy , 1);

		map.insert(Opcode::AND_imm , 1);
		map.insert(Opcode::AND_zpg , 1);
		map.insert(Opcode::AND_zpx , 1);
		map.insert(Opcode::AND_abs , 1);
		map.insert(Opcode::AND_abx , 1);
		map.insert(Opcode::AND_aby , 1);
		map.insert(Opcode::AND_idx , 1);
		map.insert(Opcode::AND_idy , 1);

		map.insert(Opcode::ASL_acc , 1);
		map.insert(Opcode::ASL_zpg , 1);
		map.insert(Opcode::ASL_zpx , 1);
		map.insert(Opcode::ASL_abs , 1);
		map.insert(Opcode::ASL_abx , 1);

		map.insert(Opcode::BCC_rel , 1);

		map.insert(Opcode::BCS_rel , 1);

		map.insert(Opcode::BEQ_rel , 1);

		map.insert(Opcode::BIT_zpg , 1);
		map.insert(Opcode::BIT_abs , 1);

		map.insert(Opcode::BMI_rel , 1);

		map.insert(Opcode::BNE_rel , 1);

		map.insert(Opcode::BPL_rel , 1);

		map.insert(Opcode::BRK_imp , 1);

		map.insert(Opcode::BVC_rel , 1);

		map.insert(Opcode::BVS_rel , 1);

		map.insert(Opcode::CLC_imp , 1);

		map.insert(Opcode::CLD_imp , 1);

		map.insert(Opcode::CLI_imp , 1);

		map.insert(Opcode::CLV_imp , 1);

		map.insert(Opcode::CMP_imm , 1);
		map.insert(Opcode::CMP_zpg , 1);
		map.insert(Opcode::CMP_zpx , 1);
		map.insert(Opcode::CMP_abs , 1);
		map.insert(Opcode::CMP_abx , 1);
		map.insert(Opcode::CMP_aby , 1);
		map.insert(Opcode::CMP_idx , 1);
		map.insert(Opcode::CMP_idy , 1);

		map.insert(Opcode::CPX_imm , 1);
		map.insert(Opcode::CPX_zpg , 1);
		map.insert(Opcode::CPX_abs , 1);

		map.insert(Opcode::CPY_imm , 1);
		map.insert(Opcode::CPY_zpg , 1);
		map.insert(Opcode::CPY_abs , 1);
		
		map.insert(Opcode::DEC_zpg , 1);
		map.insert(Opcode::DEC_zpx , 1);
		map.insert(Opcode::DEC_abs , 1);
		map.insert(Opcode::DEC_abx , 1);

		map.insert(Opcode::DEX_imp , 1);

		map.insert(Opcode::DEY_imp , 1);

		map.insert(Opcode::EOR_imm , 1);
		map.insert(Opcode::EOR_zpg , 1);
		map.insert(Opcode::EOR_zpx , 1);
		map.insert(Opcode::EOR_abs , 1);
		map.insert(Opcode::EOR_abx , 1);
		map.insert(Opcode::EOR_aby , 1);
		map.insert(Opcode::INC_zpx , 1);
		map.insert(Opcode::INC_abs , 1);
		map.insert(Opcode::INC_abx , 1);

		map.insert(Opcode::INX_imp , 1);

		map.insert(Opcode::INY_imp , 1);

		map.insert(Opcode::JMP_abs , 1);
		map.insert(Opcode::JMP_ind , 1);

		map.insert(Opcode::JSR_abs , 1);

		map.insert(Opcode::LDA_imm , 1);
		map.insert(Opcode::LDA_zpg , 1);
		map.insert(Opcode::LDA_zpx , 1);
		map.insert(Opcode::LDA_abs , 1);
		map.insert(Opcode::LDA_abx , 1);
		map.insert(Opcode::LDA_aby , 1);
		map.insert(Opcode::LDA_idx , 1);
		map.insert(Opcode::LDA_idy , 1);

		map.insert(Opcode::LDX_imm , 1);
		map.insert(Opcode::LDX_zpg , 1);
		map.insert(Opcode::LDX_zpy , 1);
		map.insert(Opcode::LDX_abs , 1);
		map.insert(Opcode::LDX_aby , 1);

		map.insert(Opcode::LDY_imm , 1);
		map.insert(Opcode::LDY_zpg , 1);
		map.insert(Opcode::LDY_zpx , 1);
		map.insert(Opcode::LDY_abs , 1);
		map.insert(Opcode::LDY_abx , 1);

		map.insert(Opcode::LSR_acc , 1);
		map.insert(Opcode::LSR_zpg , 1);
		map.insert(Opcode::LSR_zpx , 1);
		map.insert(Opcode::LSR_abs , 1);
		map.insert(Opcode::LSR_abx , 1);

		map.insert(Opcode::NOP_imp , 1);

		map.insert(Opcode::ORA_imm , 1);
		map.insert(Opcode::ORA_zpg , 1);
		map.insert(Opcode::ORA_zpx , 1);
		map.insert(Opcode::ORA_abs , 1);
		map.insert(Opcode::ORA_abx , 1);
		map.insert(Opcode::ORA_aby , 1);
		map.insert(Opcode::ORA_idx , 1);
		map.insert(Opcode::ORA_idy , 1);

		map.insert(Opcode::PHA_imp , 1);

		map.insert(Opcode::PHP_imp , 1);

		map.insert(Opcode::PLA_imp , 1);

		map.insert(Opcode::PLP_imp , 1);

		map.insert(Opcode::ROL_acc , 1);
		map.insert(Opcode::ROL_zpg , 1);
		map.insert(Opcode::ROL_zpx , 1);
		map.insert(Opcode::ROL_abs , 1);
		map.insert(Opcode::ROL_abx , 1);

		map.insert(Opcode::ROR_acc , 1);
		map.insert(Opcode::ROR_zpg , 1);
		map.insert(Opcode::ROR_zpx , 1);
		map.insert(Opcode::ROR_abs , 1);
		map.insert(Opcode::ROR_abx , 1);

		map.insert(Opcode::RTI_imp , 1);

		map.insert(Opcode::RTS_imp , 1);

		map.insert(Opcode::SBC_imm , 1);
		map.insert(Opcode::SBC_zpg , 1);
		map.insert(Opcode::SBC_zpx , 1);
		map.insert(Opcode::SBC_abs , 1);
		map.insert(Opcode::SBC_abx , 1);
		map.insert(Opcode::SBC_aby , 1);
		map.insert(Opcode::SBC_idx , 1);
		map.insert(Opcode::SBC_idy , 1);

		map.insert(Opcode::SEC_imp , 1);

		map.insert(Opcode::SED_imp , 1);

		map.insert(Opcode::SEI_imp , 1);

		map.insert(Opcode::STA_zpg , 1);
		map.insert(Opcode::STA_zpx , 1);
		map.insert(Opcode::STA_abs , 1);
		map.insert(Opcode::STA_abx , 1);
		map.insert(Opcode::STA_aby , 1);
		map.insert(Opcode::STA_idx , 1);
		map.insert(Opcode::STA_idy , 1);

		map.insert(Opcode::STX_zpg , 1);
		map.insert(Opcode::STX_zpx , 1);
		map.insert(Opcode::STX_abs , 1);

		map.insert(Opcode::STY_zpg , 1);
		map.insert(Opcode::STY_zpx , 1);
		map.insert(Opcode::STY_abs , 1);

		map.insert(Opcode::TAX_imp , 1);
		
		map.insert(Opcode::TAY_imp , 1);

		map.insert(Opcode::TSX_imp , 1);

		map.insert(Opcode::TXA_imp , 1);

		map.insert(Opcode::TXS_imp , 1);

		map.insert(Opcode::TYA_imp , 1);

		map
	};
}

