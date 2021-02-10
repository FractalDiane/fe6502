// opcodes.rs

use std::collections::HashMap;
//use std::string::ToString;
use strum_macros;

use lazy_static::lazy_static;

use crate::num_derive::FromPrimitive;
use crate::addressing::AddressMode;
use crate::program::Program;
use crate::instructions;

#[allow(non_camel_case_types)]
#[derive(FromPrimitive, PartialEq, Eq, Hash, strum_macros::ToString)]
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

type InstrFunc = fn(&mut Program, &AddressMode) -> ();

pub struct InstructionData {
	pub amode: AddressMode,
	pub func: InstrFunc,
}

lazy_static! {
	pub static ref INSTRUCTION_DATA: HashMap<Opcode, InstructionData> = {
		let mut map = HashMap::new();
		
		map.insert(Opcode::ADC_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::ADC as InstrFunc});
		map.insert(Opcode::ADC_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::ADC as InstrFunc});
		map.insert(Opcode::ADC_zpx ,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::ADC as InstrFunc});
		map.insert(Opcode::ADC_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::ADC as InstrFunc});
		map.insert(Opcode::ADC_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::ADC as InstrFunc});
		map.insert(Opcode::ADC_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::ADC as InstrFunc});
		map.insert(Opcode::ADC_idx,  InstructionData{amode: AddressMode::IndirectX, func: instructions::ADC as InstrFunc});
		map.insert(Opcode::ADC_idy,  InstructionData{amode: AddressMode::IndirectY, func: instructions::ADC as InstrFunc});

		map.insert(Opcode::AND_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::AND as InstrFunc});
		map.insert(Opcode::AND_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::AND as InstrFunc});
		map.insert(Opcode::AND_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::AND as InstrFunc});
		map.insert(Opcode::AND_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::AND as InstrFunc});
		map.insert(Opcode::AND_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::AND as InstrFunc});
		map.insert(Opcode::AND_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::AND as InstrFunc});
		map.insert(Opcode::AND_idx,  InstructionData{amode: AddressMode::IndirectX, func: instructions::AND as InstrFunc});
		map.insert(Opcode::AND_idy,  InstructionData{amode: AddressMode::IndirectY, func: instructions::AND as InstrFunc});

		map.insert(Opcode::ASL_acc,  InstructionData{amode: AddressMode::Accumulator, func: instructions::ASL as InstrFunc});
		map.insert(Opcode::ASL_zpg , InstructionData{amode: AddressMode::Zeropage, func: instructions::ASL as InstrFunc});
		map.insert(Opcode::ASL_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::ASL as InstrFunc});
		map.insert(Opcode::ASL_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::ASL as InstrFunc});
		map.insert(Opcode::ASL_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::ASL as InstrFunc});

		map.insert(Opcode::BCC_rel,  InstructionData{amode: AddressMode::Relative, func: instructions::BCC as InstrFunc});

		map.insert(Opcode::BCS_rel,  InstructionData{amode: AddressMode::Relative, func: instructions::BCS as InstrFunc});

		map.insert(Opcode::BEQ_rel,  InstructionData{amode: AddressMode::Relative, func: instructions::BEQ as InstrFunc});

		map.insert(Opcode::BIT_zpg , InstructionData{amode: AddressMode::Zeropage, func: instructions::BIT as InstrFunc});
		map.insert(Opcode::BIT_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::BIT as InstrFunc});

		map.insert(Opcode::BMI_rel,  InstructionData{amode: AddressMode::Relative, func: instructions::BMI as InstrFunc});

		map.insert(Opcode::BNE_rel,  InstructionData{amode: AddressMode::Relative, func: instructions::BNE as InstrFunc});

		map.insert(Opcode::BPL_rel,  InstructionData{amode: AddressMode::Relative, func: instructions::BPL as InstrFunc});

		map.insert(Opcode::BRK_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::BRK as InstrFunc});

		map.insert(Opcode::BVC_rel,  InstructionData{amode: AddressMode::Relative, func: instructions::BVC as InstrFunc});

		map.insert(Opcode::BVS_rel,  InstructionData{amode: AddressMode::Relative, func: instructions::BVS as InstrFunc});

		map.insert(Opcode::CLC_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::CLC as InstrFunc});

		map.insert(Opcode::CLD_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::CLD as InstrFunc});

		map.insert(Opcode::CLI_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::CLI as InstrFunc});

		map.insert(Opcode::CLV_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::CLV as InstrFunc});

		map.insert(Opcode::CMP_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::CMP as InstrFunc});
		map.insert(Opcode::CMP_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::CMP as InstrFunc});
		map.insert(Opcode::CMP_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::CMP as InstrFunc});
		map.insert(Opcode::CMP_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::CMP as InstrFunc});
		map.insert(Opcode::CMP_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::CMP as InstrFunc});
		map.insert(Opcode::CMP_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::CMP as InstrFunc});
		map.insert(Opcode::CMP_idx,  InstructionData{amode: AddressMode::IndirectX, func: instructions::CMP as InstrFunc});
		map.insert(Opcode::CMP_idy,  InstructionData{amode: AddressMode::IndirectY, func: instructions::CMP as InstrFunc});

		map.insert(Opcode::CPX_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::CPX as InstrFunc});
		map.insert(Opcode::CPX_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::CPX as InstrFunc});
		map.insert(Opcode::CPX_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::CPX as InstrFunc});

		map.insert(Opcode::CPY_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::CPY as InstrFunc});
		map.insert(Opcode::CPY_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::CPY as InstrFunc});
		map.insert(Opcode::CPY_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::CPY as InstrFunc});
		
		map.insert(Opcode::DEC_zpg , InstructionData{amode: AddressMode::Zeropage, func: instructions::DEC as InstrFunc});
		map.insert(Opcode::DEC_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::DEC as InstrFunc});
		map.insert(Opcode::DEC_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::DEC as InstrFunc});
		map.insert(Opcode::DEC_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::DEC as InstrFunc});

		map.insert(Opcode::DEX_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::DEX as InstrFunc});

		map.insert(Opcode::DEY_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::DEY as InstrFunc});

		map.insert(Opcode::EOR_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::EOR as InstrFunc});
		map.insert(Opcode::EOR_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::EOR as InstrFunc});
		map.insert(Opcode::EOR_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::EOR as InstrFunc});
		map.insert(Opcode::EOR_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::EOR as InstrFunc});
		map.insert(Opcode::EOR_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::EOR as InstrFunc});
		map.insert(Opcode::EOR_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::EOR as InstrFunc});

		map.insert(Opcode::INC_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::INC as InstrFunc});
		map.insert(Opcode::INC_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::INC as InstrFunc});
		map.insert(Opcode::INC_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::INC as InstrFunc});

		map.insert(Opcode::INX_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::INX as InstrFunc});

		map.insert(Opcode::INY_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::INY as InstrFunc});

		map.insert(Opcode::JMP_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::JMP as InstrFunc});
		map.insert(Opcode::JMP_ind ,  InstructionData{amode: AddressMode::Indirect, func: instructions::JMP as InstrFunc});

		map.insert(Opcode::JSR_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::JSR as InstrFunc});

		map.insert(Opcode::LDA_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::LDA as InstrFunc});
		map.insert(Opcode::LDA_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::LDA as InstrFunc});
		map.insert(Opcode::LDA_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::LDA as InstrFunc});
		map.insert(Opcode::LDA_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::LDA as InstrFunc});
		map.insert(Opcode::LDA_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::LDA as InstrFunc});
		map.insert(Opcode::LDA_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::LDA as InstrFunc});
		map.insert(Opcode::LDA_idx,  InstructionData{amode: AddressMode::IndirectX, func: instructions::LDA as InstrFunc});
		map.insert(Opcode::LDA_idy,  InstructionData{amode: AddressMode::IndirectY, func: instructions::LDA as InstrFunc});

		map.insert(Opcode::LDX_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::LDX as InstrFunc});
		map.insert(Opcode::LDX_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::LDX as InstrFunc});
		map.insert(Opcode::LDX_zpy,  InstructionData{amode: AddressMode::ZeropageY, func: instructions::LDX as InstrFunc});
		map.insert(Opcode::LDX_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::LDX as InstrFunc});
		map.insert(Opcode::LDX_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::LDX as InstrFunc});

		map.insert(Opcode::LDY_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::LDY as InstrFunc});
		map.insert(Opcode::LDY_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::LDY as InstrFunc});
		map.insert(Opcode::LDY_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::LDY as InstrFunc});
		map.insert(Opcode::LDY_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::LDY as InstrFunc});
		map.insert(Opcode::LDY_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::LDY as InstrFunc});

		map.insert(Opcode::LSR_acc,  InstructionData{amode: AddressMode::Accumulator, func: instructions::LSR as InstrFunc});
		map.insert(Opcode::LSR_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::LSR as InstrFunc});
		map.insert(Opcode::LSR_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::LSR as InstrFunc});
		map.insert(Opcode::LSR_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::LSR as InstrFunc});
		map.insert(Opcode::LSR_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::LSR as InstrFunc});

		map.insert(Opcode::NOP_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::NOP as InstrFunc});

		map.insert(Opcode::ORA_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::ORA as InstrFunc});
		map.insert(Opcode::ORA_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::ORA as InstrFunc});
		map.insert(Opcode::ORA_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::ORA as InstrFunc});
		map.insert(Opcode::ORA_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::ORA as InstrFunc});
		map.insert(Opcode::ORA_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::ORA as InstrFunc});
		map.insert(Opcode::ORA_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::ORA as InstrFunc});
		map.insert(Opcode::ORA_idx,  InstructionData{amode: AddressMode::IndirectX, func: instructions::ORA as InstrFunc});
		map.insert(Opcode::ORA_idy,  InstructionData{amode: AddressMode::IndirectY, func: instructions::ORA as InstrFunc});

		map.insert(Opcode::PHA_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::PHA as InstrFunc});

		map.insert(Opcode::PHP_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::PHP as InstrFunc});

		map.insert(Opcode::PLA_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::PLA as InstrFunc});

		map.insert(Opcode::PLP_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::PLP as InstrFunc});

		map.insert(Opcode::ROL_acc,  InstructionData{amode: AddressMode::Accumulator, func: instructions::ROL as InstrFunc});
		map.insert(Opcode::ROL_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::ROL as InstrFunc});
		map.insert(Opcode::ROL_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::ROL as InstrFunc});
		map.insert(Opcode::ROL_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::ROL as InstrFunc});
		map.insert(Opcode::ROL_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::ROL as InstrFunc});

		map.insert(Opcode::ROR_acc,  InstructionData{amode: AddressMode::Accumulator, func: instructions::ROR as InstrFunc});
		map.insert(Opcode::ROR_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::ROR as InstrFunc});
		map.insert(Opcode::ROR_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::ROR as InstrFunc});
		map.insert(Opcode::ROR_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::ROR as InstrFunc});
		map.insert(Opcode::ROR_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::ROR as InstrFunc});

		map.insert(Opcode::RTI_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::RTI as InstrFunc});

		map.insert(Opcode::RTS_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::RTS as InstrFunc});

		map.insert(Opcode::SBC_imm ,  InstructionData{amode: AddressMode::Immediate, func: instructions::SBC as InstrFunc});
		map.insert(Opcode::SBC_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::SBC as InstrFunc});
		map.insert(Opcode::SBC_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::SBC as InstrFunc});
		map.insert(Opcode::SBC_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::SBC as InstrFunc});
		map.insert(Opcode::SBC_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::SBC as InstrFunc});
		map.insert(Opcode::SBC_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::SBC as InstrFunc});
		map.insert(Opcode::SBC_idx,  InstructionData{amode: AddressMode::IndirectX, func: instructions::SBC as InstrFunc});
		map.insert(Opcode::SBC_idy,  InstructionData{amode: AddressMode::IndirectY, func: instructions::SBC as InstrFunc});

		map.insert(Opcode::SEC_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::SEC as InstrFunc});

		map.insert(Opcode::SED_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::SED as InstrFunc});

		map.insert(Opcode::SEI_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::SEI as InstrFunc});

		map.insert(Opcode::STA_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::STA as InstrFunc});
		map.insert(Opcode::STA_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::STA as InstrFunc});
		map.insert(Opcode::STA_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::STA as InstrFunc});
		map.insert(Opcode::STA_abx,  InstructionData{amode: AddressMode::AbsoluteX, func: instructions::STA as InstrFunc});
		map.insert(Opcode::STA_aby,  InstructionData{amode: AddressMode::AbsoluteY, func: instructions::STA as InstrFunc});
		map.insert(Opcode::STA_idx,  InstructionData{amode: AddressMode::IndirectX, func: instructions::STA as InstrFunc});
		map.insert(Opcode::STA_idy,  InstructionData{amode: AddressMode::IndirectY, func: instructions::STA as InstrFunc});

		map.insert(Opcode::STX_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::STX as InstrFunc});
		map.insert(Opcode::STX_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::STX as InstrFunc});
		map.insert(Opcode::STX_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::STX as InstrFunc});

		map.insert(Opcode::STY_zpg ,  InstructionData{amode: AddressMode::Zeropage, func: instructions::STY as InstrFunc});
		map.insert(Opcode::STY_zpx,  InstructionData{amode: AddressMode::ZeropageX, func: instructions::STY as InstrFunc});
		map.insert(Opcode::STY_abs,  InstructionData{amode: AddressMode::Absolute, func: instructions::STY as InstrFunc});

		map.insert(Opcode::TAX_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::TAX as InstrFunc});
		
		map.insert(Opcode::TAY_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::TAY as InstrFunc});

		map.insert(Opcode::TSX_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::TSX as InstrFunc});

		map.insert(Opcode::TXA_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::TXA as InstrFunc});

		map.insert(Opcode::TXS_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::TXS as InstrFunc});

		map.insert(Opcode::TYA_imp,  InstructionData{amode: AddressMode::Implied, func: instructions::TYA as InstrFunc});
		
		map
	};
}

