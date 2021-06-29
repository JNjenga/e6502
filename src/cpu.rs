
use crate::isa;

// Flag bit masks
#[allow(dead_code)]
pub struct Cpu
{
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub st: u16,
    pub mem: [u8;65536]
}

#[allow(dead_code)]
impl Default for Cpu
{
    fn default() -> Cpu
    {
        Cpu
        {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            st: 0,
            mem: [0;1<<16],
        }
    }
}

type Ins = isa::Instruction;
type Mode = isa::Mode;

#[allow(dead_code)]
impl Cpu
{
#[allow(dead_code)]
    const NegFlag:u8                = 0b10000000;
    const OverFlowFlag:u8           = 0b01000000;
    const BreakFlag:u8              = 0b00100000;
    // const UnusedFlag:u8             = 0b00010000;
    const DecimalFlag:u8            = 0b00001000;
    const IntreruptDisableFlag:u8   = 0b00000100;
    const ZeroFlag:u8               = 0b00000010;
    const CarryFlag:u8              = 0b00000001;

    // Does not reset mem 
    pub fn reset(&mut self)
    {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0;
        self.pc = 0;
        self.st = 0;
    }

    fn execute_instruction(&mut self, instruction_code: u32)
    {

        if (Ins::ADC | Mode::IMM) == instruction_code
        {
            // Process:
            //      Get operand
            //      Execute
            //      Update flags
            //      Increment pc
            
            let res = self.a + self.mem[usize::from(self.pc+1)];
            self.a = res;

            self.pc += 2;
        }
        else if (Ins::ADC | Mode::ZP) == instruction_code
        {
            let operand = self.mem[usize::from(self.pc+1)];
            let res = self.a + self.mem[usize::from(operand)];

            self.pc += 2;
        }
    }
}
