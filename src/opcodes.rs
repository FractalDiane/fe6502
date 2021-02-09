// opcodes.rs

use std::collections::HashMap;
use crate::num_derive::FromPrimitive;
use crate::addressing::AddressMode;
use crate::instructions;

use lazy_static::lazy_static;

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
	pub static ref ADDRESS_MODES: HashMap<Opcode, AddressMode> = {
		let mut map = HashMap::new();
		
		map.insert(Opcode::ADC_imm ,  AddressMode::Immediate);
		map.insert(Opcode::ADC_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::ADC_zpx ,  AddressMode::ZeropageX);
		map.insert(Opcode::ADC_abs,  AddressMode::Absolute);
		map.insert(Opcode::ADC_abx,  AddressMode::AbsoluteX);
		map.insert(Opcode::ADC_aby,  AddressMode::AbsoluteY);
		map.insert(Opcode::ADC_idx,  AddressMode::IndirectX);
		map.insert(Opcode::ADC_idy,  AddressMode::IndirectY);

		map.insert(Opcode::AND_imm ,  AddressMode::Immediate);
		map.insert(Opcode::AND_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::AND_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::AND_abs,  AddressMode::Absolute);
		map.insert(Opcode::AND_abx,  AddressMode::AbsoluteX);
		map.insert(Opcode::AND_aby,  AddressMode::AbsoluteY);
		map.insert(Opcode::AND_idx,  AddressMode::IndirectX);
		map.insert(Opcode::AND_idy,  AddressMode::IndirectY);

		map.insert(Opcode::ASL_acc,  AddressMode::Accumulator);
		map.insert(Opcode::ASL_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::ASL_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::ASL_abs,  AddressMode::Absolute);
		map.insert(Opcode::ASL_abx,  AddressMode::AbsoluteX);

		map.insert(Opcode::BCC_rel,  AddressMode::Relative);

		map.insert(Opcode::BCS_rel,  AddressMode::Relative);

		map.insert(Opcode::BEQ_rel,  AddressMode::Relative);

		map.insert(Opcode::BIT_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::BIT_abs,  AddressMode::Absolute);

		map.insert(Opcode::BMI_rel,  AddressMode::Relative);

		map.insert(Opcode::BNE_rel,  AddressMode::Relative);

		map.insert(Opcode::BPL_rel,  AddressMode::Relative);

		map.insert(Opcode::BRK_imp,  AddressMode::Implied);

		map.insert(Opcode::BVC_rel,  AddressMode::Relative);

		map.insert(Opcode::BVS_rel,  AddressMode::Relative);

		map.insert(Opcode::CLC_imp,  AddressMode::Implied);

		map.insert(Opcode::CLD_imp,  AddressMode::Implied);

		map.insert(Opcode::CLI_imp,  AddressMode::Implied);

		map.insert(Opcode::CLV_imp,  AddressMode::Implied);

		map.insert(Opcode::CMP_imm ,  AddressMode::Immediate);
		map.insert(Opcode::CMP_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::CMP_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::CMP_abs,  AddressMode::Absolute);
		map.insert(Opcode::CMP_abx,  AddressMode::AbsoluteX);
		map.insert(Opcode::CMP_aby,  AddressMode::AbsoluteY);
		map.insert(Opcode::CMP_idx,  AddressMode::IndirectX);
		map.insert(Opcode::CMP_idy,  AddressMode::IndirectY);

		map.insert(Opcode::CPX_imm ,  AddressMode::Immediate);
		map.insert(Opcode::CPX_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::CPX_abs,  AddressMode::Absolute);

		map.insert(Opcode::CPY_imm ,  AddressMode::Immediate);
		map.insert(Opcode::CPY_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::CPY_abs,  AddressMode::Absolute);
		
		map.insert(Opcode::DEC_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::DEC_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::DEC_abs,  AddressMode::Absolute);
		map.insert(Opcode::DEC_abx,  AddressMode::AbsoluteX);

		map.insert(Opcode::DEX_imp,  AddressMode::Implied);

		map.insert(Opcode::DEY_imp,  AddressMode::Implied);

		map.insert(Opcode::EOR_imm ,  AddressMode::Immediate);
		map.insert(Opcode::EOR_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::EOR_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::EOR_abs,  AddressMode::Absolute);
		map.insert(Opcode::EOR_abx,  AddressMode::AbsoluteX);
		map.insert(Opcode::EOR_aby,  AddressMode::AbsoluteY);
		map.insert(Opcode::INC_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::INC_abs,  AddressMode::Absolute);
		map.insert(Opcode::INC_abx,  AddressMode::AbsoluteX);

		map.insert(Opcode::INX_imp,  AddressMode::Implied);

		map.insert(Opcode::INY_imp,  AddressMode::Implied);

		map.insert(Opcode::JMP_abs,  AddressMode::Absolute);
		map.insert(Opcode::JMP_ind ,  AddressMode::Indirect);

		map.insert(Opcode::JSR_abs,  AddressMode::Absolute);

		map.insert(Opcode::LDA_imm ,  AddressMode::Immediate);
		map.insert(Opcode::LDA_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::LDA_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::LDA_abs,  AddressMode::Absolute);
		map.insert(Opcode::LDA_abx,  AddressMode::AbsoluteX);
		map.insert(Opcode::LDA_aby,  AddressMode::AbsoluteY);
		map.insert(Opcode::LDA_idx,  AddressMode::IndirectX);
		map.insert(Opcode::LDA_idy,  AddressMode::IndirectY);

		map.insert(Opcode::LDX_imm ,  AddressMode::Immediate);
		map.insert(Opcode::LDX_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::LDX_zpy,  AddressMode::ZeropageY);
		map.insert(Opcode::LDX_abs,  AddressMode::Absolute);
		map.insert(Opcode::LDX_aby,  AddressMode::AbsoluteY);

		map.insert(Opcode::LDY_imm ,  AddressMode::Immediate);
		map.insert(Opcode::LDY_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::LDY_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::LDY_abs,  AddressMode::Absolute);
		map.insert(Opcode::LDY_abx,  AddressMode::AbsoluteX);

		map.insert(Opcode::LSR_acc,  AddressMode::Accumulator);
		map.insert(Opcode::LSR_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::LSR_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::LSR_abs,  AddressMode::Absolute);
		map.insert(Opcode::LSR_abx,  AddressMode::AbsoluteX);

		map.insert(Opcode::NOP_imp,  AddressMode::Implied);

		map.insert(Opcode::ORA_imm ,  AddressMode::Immediate);
		map.insert(Opcode::ORA_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::ORA_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::ORA_abs,  AddressMode::Absolute);
		map.insert(Opcode::ORA_abx,  AddressMode::AbsoluteX);
		map.insert(Opcode::ORA_aby,  AddressMode::AbsoluteY);
		map.insert(Opcode::ORA_idx,  AddressMode::IndirectX);
		map.insert(Opcode::ORA_idy,  AddressMode::IndirectY);

		map.insert(Opcode::PHA_imp,  AddressMode::Implied);

		map.insert(Opcode::PHP_imp,  AddressMode::Implied);

		map.insert(Opcode::PLA_imp,  AddressMode::Implied);

		map.insert(Opcode::PLP_imp,  AddressMode::Implied);

		map.insert(Opcode::ROL_acc,  AddressMode::Accumulator);
		map.insert(Opcode::ROL_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::ROL_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::ROL_abs,  AddressMode::Absolute);
		map.insert(Opcode::ROL_abx,  AddressMode::AbsoluteX);

		map.insert(Opcode::ROR_acc,  AddressMode::Accumulator);
		map.insert(Opcode::ROR_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::ROR_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::ROR_abs,  AddressMode::Absolute);
		map.insert(Opcode::ROR_abx,  AddressMode::AbsoluteX);

		map.insert(Opcode::RTI_imp,  AddressMode::Implied);

		map.insert(Opcode::RTS_imp,  AddressMode::Implied);

		map.insert(Opcode::SBC_imm ,  AddressMode::Immediate);
		map.insert(Opcode::SBC_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::SBC_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::SBC_abs,  AddressMode::Absolute);
		map.insert(Opcode::SBC_abx,  AddressMode::AbsoluteX);
		map.insert(Opcode::SBC_aby,  AddressMode::AbsoluteY);
		map.insert(Opcode::SBC_idx,  AddressMode::IndirectX);
		map.insert(Opcode::SBC_idy,  AddressMode::IndirectY);

		map.insert(Opcode::SEC_imp,  AddressMode::Implied);

		map.insert(Opcode::SED_imp,  AddressMode::Implied);

		map.insert(Opcode::SEI_imp,  AddressMode::Implied);

		map.insert(Opcode::STA_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::STA_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::STA_abs,  AddressMode::Absolute);
		map.insert(Opcode::STA_abx,  AddressMode::AbsoluteX);
		map.insert(Opcode::STA_aby,  AddressMode::AbsoluteY);
		map.insert(Opcode::STA_idx,  AddressMode::IndirectX);
		map.insert(Opcode::STA_idy,  AddressMode::IndirectY);

		map.insert(Opcode::STX_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::STX_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::STX_abs,  AddressMode::Absolute);

		map.insert(Opcode::STY_zpg ,  AddressMode::Zeropage);
		map.insert(Opcode::STY_zpx,  AddressMode::ZeropageX);
		map.insert(Opcode::STY_abs,  AddressMode::Absolute);

		map.insert(Opcode::TAX_imp,  AddressMode::Implied);
		
		map.insert(Opcode::TAY_imp,  AddressMode::Implied);

		map.insert(Opcode::TSX_imp,  AddressMode::Implied);

		map.insert(Opcode::TXA_imp,  AddressMode::Implied);

		map.insert(Opcode::TXS_imp,  AddressMode::Implied);

		map.insert(Opcode::TYA_imp,  AddressMode::Implied);
		
		map
	};
}

