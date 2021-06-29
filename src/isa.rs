/// Instruction set of the emulator
/// Each instruction has an internal representation
///
/// Two enums
///     - AddrMode ie Addressing Mode
///     - Instruction
///
/// An instruction is represented  by a u32 value in the Instruction enum and AddrMode enum
/// i.e instruction in the MSBs and mode in the LSBs
/// The mode will probably be a u8 and instruction the rest
/// e.g
///     IMMEDIATE MODE => 01
///     LDA => 0x200
///
///     LDA IMMEDIATE 0x200 & 0x201
///

/// Source [masswerk.at](https://www.masswerk.at/6502/6502_instruction_set.html)
pub struct Instruction;
pub struct Mode;

#[allow(dead_code)]
impl Mode
{
pub const ACC: u32  = 0x00;
pub const ABS: u32  = 0x01;
pub const ABSX: u32 = 0x02;
pub const ABSY: u32 = 0x03;
pub const IMM: u32  = 0x04;
pub const IMP: u32  = 0x05;
pub const IND: u32  = 0x06;
pub const INDX: u32 = 0x07;
pub const INDY: u32 = 0x08;
pub const REL: u32  = 0x09;
pub const ZP: u32   = 0x0a;
pub const ZPY: u32  = 0x0b;
pub const ZPX: u32  = 0x0c;
}

#[allow(dead_code)]
impl Instruction
{
    // TODO : Fix the numbers
    pub const ADC: u32 = 0x0101;
    pub const AND: u32 = 0x0201;
    pub const ASL: u32 = 0x0301;
    pub const BCC: u32 = 0x0401;
    pub const BCS: u32 = 0x0501;
    pub const BEQ: u32 = 0x0601;
    pub const BIT: u32 = 0x0701;
    pub const BMI: u32 = 0x0801;
    pub const BNE: u32 = 0x0901;
    pub const BPL: u32 = 0x0a01;
    pub const BRK: u32 = 0x0b01;
    pub const BVC: u32 = 0x0c01;
    pub const CLC: u32 = 0x0d01;
    pub const CLD: u32 = 0x0e01;
    pub const CLI: u32 = 0x0f01;
    pub const CLV: u32 = 0x0001;
    pub const CMP: u32 = 0x0001;
    pub const CPX: u32 = 0x0001;
    pub const CPY: u32 = 0x0001;
    pub const DEC: u32 = 0x0001;
    pub const DEX: u32 = 0x0001;
    pub const DEY: u32 = 0x0001;
    pub const EOR: u32 = 0x0001;
    pub const INC: u32 = 0x0001;
    pub const INX: u32 = 0x0001;
    pub const INY: u32 = 0x0001;
    pub const JMP: u32 = 0x0001;
    pub const JSR: u32 = 0x0001;
    pub const LDA: u32 = 0x0001;
    pub const LDX: u32 = 0x0001;
    pub const LDY: u32 = 0x0001;
    pub const LSR: u32 = 0x0001;
    pub const NOP: u32 = 0x0001;
    pub const ORA: u32 = 0x0001;
    pub const PHA: u32 = 0x0001;
    pub const PHP: u32 = 0x0001;
    pub const PLA: u32 = 0x0001;
    pub const PLP: u32 = 0x0001;
    pub const ROL: u32 = 0x0001;
    pub const ROR: u32 = 0x0001;
    pub const RTI: u32 = 0x0001;
    pub const RTS: u32 = 0x0001;
    pub const SBC: u32 = 0x0001;
    pub const SEC: u32 = 0x0001;
    pub const SED: u32 = 0x0001;
    pub const SEI: u32 = 0x0001;
    pub const STA: u32 = 0x0001;
    pub const STX: u32 = 0x0001;
    pub const STY: u32 = 0x0001;
    pub const TAX: u32 = 0x0001;
    pub const TAY: u32 = 0x0001;
    pub const TSX: u32 = 0x0001;
    pub const TXA: u32 = 0x0001;
    pub const TXS: u32 = 0x0001;
    pub const TYA: u32 = 0x0001;
}

#[allow(dead_code)]
pub fn get_instruction(instruction:u32, mode: u32) -> u32
{
    let mut result: u32 = instruction;
    result = result << 2;
    result + mode
}

