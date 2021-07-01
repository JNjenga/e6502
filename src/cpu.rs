
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
#[allow(non_upper_case_globals)]
impl Cpu
{
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

    fn get_abs(&mut self) -> u8
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        self.pc += 2;
        self.mem[usize::from(res)]
    }

    fn get_absx(&mut self) -> u8
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        res = res + u16::from(self.x);
        self.pc += 2;
        self.mem[usize::from(res)]
    }

    fn get_absy(&mut self) -> u8
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        res = res + u16::from(self.x);
        self.pc += 2;
        self.mem[usize::from(res)]
    }

    fn get_imm(&mut self) -> u8
    {
        let value = self.mem[usize::from(self.pc)];
        self.pc += 1;
        value
    }

    fn get_ind(&mut self) -> u16
    {
        let operand = self.mem[usize::from(self.pc)];

        let lsb = self.mem[usize::from(operand)];
        let hsb = self.mem[usize::from(operand+1)];
        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        self.pc += 1;
        res
    }

    fn get_indx(&mut self) -> u8 
    {
        let mut operand = self.mem[usize::from(self.pc)];
        operand = operand + self.x;

        let mut address : u16 = u16::from(operand);
        address = address << 8;
        address = address | u16::from(operand+1);

        let value = self.mem[usize::from(address)];
        self.pc += 1;
        value
    }

    fn get_indy(&mut self) -> u8 
    {
        let mut operand = self.mem[usize::from(self.pc)];
        operand = operand + self.y;

        let mut address : u16 = u16::from(operand);
        address = address << 8;
        address = address | u16::from(operand+1);

        let value = self.mem[usize::from(address)];
        self.pc += 1;
        value
    }

    fn get_rel(&mut self) -> u16 
    {
        let operand = self.mem[usize::from(self.pc)];
        let value = self.pc + u16::from(operand);
        self.pc += 1;
        value
    }

    fn get_zp(&mut self) -> u8
    {
        let value = self.mem[usize::from(self.pc)];
        self.pc += 1;
        value 
    }

    fn get_zpx(&mut self) -> u8
    {
        let value = self.x + self.mem[usize::from(self.pc)];
        self.pc += 1;
        value
    }

    fn get_zpy(&mut self) -> u8
    {
        let value = self.y + self.mem[usize::from(self.pc)];
        self.pc += 1;
        value
    }

    fn execute_instruction(&mut self, op_code: u32)
    {
        let instruction_code = (op_code >> 8) << 8;
        let mode :u32= (op_code << 24) >> 24;

        self.pc += 1;

        match instruction_code 
        {
            Ins::ADC =>
            {
                match mode
                {
                    Mode::IMM => 
                    {
                        self.a = self.a + self.get_imm();
                        // Set flags
                        // self.st |= OverFlowFlag;
                    },
                    Mode::ZP => 
                    {
                        self.a = self.a + self.get_zp();
                        // Set flags
                        // self.st |= OverFlowFlag;
                    },
                    Mode::ZPX => 
                    {
                        self.a = self.a + self.get_zpx();
                        // Set flags
                        // self.st |= OverFlowFlag;
                    },
                    Mode::ABS => 
                    {
                        self.a = self.a + self.get_abs();
                        // Set flags
                        // self.st |= OverFlowFlag;
                    },
                    Mode::ABSX => 
                    {
                        self.a = self.a + self.get_absx();
                        // Set flags
                        // self.st |= OverFlowFlag;
                    },
                    Mode::ABSY=> 
                    {
                        self.a = self.a + self.get_absy();
                        // Set flags
                        // self.st |= OverFlowFlag;
                    },
                    Mode::INDX => 
                    {
                        self.a = self.a + self.get_indx();
                        // Set flags
                        // self.st |= OverFlowFlag;
                    },
                    Mode::INDY => 
                    {
                        self.a = self.a + self.get_indy();
                        // Set flags
                        // self.st |= OverFlowFlag;
                    },
                    _ => panic!("Don't know this mode!")
                }
                self.pc +=1;
            },
            Ins::AND =>
            {
                match mode
                {
                    Mode::IMM => 
                    {
                        self.a &= self.get_imm();
                    },
                    Mode::ZP => 
                    {
                        self.a &= self.get_zp();
                    },
                    Mode::ZPX => 
                    {
                        self.a &= self.get_zpx();
                    },
                    Mode::ABS => 
                    {
                        self.a &= self.get_abs();
                    },
                    Mode::ABSX => 
                    {
                        self.a &= self.get_absx();
                    },
                    Mode::ABSY=> 
                    {
                        self.a &= self.get_absy();
                    },
                    Mode::INDX => 
                    {
                        self.a &= self.get_absx();
                    },
                    Mode::INDY => 
                    {
                        self.a &= self.get_absy();
                    },
                    _ => panic!("Don't know this mode!")
                }
            },
            Ins::ASL =>
            {
                match mode
                {
                    Mode::ACC => 
                    {
                        self.a = self.a << 1;
                    },
                    Mode::ZP => 
                    {
                        // TODO : Implement this
                    },
                    Mode::ZPX => 
                    {
                        // TODO  
                    },
                    Mode::ABS => 
                    {
                        // TODO  
                    },
                    Mode::ABSX => 
                    {
                        // TODO  
                    },
                    _ => panic!("Don't know this mode!")
                }
            }
            _ => {}
        }
    }

    pub fn print_regs(&self)
    {
        println!("Reg\tValue");

        println!("A\t{:02x}", self.a);
        println!("X\t{:02x}", self.x);
        println!("Y\t{:02x}", self.y);
        println!("SP\t{:02x}", self.sp);
        println!("PC\t{:04x}", self.pc);
        println!("ST\t{:04x}", self.st);
    }
}
