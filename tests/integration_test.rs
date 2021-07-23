
#[path= "../src/isa.rs"]
pub mod isa;
#[path= "../src/cpu.rs"]
pub mod cpu;

const NegFlag:u8                = 0b10000000;
const OverFlowFlag:u8           = 0b01000000;
const BreakFlag:u8              = 0b00100000;
// const UnusedFlag:u8             = 0b00010000;
const DecimalFlag:u8            = 0b00001000;
const InterruptFlag:u8          = 0b00000100;
const ZeroFlag:u8               = 0b00000010;
const CarryFlag:u8              = 0b00000001;

#[test]
fn addr_mode_abs_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x00,
        x: 0x2,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0xd010] = 0xd0;

    cpu.push_instruction_16(isa::Instruction::LDA_ABS, 0xd0, 0x10);
    cpu.step();

    assert_eq!(cpu.a, 0xd0, "Value {} != {}", cpu.mem[0xd010], cpu.a);
}

#[test]
fn addr_mode_absy_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x00,
        x: 0x0,
        y: 0x2,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0xc003] = 0x5a;

    cpu.push_instruction_16(isa::Instruction::ADC_ABSY, 0xc0, 0x01);
    cpu.step();

    assert_eq!(cpu.a, 0x5a, "Value 0x5a != {}", cpu.a);
}

#[test]
fn addr_mode_absx_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x00,
        x: 0x03,
        y: 0x03,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0xf004] = 0xef;

    cpu.push_instruction_16(isa::Instruction::INC_ABSX, 0xf0, 0x01);
    cpu.step();

    assert_eq!(cpu.mem[0xf004], 0xf0, "Value 0xf0 != {}", cpu.mem[0xf004]);
}

#[test]
fn addr_mode_imm_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x0f,
        x: 0x2,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    cpu.push_instruction(isa::Instruction::LDA_IMM, 0x22);
    cpu.step();

    assert_eq!(cpu.a, 0x22, "Value {} != 0x22", cpu.a);
}

#[test]
fn addr_mode_ind_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x0f,
        x: 0x2,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0xa001] = 0xff;
    cpu.mem[0xa002] = 0x00;

    cpu.push_instruction_16(isa::Instruction::JMP_IND, 0xa0, 0x01);
    cpu.step();

    assert_eq!(cpu.pc, 0x6ff, "Value {:#4x} != 0x00ff", cpu.pc);
}

#[test]
fn addr_mode_indx_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x0f,
        x: 0x2,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0x0017] = 0xd0;
    cpu.mem[0x0018] = 0x10;

    cpu.push_instruction(isa::Instruction::STA_INDX, 0x15);
    cpu.step();

    assert_eq!(cpu.mem[0xd010], cpu.a, "Value {} != {}", cpu.mem[0xd010], cpu.a);
}

#[test]
fn addr_mode_indy_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x0f,
        x: 0x2,
        y: 0x2,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0x0017] = 0xd0;
    cpu.mem[0x0018] = 0x10;

    cpu.push_instruction(isa::Instruction::STA_INDY, 0x15);
    cpu.step();

    assert_eq!(cpu.mem[0xd010], cpu.a, "Value {} != {}", cpu.mem[0xd010], cpu.a);
}

#[test]
fn addr_mode_rel_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x0f,
        x: 0x2,
        y: 0x2,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    cpu.push_instruction(isa::Instruction::BNE_REL, 0x15);
    cpu.step();

    assert_eq!(cpu.pc, 0x615, "Value {} != 0x615", cpu.pc);
}

#[test]
fn addr_mode_zp_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x00,
        x: 0x2,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0x02] = 0xd0;

    cpu.push_instruction(isa::Instruction::LDA_ZP, 0x02);
    cpu.step();

    assert_eq!(cpu.a, 0xd0, "Value {} != {}", cpu.mem[0x02], cpu.a);
}

#[test]
fn addr_mode_zpy_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x00,
        x: 0x0,
        y: 0x2,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0x0003] = 0x5a;

    cpu.push_instruction(isa::Instruction::LDX_ZPY, 0x01);
    cpu.step();

    assert_eq!(cpu.x, 0x5a, "Value 0x5a != {}", cpu.x);
}

