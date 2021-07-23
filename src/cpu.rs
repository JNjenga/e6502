
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
    pub sr: u8,
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
            sr: 0,
            mem: [0;1<<16],
        }
    }
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
impl Cpu
{
    const NegFlag:u8                = 0b10000000;
    const OverFlowFlag:u8           = 0b01000000;
    const BreakFlag:u8              = 0b00100000;
    // const UnusedFlag:u8             = 0b00010000;
    const DecimalFlag:u8            = 0b00001000;
    const InterruptFlag:u8          = 0b00000100;
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
        self.sr = 0;
    }

    fn get_abs_address(&mut self) -> u16
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        self.pc += 2;
        res
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

    fn get_abs_ref(&mut self) -> &mut u8
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        self.pc += 2;
        &mut self.mem[usize::from(res)]
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

    fn get_absx_ref(&mut self) -> &mut u8
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        res = res + u16::from(self.x);
        self.pc += 2;
        &mut self.mem[usize::from(res)]
    }

    fn get_absx_address(&mut self) -> u16
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        res = res + u16::from(self.x);
        self.pc += 2;
        res
    }

    fn get_absy(&mut self) -> u8
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        res = res + u16::from(self.y);
        self.pc += 2;
        self.mem[usize::from(res)]
    }

    fn get_absy_address(&mut self) -> u16
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        res = res + u16::from(self.y);
        self.pc += 2;
        res
    }

    fn get_absy_ref(&mut self) -> &mut u8
    {
        let hsb = self.mem[usize::from(self.pc)];
        let lsb = self.mem[usize::from(self.pc+1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        res = res + u16::from(self.y);
        self.pc += 2;
        &mut self.mem[usize::from(res)]
    }

    fn get_imm(&mut self) -> u8
    {
        let value = self.mem[usize::from(self.pc)];
        self.pc += 1;
        value
    }

    fn get_ind(&mut self) -> u16
    {
        let address_hsb = self.mem[usize::from(self.pc)];
        let address_lsb = self.mem[usize::from(self.pc+1)];

        let mut address: u16 = address_hsb as u16;
        address = address << 8;
        address |= address_lsb as u16;

        let lsb = self.mem[usize::from(address)];
        let hsb = self.mem[usize::from(address + 1)];

        let mut res: u16 = u16::from(hsb);
        res = res << 8;
        res = res | u16::from(lsb);
        self.pc += 2;
        res
    }

    fn get_indx(&mut self) -> u8 
    {
        let mut operand = self.mem[usize::from(self.pc)];
        operand = operand + self.x;

        let mut address : u16 = self.mem[operand as usize] as u16;
        address = address << 8;
        address = address | self.mem[(operand + 1) as usize] as u16;

        self.pc += 1;
        self.mem[usize::from(address)]
    }

    fn get_indx_ref(&mut self) -> &mut u8 
    {
        let mut operand = self.mem[usize::from(self.pc)];
        operand = operand + self.x;

        let mut address : u16 = self.mem[operand as usize] as u16;
        address = address << 8;
        address = address | self.mem[(operand + 1) as usize] as u16;

        self.pc += 1;
        &mut self.mem[usize::from(address)]
    }

    fn get_indx_address(&mut self) -> u16
    {
        let mut operand = self.mem[usize::from(self.pc)];
        operand = operand + self.x;

        let mut address : u16 = self.mem[operand as usize] as u16;
        address = address << 8;
        address = address | self.mem[(operand + 1) as usize] as u16;

        self.pc += 1;
        address
    }

    fn get_indy(&mut self) -> u8 
    {
        let mut operand = self.mem[usize::from(self.pc)];
        operand = operand + self.y;

        let mut address : u16 = self.mem[operand as usize] as u16;
        address = address << 8;
        address = address | self.mem[(operand + 1) as usize] as u16;

        self.pc += 1;
        self.mem[usize::from(address)]
    }

    fn get_indy_ref(&mut self) -> &mut u8 
    {
        let mut operand = self.mem[usize::from(self.pc)];
        operand = operand + self.y;

        let mut address : u16 = self.mem[operand as usize] as u16;
        address = address << 8;
        address = address | self.mem[(operand + 1) as usize] as u16;

        self.pc += 1;
        &mut self.mem[usize::from(address)]
    }

    fn get_indy_address(&mut self) -> u16
    {
        let mut operand = self.mem[usize::from(self.pc)];
        operand = operand + self.y;

        let mut address : u16 = self.mem[operand as usize] as u16;
        address = address << 8;
        address = address | self.mem[(operand + 1) as usize] as u16;

        self.pc += 1;
        address
    }

    fn get_rel(&mut self) -> u16
    {
        let operand = self.mem[usize::from(self.pc)];
        // TODO(James) : Make 0x600 not static value
        let value = 0x600 + u16::from(operand);
        self.pc += 1;
        value
    }

    fn get_zp(&mut self) -> u8
    {
        let address = self.mem[usize::from(self.pc)];
        self.pc += 1;
        self.mem[usize::from(address)]
    }

    fn get_zp_ref(&mut self) -> &mut u8
    {
        let address = self.mem[usize::from(self.pc)];
        self.pc += 1;
        &mut self.mem[usize::from(address)]
    }

    fn get_zp_address(&mut self) -> u16
    {
        let address = self.mem[usize::from(self.pc)];
        self.pc += 1;
        u16::from(address)
    }

    fn get_zpx(&mut self) -> u8
    {
        let value = self.x + self.mem[usize::from(self.pc)];
        self.pc += 1;
        self.mem[usize::from(value)]
    }

    fn get_zpx_ref(&mut self) -> &mut u8
    {
        let value = self.x + self.mem[usize::from(self.pc)];
        self.pc += 1;
        &mut self.mem[usize::from(value)]
    }

    fn get_zpx_address(&mut self) -> u16
    {
        let address = self.x + self.mem[usize::from(self.pc)];
        self.pc += 1;
        u16::from(address)
    }

    fn get_zpy(&mut self) -> u8
    {
        let value = self.y + self.mem[usize::from(self.pc)];
        self.pc += 1;
        self.mem[usize::from(value)]
    }

    fn get_zpy_ref(&mut self) -> &mut u8
    {
        let value = self.y + self.mem[usize::from(self.pc)];
        self.pc += 1;
        &mut self.mem[usize::from(value)]
    }

    fn get_zpy_address(&mut self) -> u16
    {
        let address = self.y + self.mem[usize::from(self.pc)];
        self.pc += 1;
        u16::from(address)
    }

    fn stack_push(&mut self, value : u8)
    {
        self.mem[usize::from(0x100 + u16::from(self.sp))] = value;
        self.sp = self.sp.wrapping_sub(1);
    }

    fn stack_push_16(&mut self, value : u16)
    {
        self.mem[usize::from(0x100 + u16::from(self.sp))] = (value >> 8 ) as u8;
        self.sp = self.sp.wrapping_sub(1);
        self.mem[usize::from(0x100 + u16::from(self.sp))]  = ((value << 8) >> 8) as u8;
        self.sp = self.sp.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8
    {
        self.sp = self.sp.wrapping_add(1);
        let value = self.mem[usize::from(0x100 + u16::from(self.sp))];
        self.mem[usize::from(0x100 + u16::from(self.sp))] = 0;

        value
    }

    fn stack_pop_16(&mut self) -> u16
    {
        self.sp = self.sp.wrapping_add(1);
        let lsb = self.mem[usize::from(0x100 + u16::from(self.sp))];
        self.mem[usize::from(0x100 + u16::from(self.sp))] = 0;
        self.sp = self.sp.wrapping_add(1);
        let hsb = self.mem[usize::from(0x100 + u16::from(self.sp))];
        self.mem[usize::from(0x100 + u16::from(self.sp))] = 0;

        let mut res: u16 = u16::from(hsb);
        res = (res << 8) | u16::from(lsb);

        res
    }

    #[inline(always)]
    fn set_zerof(&mut self, value: u8, clear : bool)
    {
        if value == 0
        {
            self.sr |= Cpu::ZeroFlag;
        }
        else if clear
        {
            self.sr &= !Cpu::ZeroFlag;
        }
    }

    #[inline(always)]
    fn set_negf(&mut self, value: u8, clear : bool)
    {
        if value >> 7 == 1
        {
            self.sr |= Cpu::NegFlag;
        }
        else if clear
        {
            self.sr &= !Cpu::NegFlag;
        }
    }

    fn set_vflag(&mut self, prev_value:u8, value:u8)
    {
        if prev_value &  0b01000000 == 0b01000000
            && value & 0b10000000 == 0b10000000
            {
                self.sr |= Cpu::OverFlowFlag;
            }
        else
        {
            self.sr &= !Cpu::OverFlowFlag;
        }
    }

    pub fn push_instruction(&mut self, instruction : u8, operand: u8)
    {
        self.mem[usize::from(self.pc)] = instruction;
        self.mem[usize::from(self.pc + 1)] = operand;
    }

    pub fn push_instruction_16(&mut self, instruction : u8, operand: u8, operand2 : u8)
    {
        self.mem[usize::from(self.pc)] = instruction;
        self.mem[usize::from(self.pc + 1)] = operand;
        self.mem[usize::from(self.pc + 2)] = operand2;
    }

    fn execute_instruction(&mut self, opcode: u8)
    {
        match opcode
        {
            isa::Instruction::ADC_IMM =>
            {
                let operand = self.get_imm();

                self.adc(operand);
            },
            isa::Instruction::ADC_ABS =>
            {
                let operand = self.get_abs();

                self.adc(operand);
            },
            isa::Instruction::ADC_ABSX =>
            {
                let operand = self.get_absx();
                self.adc(operand);
            },
            isa::Instruction::ADC_ABSY =>
            {
                let operand = self.get_absy();
                self.adc(operand);
            },
            isa::Instruction::ADC_INDX =>
            {
                let operand = self.get_indx();
                self.adc(operand);
            },
            isa::Instruction::ADC_INDY =>
            {
                let operand = self.get_indy();
                self.adc(operand);
            },
            isa::Instruction::ADC_ZP =>
            {
                let operand = self.get_zp();
                self.adc(operand);
            },
            isa::Instruction::ADC_ZPX =>
            {
                let operand = self.get_zpx();
                self.adc(operand);
            },
            isa::Instruction::AND_ABS =>
            {
                let operand = self.get_abs();
                self.a &= operand;
            },
            isa::Instruction::AND_ABSX =>
            {
                let operand = self.get_absx();
                self.a &= operand;
            },
            isa::Instruction::AND_ABSY =>
            {
                let operand = self.get_absy();
                self.a &= operand;
            },
            isa::Instruction::AND_IMM =>
            {
                let operand = self.get_imm();
                self.a &= operand;
            },
            isa::Instruction::AND_INDX =>
            {
                let operand = self.get_indx();
                self.a &= operand;
            },
            isa::Instruction::AND_INDY =>
            {
                let operand = self.get_indy();
                self.a &= operand;
            },
            isa::Instruction::AND_ZP =>
            {
                let operand = self.get_zp();
                self.a &= operand;
            },
            isa::Instruction::AND_ZPX =>
            {
                let operand = self.get_zpx();
                self.a &= operand;
            },
            isa::Instruction::ASL_ACC =>
            {
                let prev_a = self.a;
                self.a = self.a << 1;

                if (prev_a >> 7) == 0 || prev_a == 0
                {
                    self.sr &= !Cpu::CarryFlag;
                }
                else
                {
                    self.sr |= Cpu::CarryFlag;
                }

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::ASL_ABS =>
            {
                let value = self.get_abs_address();

                self.mem[value as usize] = self.asl(self.mem[value as usize]);

            },
            isa::Instruction::ASL_ABSX =>
            {
                let value = self.get_absx_address() as usize;

                self.mem[value] = self.asl(self.mem[value]);
            },
            isa::Instruction::ASL_ZP =>
            {
                let value = self.get_zp_address() as usize;

                self.mem[value] = self.asl(self.mem[value]);
            },
            isa::Instruction::ASL_ZPX =>
            {

                let value = self.get_zpx_address() as usize;

                self.mem[value] = self.asl(self.mem[value]);
           },
            isa::Instruction::BCC_REL =>
            {
                let carry_flag = self.sr & Cpu::CarryFlag;
                if carry_flag == 0
                {
                    self.pc = self.get_rel();
                }
            },
            isa::Instruction::BCS_REL =>
            {
                let carry_flag = self.sr & Cpu::CarryFlag;
                if carry_flag == Cpu::CarryFlag
                {
                    self.pc = self.get_rel();
                }
            },
            isa::Instruction::BEQ_REL =>
            {
                let zero_flag = self.sr & Cpu::ZeroFlag;
                if zero_flag == Cpu::ZeroFlag
                {
                    self.pc = self.get_rel();
                }
            },
            isa::Instruction::BIT_ABS =>
            {
                let operand = self.get_abs();
                let neg_flag = (operand << 1) >> 7;
                let overflow_flag = (operand << 2) >> 7;
                let zero_flag = self.a & operand;

                if neg_flag == 0
                {
                    self.sr &= !Cpu::NegFlag;
                }
                else 
                {
                    self.sr |= Cpu::NegFlag;
                }

                if overflow_flag == 0
                {
                    self.sr &= !Cpu::OverFlowFlag;
                }
                else 
                {
                    self.sr |= Cpu::OverFlowFlag;
                }

                if zero_flag == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
            },
            isa::Instruction::BIT_ZP =>
            {
                let operand = self.get_zp();
                let neg_flag = (operand << 1) >> 7;
                let overflow_flag = (operand << 2) >> 7;
                let zero_flag = self.a & operand;

                if neg_flag == 0
                {
                    self.sr &= !Cpu::NegFlag;
                }
                else 
                {
                    self.sr |= Cpu::NegFlag;
                }

                if overflow_flag == 0
                {
                    self.sr &= !Cpu::OverFlowFlag;
                }
                else 
                {
                    self.sr |= Cpu::OverFlowFlag;
                }

                if zero_flag == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
            },
            isa::Instruction::BMI_REL =>
            {
                let neg_flag = self.sr & Cpu::NegFlag;
                if neg_flag == Cpu::NegFlag 
                {
                    let operand = self.get_rel();
                    self.pc = operand;
                }
            },
            isa::Instruction::BNE_REL =>
            {
                let zero_flag = self.sr & Cpu::ZeroFlag;
                if zero_flag == 0
                {
                    let operand = self.get_rel();
                    self.pc = operand;
                }
            },
            isa::Instruction::BPL_REL =>
            {
                let neg_flag = self.sr & Cpu::NegFlag;
                if neg_flag == 0
                {
                    let operand = self.get_rel();
                    self.pc = operand;
                }
            },
            isa::Instruction::BRK_IMP =>
            {
                self.sr |= Cpu::BreakFlag;
                self.pc += 1;

            },
            isa::Instruction::BVC_REL =>
            {
                let overflow_flag = self.sr & Cpu::OverFlowFlag;
                if overflow_flag == 0
                {
                    let operand = self.get_rel();
                    self.pc = operand;
                }
            },
            isa::Instruction::BVS_REL =>
            {
                let overflow_flag = self.sr & Cpu::OverFlowFlag;
                if overflow_flag == Cpu::OverFlowFlag 
                {
                    let operand = self.get_rel();
                    self.pc = operand;
                }
            },
            isa::Instruction::CLC_IMP =>
            {
                self.sr &= !Cpu::CarryFlag;
            },
            isa::Instruction::CLD_IMP =>
            {
                self.sr &= !Cpu::DecimalFlag;
            },
            isa::Instruction::CLI_IMP =>
            {
                self.sr &= !Cpu::InterruptFlag;
            },
            isa::Instruction::CLV_IMP =>
            {
                self.sr &= !Cpu::OverFlowFlag;
            },

            isa::Instruction::CMP_ABS =>
            {
                let operand = self.get_abs();

                if self.a < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.a == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.a >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CMP_ABSX =>
            {
                let operand = self.get_absx();

                if self.a < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.a == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.a >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CMP_ABSY =>
            {
                let operand = self.get_absy();

                if self.a < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.a == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.a >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CMP_IMM =>
            {
                let operand = self.get_imm();

                if self.a < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.a == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.a >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CMP_INDX =>
            {
                let operand = self.get_indx();

                if self.a < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.a == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.a >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CMP_INDY =>
            {
                let operand = self.get_indy();

                if self.a < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.a == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.a >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CMP_ZP =>
            {
                let operand = self.get_zp();

                if self.a < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.a == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.a >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CMP_ZPX =>
            {
                let operand = self.get_zpx();

                if self.a < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.a == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.a >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CPX_ABS =>
            {
                let operand = self.get_abs();

                if self.x < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.x == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                }
                else if self.x >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CPX_IMM =>
            {
                let operand = self.get_imm();

                if self.x < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.x == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr |= Cpu::CarryFlag;
                }
                else if self.x >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CPX_ZP =>
            {
                let operand = self.get_zp();

                if self.x < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.x == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr |= Cpu::CarryFlag;
                }
                else if self.x >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CPY_ABS =>
            {
                let operand = self.get_abs();

                if self.y < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.y == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr |= Cpu::CarryFlag;
                }
                else if self.y >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CPY_IMM =>
            {
                let operand = self.get_imm();

                if self.y < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.y == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr |= Cpu::CarryFlag;
                }
                else if self.y >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::CPY_ZP =>
            {
                let operand = self.get_zp();

                if self.y < operand
                {
                    self.sr |= Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                    self.sr &= !Cpu::CarryFlag;
                }
                else if self.y == operand
                {
                    self.sr |= Cpu::ZeroFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr |= Cpu::CarryFlag;
                }
                else if self.y >= operand
                {
                    self.sr |= Cpu::CarryFlag;
                    self.sr &= !Cpu::NegFlag;
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::DEC_ABS =>
            {
                let operand = self.get_abs_ref();

                *operand = operand.wrapping_sub(1);
                let result = *operand;

                if result >> 7 == 1
                {
                    self.sr |= Cpu::OverFlowFlag;
                }

                if result == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
            },
            isa::Instruction::DEC_ABSX =>
            {
                let operand = self.get_absx_ref();

                *operand = operand.wrapping_sub(1);
                let result = *operand;

                if result >> 7 == 1
                {
                    self.sr |= Cpu::OverFlowFlag;
                }

                if result == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
            },
            isa::Instruction::DEC_ZP =>
            {
                let operand = self.get_zp_ref();

                *operand = operand.wrapping_sub(1);

                let result = *operand;

                if result >> 7 == 1
                {
                    self.sr |= Cpu::OverFlowFlag;
                }

                if result == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
            },
            isa::Instruction::DEC_ZPX =>
            {
                let operand = self.get_zpx_ref();

                *operand = operand.wrapping_sub(1);

                let result = *operand;

                if result >> 7 == 1
                {
                    self.sr |= Cpu::OverFlowFlag;
                }

                if result == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
            },
            isa::Instruction::DEX_IMP =>
            {
                self.x = self.x.wrapping_sub(1);

                if self.x >> 7 == 1
                {
                    self.sr |= Cpu::OverFlowFlag;
                }

                if self.x == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
            },
            isa::Instruction::DEY_IMP =>
            {
                self.y = self.x.wrapping_sub(1);

                if self.y >> 7 == 1
                {
                    self.sr |= Cpu::OverFlowFlag;
                }

                if self.y == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
            },
            isa::Instruction::EOR_ABS =>
            {
                let operand = self.get_abs();

                self.a ^= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::EOR_ABSX =>
            {
                let operand = self.get_absx();

                self.x ^= operand;

                if self.x >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.x == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::EOR_ABSY =>
            {
                let operand = self.get_absy();

                self.y ^= operand;

                if self.y >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.y == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::EOR_IMM =>
            {
                let operand = self.get_imm();

                self.a ^= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::EOR_INDX =>
            {
                let operand = self.get_indx();

                self.x ^= operand;

                if self.x >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.x == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::EOR_INDY =>
            {
                let operand = self.get_indy();

                self.y ^= operand;

                if self.y >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.y == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }

            },
            isa::Instruction::EOR_ZP =>
            {
                let operand = self.get_zp();

                self.a ^= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::EOR_ZPX =>
            {
                let operand = self.get_zpx();

                self.x ^= operand;

                if self.x >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.x == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::INC_ABS =>
            {
                let operand = self.get_abs_ref();

                *operand = operand.wrapping_add(1);

                let result = *operand;

                self.set_zerof(result, false);
                self.set_negf(result, true);
            },
            isa::Instruction::INC_ABSX =>
            {
                let operand = self.get_absx_ref();

                *operand = operand.wrapping_add(1);
                let result = *operand;


                self.set_zerof(result, false);
                self.set_negf(result, true);
            },
            isa::Instruction::INC_ZP =>
            {
                let operand = self.get_zp_ref();

                *operand = operand.wrapping_add(1);

                let result = *operand;


                self.set_zerof(result, false);
                self.set_negf(result, true);
            },
            isa::Instruction::INC_ZPX =>
            {
                let operand = self.get_zpx_ref();

                *operand = operand.wrapping_add(1);

                let result = *operand;

                self.set_zerof(result, false);
                self.set_negf(result, true);

            },
            isa::Instruction::INX_IMP =>
            {
                self.x = self.x.wrapping_add(1);

                self.set_zerof(self.x, false);
                self.set_negf(self.x, true);

            },
            isa::Instruction::INY_IMP =>
            {
                self.y = self.y.wrapping_add(1);

                self.set_zerof(self.y, false);
                self.set_negf(self.y, true);

            },
            isa::Instruction::JMP_ABS =>
            {
                let operand = self.get_abs_address();
                self.pc = 0x600 + operand;
            },
            isa::Instruction::JMP_IND =>
            {
                let operand = self.get_ind();
                self.pc = 0x600 + operand;
            },
            isa::Instruction::JSR_ABS =>
            {
                let operand = self.get_abs_address();

                println!("Before push");
                println!("{:#4x}", self.mem[0x1ff]);
                println!("{:#4x}", self.mem[0x1fe]);
                println!("{:#4x}", self.mem[0x1fd]);
                self.stack_push_16(self.pc);
                println!("After push");
                println!("{:#4x}", self.mem[0x1ff]);
                println!("{:#4x}", self.mem[0x1fe]);
                println!("{:#4x}", self.mem[0x1fd]);
                self.pc = 0x600 + operand;
            },
            isa::Instruction::LDA_ABS =>
            {
                let operand = self.get_abs();
                
                self.a = operand;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::LDA_ABSX =>
            {
                let operand = self.get_absx();

                self.a = operand;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::LDA_ABSY =>
            {
                let operand = self.get_absy();

                self.a = operand;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::LDA_IMM =>
            {
                let operand = self.get_imm();

                self.a = operand;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::LDA_INDX =>
            {
                let operand = self.get_indx();

                self.a = operand;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::LDA_INDY =>
            {
                let operand = self.get_indy();

                self.a = operand;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::LDA_ZP =>
            {
                let operand = self.get_zp();
                self.a = operand;
                assert_eq!(self.a, operand);

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::LDA_ZPX =>
            {
                let operand = self.get_zpx();

                self.a = operand;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::LDX_ABS =>
            {
                let operand = self.get_abs();

                self.x = operand;

                self.set_zerof(self.x, true);
                self.set_negf(self.x, true);
            },
            isa::Instruction::LDX_ABSY =>
            {
                let operand = self.get_absy();

                self.x = operand;

                self.set_zerof(self.x, true);
                self.set_negf(self.x, true);
            },
            isa::Instruction::LDX_IMM =>
            {
                let operand = self.get_imm();

                self.x = operand;

                self.set_zerof(self.x, true);
                self.set_negf(self.x, true);
            },
            isa::Instruction::LDX_ZP =>
            {
                let operand = self.get_zp();

                self.x = operand;

                self.set_zerof(self.x, true);
                self.set_negf(self.x, true);
            },
            isa::Instruction::LDX_ZPY =>
            {
                let operand = self.get_zpy();

                self.x = operand;

                self.set_zerof(self.x, true);
                self.set_negf(self.x, true);
            },
            isa::Instruction::LDY_ABS =>
            {
                let operand = self.get_abs();

                self.y = operand;

                self.set_zerof(self.y, true);
                self.set_negf(self.y, true);
            },
            isa::Instruction::LDY_ABSX =>
            {
                let operand = self.get_absx();

                self.y = operand;

                self.set_zerof(self.y, true);
                self.set_negf(self.y, true);
            },
            isa::Instruction::LDY_IMM =>
            {
                let operand = self.get_imm();

                self.y = operand;

                self.set_zerof(self.y, true);
                self.set_negf(self.y, true);
            },
            isa::Instruction::LDY_ZP =>
            {
                let operand = self.get_zp();

                self.y = operand;

                self.set_zerof(self.y, true);
                self.set_negf(self.y, true);
            },
            isa::Instruction::LDY_ZPX =>
            {
                let operand = self.get_zpx();

                self.y = operand;

                self.set_zerof(self.y, true);
                self.set_negf(self.y, true);
            },
            isa::Instruction::LSR_ACC =>
            {
                let prev_a = self.a;
                self.a = self.a >> 1;

                self.sr &= !Cpu::NegFlag;

                if (prev_a << 7 ) >> 7 == 0
                {
                    self.sr &= !Cpu::CarryFlag;
                }
                else
                {
                    self.sr |= Cpu::CarryFlag;
                }
                self.set_zerof(self.a, true);
            },
            isa::Instruction::LSR_ABS =>
            {
                let operand = self.get_abs_ref();

                let lsb = *operand >> 7;
                *operand >>= 1;
                let res = *operand;

                if lsb == 0
                {
                    self.sr &= !Cpu::CarryFlag;
                }
                else
                {
                    self.sr |= Cpu::CarryFlag;
                }

                if res == 0
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::LSR_ABSX =>
            {
                let operand = self.get_absx_ref();

                let lsb = *operand >> 7;
                *operand >>= 1;
                let res = *operand;

                if lsb == 0
                {
                    self.sr &= !Cpu::CarryFlag;
                }
                else
                {
                    self.sr |= Cpu::CarryFlag;
                }

                if res == 0
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::LSR_ZP =>
            {
                let operand = self.get_zp_ref();

                let lsb = *operand >> 7;
                *operand >>= 1;
                let res = *operand;

                if lsb == 0
                {
                    self.sr &= !Cpu::CarryFlag;
                }
                else
                {
                    self.sr |= Cpu::CarryFlag;
                }

                if res == 0
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::LSR_ZPX =>
            {
                let operand = self.get_zpx_ref();

                let lsb = *operand >> 7;
                *operand >>= 1;
                let res = *operand;

                if lsb == 0
                {
                    self.sr &= !Cpu::CarryFlag;
                }
                else
                {
                    self.sr |= Cpu::CarryFlag;
                }

                if res == 0
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::NOP_IMP =>
            {
                self.pc += 1;
            },
            isa::Instruction::ORA_ABS =>
            {
                let operand = self.get_abs();

                self.a |= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::ORA_ABSX =>
            {
                let operand = self.get_absx();

                self.a |= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::ORA_ABSY =>
            {
                let operand = self.get_absy();

                self.a |= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::ORA_IMM =>
            {
                let operand = self.get_imm();

                self.a |= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::ORA_INDX =>
            {
                let operand = self.get_indx();

                self.a |= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::ORA_INDY =>
            {
                let operand = self.get_indy();

                self.a |= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::ORA_ZP =>
            {
                let operand = self.get_zp();

                self.a |= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::ORA_ZPX =>
            {
                let operand = self.get_zpx();

                self.a |= operand;

                if self.a >> 7 == 1
                {
                    self.sr |= Cpu::NegFlag;
                }

                if self.a == 0
                {
                    self.sr |= Cpu::ZeroFlag;
                }
                else
                {
                    self.sr &= !Cpu::ZeroFlag;
                }
            },
            isa::Instruction::PHA_IMP =>
            {
                self.stack_push(self.a);
            },
            isa::Instruction::PHP_IMP =>
            {
                self.stack_push(self.sr);
            },
            isa::Instruction::PLA_IMP =>
            {
                self.a = self.stack_pop();

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::PLP_IMP =>
            {
                self.sp = self.stack_pop();
            },
            isa::Instruction::ROL_ACC =>
            {
                let hsb = self.a >> 7;
                let prev_carry = self.sr & Cpu::CarryFlag;

                self.a <<= 1;
                self.a |= prev_carry;

                self.sr &= hsb;
                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::ROL_ABS =>
            {
                let prev_carry = self.sr & Cpu::CarryFlag;

                let operand = self.get_abs_ref();
                let hsb = *operand >> 7;

                *operand <<= 1;
                *operand |= prev_carry;

                let res = *operand;
                self.sr &= hsb;
                self.set_zerof(res, true);
                self.set_negf(res, true);
            },
            isa::Instruction::ROL_ABSX =>
            {
                let prev_carry = self.sr & Cpu::CarryFlag;

                let operand = self.get_absx_ref();
                let hsb = *operand >> 7;

                *operand <<= 1;
                *operand |= prev_carry;

                let res = *operand;
                self.sr &= hsb;
                self.set_zerof(res, true);
                self.set_negf(res, true);
            },
            isa::Instruction::ROL_ZP =>
            {
                let prev_carry = self.sr & Cpu::CarryFlag;

                let operand = self.get_zp_ref();
                let hsb = *operand >> 7;

                *operand <<= 1;
                *operand |= prev_carry;

                let res = *operand;
                self.sr &= hsb;
                self.set_zerof(res, true);
                self.set_negf(res, true);
            },
            isa::Instruction::ROL_ZPX =>
            {
                let prev_carry = self.sr & Cpu::CarryFlag;

                let operand = self.get_zpx_ref();
                let hsb = *operand >> 7;

                *operand <<= 1;
                *operand |= prev_carry;

                let res = *operand;
                self.sr &= hsb;
                self.set_zerof(res, true);
                self.set_negf(res, true);
            },
            isa::Instruction::ROR_ACC =>
            {
                let lsb = (self.a << 7) >> 7;
                let prev_carry = self.sr & Cpu::CarryFlag;

                self.a >>= 1;
                self.a |= prev_carry << 7;

                self.sr &= lsb;
                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::ROR_ABS =>
            {
                let prev_carry = self.sr & Cpu::CarryFlag;

                let operand = self.get_abs_ref();
                let lsb = (*operand << 7) >> 7;

                *operand >>= 1;
                *operand |= prev_carry << 7;

                let res = *operand;
                self.sr &= lsb;
                self.set_zerof(res, true);
                self.set_negf(res, true);
            },
            isa::Instruction::ROR_ABSX =>
            {
                let prev_carry = self.sr & Cpu::CarryFlag;

                let operand = self.get_absx_ref();
                let lsb = (*operand << 7) >> 7;

                *operand >>= 1;
                *operand |= prev_carry << 7;

                let res = *operand;
                self.sr &= lsb;
                self.set_zerof(res, true);
                self.set_negf(res, true);
            },
            isa::Instruction::ROR_ZP =>
            {
                let prev_carry = self.sr & Cpu::CarryFlag;

                let operand = self.get_zp_ref();
                let lsb = (*operand << 7) >> 7;

                *operand >>= 1;
                *operand |= prev_carry << 7;

                let res = *operand;
                self.sr &= lsb;
                self.set_zerof(res, true);
                self.set_negf(res, true);
            },
            isa::Instruction::ROR_ZPX =>
            {
                let prev_carry = self.sr & Cpu::CarryFlag;

                let operand = self.get_zpx_ref();
                let lsb = (*operand << 7) >> 7;

                *operand >>= 1;
                *operand |= prev_carry << 7;

                let res = *operand;
                self.sr &= lsb;
                self.set_zerof(res, true);
                self.set_negf(res, true);
            },
            isa::Instruction::RTI_IMP =>
            {
                self.sp = self.stack_pop();
                self.pc = self.stack_pop_16();
            },
            isa::Instruction::RTS_IMP =>
            {
                self.pc = self.stack_pop_16();
                // self.pc -= 1;
            },
            isa::Instruction::SBC_ABS =>
            {
                let operand = self.get_abs();

                // Previous a
                let prev_a = self.a & 0b10000000;
                let option = self.a.checked_sub(operand);

                match option
                {
                    None => 
                    {
                        self.a = 0;
                        self.sr |= Cpu::CarryFlag;
                        self.set_zerof(self.a, true);
                    },
                    Some(value) =>
                    {
                        self.a = value;
                        self.set_zerof(self.a, true);
                        self.set_negf(self.a, false);

                        // Set overflow flag
                        self.set_vflag(prev_a, self.a);
                    }
                }
            },
            isa::Instruction::SBC_ABSX =>
            {
                let operand = self.get_absx();

                // Previous a
                let prev_a = self.a & 0b10000000;
                let option = self.a.checked_sub(operand);

                match option
                {
                    None => 
                    {
                        self.a = 0;
                        self.sr |= Cpu::CarryFlag;
                        self.set_zerof(self.a, true);
                    },
                    Some(value) =>
                    {
                        self.a = value;
                        self.set_zerof(self.a, true);
                        self.set_negf(self.a, false);

                        // Set overflow flag
                        self.set_vflag(prev_a, self.a);
                    }
                }
            },
            isa::Instruction::SBC_ABSY =>
            {
                let operand = self.get_absy();

                // Previous a
                let prev_a = self.a & 0b10000000;
                let option = self.a.checked_sub(operand);

                match option
                {
                    None => 
                    {
                        self.a = 0;
                        self.sr |= Cpu::CarryFlag;
                        self.set_zerof(self.a, true);
                    },
                    Some(value) =>
                    {
                        self.a = value;
                        self.set_zerof(self.a, true);
                        self.set_negf(self.a, false);

                        // Set overflow flag
                        self.set_vflag(prev_a, self.a);
                    }
                }
            },
            isa::Instruction::SBC_IMM =>
            {
                let operand = self.get_imm();

                // Previous a
                let prev_a = self.a & 0b10000000;
                let option = self.a.checked_sub(operand);

                match option
                {
                    None => 
                    {
                        self.a = 0;
                        self.sr |= Cpu::CarryFlag;
                        self.set_zerof(self.a, true);
                    },
                    Some(value) =>
                    {
                        self.a = value;
                        self.set_zerof(self.a, true);
                        self.set_negf(self.a, false);

                        // Set overflow flag
                        self.set_vflag(prev_a, self.a);
                    }
                }
            },
            isa::Instruction::SBC_INDX =>
            {
                let operand = self.get_indx();

                // Previous a
                let prev_a = self.a & 0b10000000;
                let option = self.a.checked_sub(operand);

                match option
                {
                    None => 
                    {
                        self.a = 0;
                        self.sr |= Cpu::CarryFlag;
                        self.set_zerof(self.a, true);
                    },
                    Some(value) =>
                    {
                        self.a = value;
                        self.set_zerof(self.a, true);
                        self.set_negf(self.a, false);

                        // Set overflow flag
                        self.set_vflag(prev_a, self.a);
                    }
                }
            },
            isa::Instruction::SBC_INDY =>
            {
                let operand = self.get_indy();

                // Previous a
                let prev_a = self.a & 0b10000000;
                let option = self.a.checked_sub(operand);

                match option
                {
                    None => 
                    {
                        self.a = 0;
                        self.sr |= Cpu::CarryFlag;
                        self.set_zerof(self.a, true);
                    },
                    Some(value) =>
                    {
                        self.a = value;
                        self.set_zerof(self.a, true);
                        self.set_negf(self.a, false);

                        // Set overflow flag
                        self.set_vflag(prev_a, self.a);
                    }
                }
            },
            isa::Instruction::SBC_ZP =>
            {
                let operand = self.get_zp();

                // Previous a
                let prev_a = self.a & 0b10000000;
                let option = self.a.checked_sub(operand);

                match option
                {
                    None => 
                    {
                        self.a = 0;
                        self.sr |= Cpu::CarryFlag;
                        self.set_zerof(self.a, true);
                    },
                    Some(value) =>
                    {
                        self.a = value;
                        self.set_zerof(self.a, true);
                        self.set_negf(self.a, false);

                        // Set overflow flag
                        self.set_vflag(prev_a, self.a);
                    }
                }
            },
            isa::Instruction::SBC_ZPX =>
            {
                let operand = self.get_zpx();

                // Previous a
                let prev_a = self.a & 0b10000000;
                let option = self.a.checked_sub(operand);

                match option
                {
                    None => 
                    {
                        self.a = 0;
                        self.sr |= Cpu::CarryFlag;
                        self.set_zerof(self.a, true);
                    },
                    Some(value) =>
                    {
                        self.a = value;
                        self.set_zerof(self.a, true);
                        self.set_negf(self.a, false);

                        // Set overflow flag
                        self.set_vflag(prev_a, self.a);
                    }
                }
            },
            isa::Instruction::SEC_IMP =>
            {
                self.sr |= Cpu::CarryFlag;
            },
            isa::Instruction::SED_IMP =>
            {
                self.sr |= Cpu::DecimalFlag;
            },
            isa::Instruction::SEI_IMP =>
            {
                self.sr |= Cpu::InterruptFlag;
            },
            isa::Instruction::STA_ABS =>
            {
                 // *self.get_abs_ref() = self.a;
                let val = self.a;
                let operand = self.get_abs_ref();
                *operand = val;
            },
            isa::Instruction::STA_ABSX =>
            {
                let val = self.a;
                let operand = self.get_absx_ref();
                *operand = val;
            },
            isa::Instruction::STA_ABSY =>
            {
                let val = self.a;
                let operand = self.get_absy_ref();
                *operand = val;
            },
            isa::Instruction::STA_INDX =>
            {
                let val = self.a;
                let operand = self.get_indx_ref();
                *operand = val;
            },
            isa::Instruction::STA_INDY =>
            {
                let val = self.a;
                let operand = self.get_indy_ref();
                *operand = val;
            },
            isa::Instruction::STA_ZP =>
            {
                let val = self.a;
                let operand = self.get_zp_ref();
                *operand = val;
            },
            isa::Instruction::STA_ZPX =>
            {
                let val = self.a;
                let operand = self.get_zpx_ref();
                *operand = val;
            },
            isa::Instruction::STX_ABS =>
            {
                let val = self.x;
                let operand = self.get_zpx_ref();
                *operand = val;
            },
            isa::Instruction::STX_ZP =>
            {
                let val = self.x;
                let operand = self.get_zp_ref();
                *operand = val;
            },
            isa::Instruction::STX_ZPY =>
            {
                let val = self.x;
                let operand = self.get_zpy_ref();
                *operand = val;
            },
            isa::Instruction::STY_ABS =>
            {
                let val = self.y;
                let operand = self.get_abs_ref();
                *operand = val;
            },
            isa::Instruction::STY_ZP =>
            {
                let val = self.y;
                let operand = self.get_zp_ref();
                *operand = val;
            },
            isa::Instruction::STY_ZPX =>
            {
                let val = self.y;
                let operand = self.get_zpx_ref();
                *operand = val;
            },
            isa::Instruction::TAX_IMP =>
            {
                self.x = self.a;

                self.set_zerof(self.x, true);
                self.set_negf(self.x, true);
            },
            isa::Instruction::TAY_IMP =>
            {
                self.y = self.a;

                self.set_zerof(self.y, true);
                self.set_negf(self.y, true);
            },
            isa::Instruction::TSX_IMP =>
            {
                self.x = self.sr;

                self.set_zerof(self.x, true);
                self.set_negf(self.x, true);
            },
            isa::Instruction::TXA_IMP =>
            {
                self.a = self.x;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::TXS_IMP =>
            {
                self.sr = self.x;
            },
            isa::Instruction::TYA_IMP =>
            {
                self.a = self.y;

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            _ => { panic!("Opcode {:#4x} at pc {:#4x} not supported", opcode, self.pc - 1) }

        }
    }

    #[inline(always)]
    fn adc(&mut self, operand : u8)
    {
        let prev_a = self.a;
        let option = self.a.checked_add(operand);

        match option
        {
            None => 
            {
                self.a = 0;
                self.sr |= Cpu::CarryFlag;
                self.set_zerof(self.a, true);
                self.set_negf(self.a, false);
                self.set_vflag(prev_a, self.a);
            },
            Some(value) =>
            {
                self.a = value;
                self.set_zerof(self.a, true);
                self.set_negf(self.a, false);
                self.sr &= !Cpu::CarryFlag;

                // Set overflow flag
                self.set_vflag(prev_a, self.a);
            }
        }
    }

    fn asl(&mut self, operand : u8) -> u8
    {
        let mut res = operand;
        let prev_val = operand;
        res = prev_val << 1;

        if prev_val & 0b10000000 == 0b10000000
        {
            self.sr |= Cpu::CarryFlag;
        }
        else
        {
            self.sr &= !Cpu::CarryFlag;
        }

        self.set_zerof(res, true);
        self.set_negf(res, true);

        res 
    }

    pub fn step(&mut self)
    {
        let opcode = self.mem[self.pc as usize];
        self.pc += 1;

        self.execute_instruction(opcode);
    }

    pub fn print_regs(&self)
    {
        println!("Reg\tValue");

        println!("A\t{:04x}", self.a);
        println!("X\t{:04x}", self.x);
        println!("Y\t{:04x}", self.y);
        println!("SP\t{:04x}", self.sp);
        println!("PC\t{:04x}", self.pc);
        println!("ST\t{:08b}\n-------", self.sr);
    }
}
