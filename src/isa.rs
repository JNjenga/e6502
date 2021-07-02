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
pub const UNKNOWN: u32  = 0x0d;
}

#[allow(dead_code)]
impl Instruction
{
    pub const ADC_IMM:u8 =	 0x00;
    pub const ADC_ABS:u8 =	 0x01;
    pub const ADC_ABSX:u8 =	 0x02;
    pub const ADC_ABSY:u8 =	 0x03;
    pub const ADC_INDX:u8 =	 0x04;
    pub const ADC_INDY:u8 =	 0x05;
    pub const ADC_ZP:u8 =	 0x06;
    pub const ADC_ZPX:u8 =	 0x07;
    pub const AND_ABS:u8 =	 0x08;
    pub const AND_ABSX:u8 =	 0x09;
    pub const AND_ABSY:u8 =	 0x0a;
    pub const AND_IMM:u8 =	 0x0b;
    pub const AND_INDX:u8 =	 0x0c;
    pub const AND_INDY:u8 =	 0x0d;
    pub const AND_ZP:u8 =	 0x0e;
    pub const AND_ZPX:u8 =	 0x0f;
    pub const ASL_ACC:u8 =	 0x10;
    pub const ASL_ABS:u8 =	 0x11;
    pub const ASL_ABSX:u8 =	 0x12;
    pub const ASL_ZP:u8 =	 0x13;
    pub const ASL_ZPX:u8 =	 0x14;
    pub const BCC_REL:u8 =	 0x15;
    pub const BCS_REL:u8 =	 0x16;
    pub const BEQ_REL:u8 =	 0x17;
    pub const BIT_ABS:u8 =	 0x18;
    pub const BIT_ZP:u8 =	 0x19;
    pub const BMI_REL:u8 =	 0x1a;
    pub const BNE_REL:u8 =	 0x1b;
    pub const BPL_REL:u8 =	 0x1c;
    pub const BRK_IMP:u8 =	 0x1d;
    pub const BVC_REL:u8 =	 0x1e;
    pub const CLC_IMP:u8 =	 0x1f;
    pub const CLD_IMP:u8 =	 0x20;
    pub const CLI_IMP:u8 =	 0x21;
    pub const CLV_IMP:u8 =	 0x22;
    pub const CMP_ABS:u8 =	 0x23;
    pub const CMP_ABSX:u8 =	 0x24;
    pub const CMP_ABSY:u8 =	 0x25;
    pub const CMP_IMM:u8 =	 0x26;
    pub const CMP_INDX:u8 =	 0x27;
    pub const CMP_INDY:u8 =	 0x28;
    pub const CMP_ZP:u8 =	 0x29;
    pub const CMP_ZPX:u8 =	 0x2a;
    pub const CPX_ABS:u8 =	 0x2b;
    pub const CPX_IMM:u8 =	 0x2c;
    pub const CPX_ZP:u8 =	 0x2d;
    pub const CPY_ABS:u8 =	 0x2e;
    pub const CPY_IMM:u8 =	 0x2f;
    pub const CPY_ZP:u8 =	 0x30;
    pub const DEC_ABS:u8 =	 0x31;
    pub const DEC_ABSX:u8 =	 0x32;
    pub const DEC_ZP:u8 =	 0x33;
    pub const DEC_ZPX:u8 =	 0x34;
    pub const DEX_IMP:u8 =	 0x35;
    pub const DEY_IMP:u8 =	 0x36;
    pub const EOR_ABS:u8 =	 0x37;
    pub const EOR_ABSX:u8 =	 0x38;
    pub const EOR_ABSY:u8 =	 0x39;
    pub const EOR_IMM:u8 =	 0x3a;
    pub const EOR_INDX:u8 =	 0x3b;
    pub const EOR_INDY:u8 =	 0x3c;
    pub const EOR_ZP:u8 =	 0x3d;
    pub const EOR_ZPX:u8 =	 0x3e;
    pub const INC_ABS:u8 =	 0x3f;
    pub const INC_ABSX:u8 =	 0x40;
    pub const INC_ZP:u8 =	 0x41;
    pub const INC_ZPX:u8 =	 0x42;
    pub const INX_IMP:u8 =	 0x43;
    pub const INY_IMP:u8 =	 0x44;
    pub const JMP_ABS:u8 =	 0x45;
    pub const JMP_IND:u8 =	 0x46;
    pub const JSR_ABS:u8 =	 0x47;
    pub const LDA_ABS:u8 =	 0x48;
    pub const LDA_ABSX:u8 =	 0x49;
    pub const LDA_ABSY:u8 =	 0x4a;
    pub const LDA_IMM:u8 =	 0x4b;
    pub const LDA_INDX:u8 =	 0x4c;
    pub const LDA_INDY:u8 =	 0x4d;
    pub const LDA_ZP:u8 =	 0x4e;
    pub const LDA_ZPX:u8 =	 0x4f;
    pub const LDX_ABS:u8 =	 0x50;
    pub const LDX_ABSY:u8 =	 0x51;
    pub const LDX_IMM:u8 =	 0x52;
    pub const LDX_ZP:u8 =	 0x53;
    pub const LDX_ZPY:u8 =	 0x54;
    pub const LDY_ABS:u8 =	 0x55;
    pub const LDY_ABSX:u8 =	 0x56;
    pub const LDY_IMM:u8 =	 0x57;
    pub const LDY_ZP:u8 =	 0x58;
    pub const LDY_ZPX:u8 =	 0x59;
    pub const LSR_ACC:u8 =	 0x5a;
    pub const LSR_ABS:u8 =	 0x5b;
    pub const LSR_ABSX:u8 =	 0x5c;
    pub const LSR_ZP:u8 =	 0x5d;
    pub const LSR_ZPX:u8 =	 0x5e;
    pub const NOP_IMP:u8 =	 0x5f;
    pub const ORA_ABS:u8 =	 0x60;
    pub const ORA_ABSX:u8 =	 0x61;
    pub const ORA_ABSY:u8 =	 0x62;
    pub const ORA_IMM:u8 =	 0x63;
    pub const ORA_INDX:u8 =	 0x64;
    pub const ORA_INDY:u8 =	 0x65;
    pub const ORA_ZP:u8 =	 0x66;
    pub const ORA_ZPX:u8 =	 0x67;
    pub const PHA_IMP:u8 =	 0x68;
    pub const PHP_IMP:u8 =	 0x69;
    pub const PLA_IMP:u8 =	 0x6a;
    pub const PLP_IMP:u8 =	 0x6b;
    pub const ROL_ACC:u8 =	 0x6c;
    pub const ROL_ABS:u8 =	 0x6d;
    pub const ROL_ABSX:u8 =	 0x6e;
    pub const ROL_ZP:u8 =	 0x6f;
    pub const ROL_ZPX:u8 =	 0x70;
    pub const ROR_ACC:u8 =	 0x71;
    pub const ROR_ABS:u8 =	 0x72;
    pub const ROR_ABSX:u8 =	 0x73;
    pub const ROR_ZP:u8 =	 0x74;
    pub const ROR_ZPX:u8 =	 0x75;
    pub const RTI_IMP:u8 =	 0x76;
    pub const RTS_IMP:u8 =	 0x77;
    pub const SBC_ABS:u8 =	 0x78;
    pub const SBC_ABSX:u8 =	 0x79;
    pub const SBC_ABSY:u8 =	 0x7a;
    pub const SBC_IMM:u8 =	 0x7b;
    pub const SBC_INDX:u8 =	 0x7c;
    pub const SBC_INDY:u8 =	 0x7d;
    pub const SBC_ZP:u8 =	 0x7e;
    pub const SBC_ZPX:u8 =	 0x7f;
    pub const SEC_IMP:u8 =	 0x80;
    pub const SED_IMP:u8 =	 0x81;
    pub const SEI_IMP:u8 =	 0x82;
    pub const STA_ABS:u8 =	 0x83;
    pub const STA_ABSX:u8 =	 0x84;
    pub const STA_ABSY:u8 =	 0x85;
    pub const STA_INDX:u8 =	 0x86;
    pub const STA_INDY:u8 =	 0x87;
    pub const STA_ZP:u8 =	 0x88;
    pub const STA_ZPX:u8 =	 0x89;
    pub const STX_ABS:u8 =	 0x8a;
    pub const STX_ZP:u8 =	 0x8b;
    pub const STX_ZPY:u8 =	 0x8c;
    pub const STY_ABS:u8 =	 0x8d;
    pub const STY_ZP:u8 =	 0x8e;
    pub const STY_ZPX:u8 =	 0x8f;
    pub const TAX_IMP:u8 =	 0x90;
    pub const TAY_IMP:u8 =	 0x91;
    pub const TSX_IMP:u8 =	 0x92;
    pub const TXA_IMP:u8 =	 0x93;
    pub const TXS_IMP:u8 =	 0x94;
    pub const TYA_IMP:u8 =	 0x95;
}
