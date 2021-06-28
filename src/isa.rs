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
#[allow(dead_code)]
pub enum Mode
{
    // Accumulator
    ACC = 0x00, 
    ABS, 
    ABSX, 
    ABSY, 
    IMM, 
    IMP, 
    IND, 
    INDX, 
    INDY, 
    REL, 
    ZP, 
    ZPY, 
    ZPX, 
}

#[allow(dead_code)]
pub enum Instruction
{
    ADC = 0x00,
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

#[allow(dead_code)]
pub fn get_instruction(instruction:u32, mode: u32) -> u32
{
    let mut result: u32 = instruction;
    result = result << 2;
    result + mode
}