#[test]
fn addr_mode_zpx_addr_mode()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 0x00,
        x: 0x03,
        y: 0x03,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0xf3] = 0xef;

    cpu.push_instruction(isa::Instruction::INC_ZPX, 0xf0);
    cpu.step();

    assert_eq!(cpu.mem[0xf3], 0xf0, "Value 0xf0 != {}", cpu.mem[0xf3]);
}


#[test]
fn ins_adc()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 1,
        x: 0,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    cpu.push_instruction(isa::Instruction::ADC_IMM, 255);
    cpu.step();

    assert_eq!(cpu.a, 0);
    assert_eq!((cpu.sr & CarryFlag), CarryFlag, "Carry flag not set correctly");
    assert_eq!((cpu.sr & ZeroFlag), ZeroFlag, "Zero flag not set correctly");

    // 0 + 128
    cpu.a = 127;
    cpu.push_instruction(isa::Instruction::ADC_IMM, 1);
    cpu.step();
    
    assert_eq!(cpu.a, 128);
    assert_eq!((cpu.sr & CarryFlag), 0, "Carry Flag not set correctly");
    assert_eq!((cpu.sr & ZeroFlag), 0, "Zero flag not set correctly");
    assert_eq!((cpu.sr & NegFlag), NegFlag, "Neg flag not set correctly");
    assert_eq!((cpu.sr & OverFlowFlag), OverFlowFlag, "Overflow flag not set correctly {:#4b}", cpu.sr);
}

#[test]
fn ins_and()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 255,
        x: 0,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0x01] = 0x2;
    cpu.push_instruction(isa::Instruction::AND_ZP, 0x01);
    cpu.step();

    assert_eq!(cpu.a, 255 & cpu.mem[0x01], "{}", cpu.a);
}

#[test]
fn ins_asl()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 255,
        x: 0,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Operand
    cpu.mem[0x1234] = 0x80;
    cpu.push_instruction_16(isa::Instruction::ASL_ABS, 0x12, 0x34);
    cpu.step();

    assert_eq!(cpu.mem[0x1234], 0);
    assert_eq!((cpu.sr & CarryFlag), CarryFlag, "Carry Flag not set correctly");
    assert_eq!((cpu.sr & ZeroFlag), ZeroFlag, "Zero flag not set correctly");
    assert_eq!((cpu.sr & NegFlag), 0, "Neg flag not set correctly");

    // Test neg flag
    // Operand
    cpu.mem[0x1234] = 0x40;
    cpu.push_instruction_16(isa::Instruction::ASL_ABS, 0x12, 0x34);
    cpu.step();

    assert_eq!(cpu.mem[0x1234], 0x80);
    assert_eq!((cpu.sr & CarryFlag), 0, "Carry Flag not set correctly");
    assert_eq!((cpu.sr & ZeroFlag), 0, "Zero flag not set correctly");
    assert_eq!((cpu.sr & NegFlag), NegFlag, "Neg flag not set correctly");

}

#[test]
fn ins_bcc()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 255,
        x: 0,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Set carry flag
    cpu.sr &= !CarryFlag;

    cpu.push_instruction(isa::Instruction::BCC_REL, 0x12);
    cpu.step();

    assert_eq!(cpu.pc, 0x612, "Did not branch");
    cpu.pc = 0x0600;

    // Set carry flag
    cpu.sr |= CarryFlag;

    cpu.push_instruction(isa::Instruction::BCC_REL, 0x14);
    cpu.step();

    assert_eq!(cpu.pc, 0x601, "Branched");
}

#[test]
fn ins_bcs()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 255,
        x: 0,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Clear carry flag
    cpu.sr &= !CarryFlag;

    cpu.push_instruction(isa::Instruction::BCS_REL, 0x12);
    cpu.step();

    assert_eq!(cpu.pc, 0x601, "Branched");
    cpu.pc = 0x0600;

    // Set carry flag
    cpu.sr |= CarryFlag;

    cpu.push_instruction(isa::Instruction::BCS_REL, 0x14);
    cpu.step();

    assert_eq!(cpu.pc, 0x614, "Did not branch");
}

#[test]
fn ins_jmp()
{
    // 1 + 255
    let mut cpu = cpu::Cpu
    {
        a: 255,
        x: 0,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    // Clear carry flag
    cpu.sr &= !CarryFlag;

    cpu.push_instruction_16(isa::Instruction::JMP_ABS, 0x00, 0x14);
    cpu.step();

    assert_eq!(cpu.pc, 0x614, "Incorect jump");
}
