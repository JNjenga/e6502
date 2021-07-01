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
///
/// Source [addressing_modes](https://wiki.cdot.senecacollege.ca/wiki/6502_Addressing_Modes#Accumulator)
/// Notes 
/// ----
/// ## Addressing Modes
/// - ACC - Works on the accumulator, can be done explicitly
/// - ABS - Data is accessed using the 16-bit address specified as a constant
/// - ABSX - Absolute,x Data is accessed using a 16-bit address specified as a constant to which the value of the X register is added (with carry)
/// - ABSY - Absolute,x Data is accessed using a 16-bit address specified as a constant to which the value of the Y register is added (with carry)
/// - IMM - Immediate mode, data is taken from the byte following the opcode
/// - IMP - Implied mode, data is implied by operations ie(nooperand)
/// - IND - Data is accessed using a (16-bit)pointer
/// - INDX - Add 8-bit address without cary with the content of the x register
/// - INDY - Same as INDX, but x register
/// - REL - 8-bit signed offset is provided. The value is added to the program counter to find the
/// effective address
/// ZP/ZPX/ZPY - are similar to IND/INDX/INDY respectively

pub struct Instruction;
pub struct Mode;

#[allow(dead_code)]
impl Mode
{
pub const ACC: u32  = 0x00;// Accumator
pub const ABS: u32  = 0x01;// Data is accessed using 16bit address specified as a constant
pub const ABSX: u32 = 0x02;// 
pub const ABSY: u32 = 0x03;// 
pub const IMM: u32  = 0x04;// Operate directly on the operand
pub const IMP: u32  = 0x05;
pub const IND: u32  = 0x06;// AKA Absolute indirect
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
    pub const ADC: u32 = 0x100;
    pub const AND: u32 = 0x200;
    pub const ASL: u32 = 0x300;
    pub const BCC: u32 = 0x400;
    pub const BCS: u32 = 0x500;
    pub const BEQ: u32 = 0x600;
    pub const BIT: u32 = 0x700;
    pub const BMI: u32 = 0x800;
    pub const BNE: u32 = 0x900;
    pub const BPL: u32 = 0xa00;
    pub const BRK: u32 = 0xb00;
    pub const BVC: u32 = 0xc00;
    pub const CLC: u32 = 0xd00;
    pub const CLD: u32 = 0xe00;
    pub const CLI: u32 = 0xf00;
    pub const CLV: u32 = 0x1000;
    pub const CMP: u32 = 0x1100;
    pub const CPX: u32 = 0x1200;
    pub const CPY: u32 = 0x1300;
    pub const DEC: u32 = 0x1400;
    pub const DEX: u32 = 0x1500;
    pub const DEY: u32 = 0x1600;
    pub const EOR: u32 = 0x1700;
    pub const INC: u32 = 0x1800;
    pub const INX: u32 = 0x1900;
    pub const INY: u32 = 0x1a00;
    pub const JMP: u32 = 0x1b00;
    pub const JSR: u32 = 0x1c00;
    pub const LDA: u32 = 0x1d00;
    pub const LDX: u32 = 0x1e00;
    pub const LDY: u32 = 0x1f00;
    pub const LSR: u32 = 0x2000;
    pub const NOP: u32 = 0x2100;
    pub const ORA: u32 = 0x2200;
    pub const PHA: u32 = 0x2300;
    pub const PHP: u32 = 0x2400;
    pub const PLA: u32 = 0x2500;
    pub const PLP: u32 = 0x2600;
    pub const ROL: u32 = 0x2700;
    pub const ROR: u32 = 0x2800;
    pub const RTI: u32 = 0x2900;
    pub const RTS: u32 = 0x2a00;
    pub const SBC: u32 = 0x2b00;
    pub const SEC: u32 = 0x2c00;
    pub const SED: u32 = 0x2d00;
    pub const SEI: u32 = 0x2e00;
    pub const STA: u32 = 0x2f00;
    pub const STX: u32 = 0x3000;
    pub const STY: u32 = 0x3100;
    pub const TAX: u32 = 0x3200;
    pub const TAY: u32 = 0x3300;
    pub const TSX: u32 = 0x3400;
    pub const TXA: u32 = 0x3500;
    pub const TXS: u32 = 0x3600;
    pub const TYA: u32 = 0x3700;
}

#[allow(dead_code)]
pub fn get_instruction(instruction:u32, mode: u32) -> u32
{
    let mut result: u32 = instruction;
    result = result << 2;
    result + mode
}

