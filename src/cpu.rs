
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
        // Add with carry
        let x = self.x;
        let option = res.checked_add(u16::from(x));

        match option
        {
            None => 
            {
                res = res.wrapping_add(u16::from(x));
                self.sr |= Cpu::CarryFlag;
            },
            Some(value) =>
            {
                res = value;
                self.sr &= !Cpu::CarryFlag;
            }
        }
        if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
        {
            res += 1;
        }

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
        // Add with carry
        let x = self.x;
        let option = res.checked_add(u16::from(x));

        match option
        {
            None => 
            {
                res = res.wrapping_add(u16::from(x));
                self.sr |= Cpu::CarryFlag;
            },
            Some(value) =>
            {
                res = value;
                self.sr &= !Cpu::CarryFlag;
            }
        }
        if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
        {
            res += 1;
        }

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
        // Add with carry
        let x = self.x;
        let option = res.checked_add(u16::from(x));

        match option
        {
            None => 
            {
                res = res.wrapping_add(u16::from(x));
                self.sr |= Cpu::CarryFlag;
            },
            Some(value) =>
            {
                res = value;
                self.sr &= !Cpu::CarryFlag;
            }
        }
        if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
        {
            res += 1;
        }


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
        let y = self.y;
        let option = res.checked_add(u16::from(y));

        match option
        {
            None => 
            {
                res = res.wrapping_add(u16::from(y));
                self.sr |= Cpu::CarryFlag;
            },
            Some(value) =>
            {
                res = value;
                self.sr &= !Cpu::CarryFlag;
            }
        }
        if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
        {
            res += 1;
        }

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
        let y = self.y;
        let option = res.checked_add(u16::from(y));

        match option
        {
            None => 
            {
                res = res.wrapping_add(u16::from(y));
                self.sr |= Cpu::CarryFlag;
            },
            Some(value) =>
            {
                res = value;
                self.sr &= !Cpu::CarryFlag;
            }
        }
        if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
        {
            res += 1;
        }

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
        let y = self.y;
        let option = res.checked_add(u16::from(y));

        match option
        {
            None => 
            {
                res = res.wrapping_add(u16::from(y));
                self.sr |= Cpu::CarryFlag;
            },
            Some(value) =>
            {
                res = value;
                self.sr &= !Cpu::CarryFlag;
            }
        }
        if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
        {
            res += 1;
        }

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
        let operand = self.mem[usize::from(self.pc)];
        let mut address: u16 = self.mem[usize::from(operand + 1)] as u16;
        address <<= 8;
        address |= self.mem[usize::from(operand)] as u16;
       address += self.x as u16;

        self.pc += 1;
        self.mem[usize::from(address)]
    }

    fn get_indx_ref(&mut self) -> &mut u8 
    {
        let operand = self.mem[usize::from(self.pc)];
        let mut address: u16 = self.mem[usize::from(operand + 1)] as u16;
        address <<= 8;
        address |= self.mem[usize::from(operand)] as u16;
        address += self.x as u16;

        self.pc += 1;
        &mut self.mem[usize::from(address)]
    }

    fn get_indx_address(&mut self) -> u16
    {
        let operand = self.mem[usize::from(self.pc)];
        let mut address: u16 = self.mem[usize::from(operand + 1)] as u16;
        address <<= 8;
        address |= self.mem[usize::from(operand)] as u16;
        address += self.x as u16;

        self.pc += 1;
        address
    }

    fn get_indy(&mut self) -> u8 
    {
        let operand = self.mem[usize::from(self.pc)];
        let mut address: u16 = self.mem[usize::from(operand + 1)] as u16;
        address <<= 8;
        address |= self.mem[usize::from(operand)] as u16;
        address += self.y as u16;

        self.pc += 1;
        self.mem[usize::from(address)]
    }

    fn get_indy_ref(&mut self) -> &mut u8 
    { 
        let operand = self.mem[usize::from(self.pc)];
        let mut address: u16 = self.mem[usize::from(operand + 1)] as u16;
        address <<= 8;
        address |= self.mem[usize::from(operand)] as u16;
        address += self.y as u16;

        self.pc += 1;
        &mut self.mem[usize::from(address)]
    }

    fn get_indy_address(&mut self) -> u16
    {
        let operand = self.mem[usize::from(self.pc)];
        let mut address: u16 = self.mem[usize::from(operand + 1)] as u16;
        address <<= 8;
        address |= self.mem[usize::from(operand)] as u16;
        address += self.y as u16;

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

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
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

                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
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
                let value = self.get_abs_address() as usize;

                self.mem[value] = self.asl(self.mem[value]);
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
            isa::Instruction::BCC_REL | isa::Instruction::BCC_REL_16 =>
            {
                let carry_flag = self.sr & Cpu::CarryFlag;
                let operand = self.get_rel();
                if carry_flag == 0
                {
                    self.pc = operand;
                }
            },
            isa::Instruction::BCS_REL | isa::Instruction::BCS_REL_16 =>
            {
                let carry_flag = self.sr & Cpu::CarryFlag;
                let operand = self.get_rel();
                if carry_flag == Cpu::CarryFlag
                {
                    self.pc = operand;
                }
            },
            isa::Instruction::BEQ_REL | isa::Instruction::BEQ_REL_16 =>
            {
                let zero_flag = self.sr & Cpu::ZeroFlag;
                let operand = self.get_rel();
                if zero_flag == Cpu::ZeroFlag
                {
                    self.pc = operand;
                }
            },
            isa::Instruction::BIT_ABS =>
            {
                let operand = self.get_abs();
                self.bit(operand);

            },
            isa::Instruction::BIT_ZP =>
            {
                let operand = self.get_zp();
                self.bit(operand);
            },
            isa::Instruction::BMI_REL | isa::Instruction::BMI_REL_16 =>
            {
                let neg_flag = self.sr & Cpu::NegFlag;
                let operand = self.get_rel();
                if neg_flag == Cpu::NegFlag 
                {
                    self.pc = operand;
                }
            },
            isa::Instruction::BNE_REL | isa::Instruction::BNE_REL_16 =>
            {
                let zero_flag = self.sr & Cpu::ZeroFlag;
                let operand = self.get_rel();
                if zero_flag == 0
                {
                    self.pc = operand;
                }
            },
            isa::Instruction::BPL_REL | isa::Instruction::BPL_REL_16 =>
            {
                let neg_flag = self.sr & Cpu::NegFlag;
                let operand = self.get_rel();
                if neg_flag == 0
                {
                    self.pc = operand;
                }
            },
            isa::Instruction::BRK_IMP =>
            {
                self.sr |= Cpu::BreakFlag;
                self.pc += 1;
            },
            isa::Instruction::BVC_REL | isa::Instruction::BVC_REL_16 =>
            {
                let overflow_flag = self.sr & Cpu::OverFlowFlag;
                let operand = self.get_rel();
                if overflow_flag == 0
                {
                    self.pc = operand;
                }
            },
            isa::Instruction::BVS_REL | isa::Instruction::BVS_REL_16 =>
            {
                let overflow_flag = self.sr & Cpu::OverFlowFlag;
                let operand = self.get_rel();
                if overflow_flag == Cpu::OverFlowFlag 
                {
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

                self.cmp(self.a, operand);
            },
            isa::Instruction::CMP_ABSX =>
            {
                let operand = self.get_absx();
                self.cmp(self.a, operand);
            },
            isa::Instruction::CMP_ABSY =>
            {
                let operand = self.get_absy();
                self.cmp(self.a, operand);
            },
            isa::Instruction::CMP_IMM =>
            {
                let operand = self.get_imm();
                self.cmp(self.a, operand);
            },
            isa::Instruction::CMP_INDX =>
            {
                let operand = self.get_indx();
                self.cmp(self.a, operand);
            },
            isa::Instruction::CMP_INDY =>
            {
                let operand = self.get_indy();
                self.cmp(self.a, operand);
            },
            isa::Instruction::CMP_ZP =>
            {
                let operand = self.get_zp();
                self.cmp(self.a, operand);
            },
            isa::Instruction::CMP_ZPX =>
            {
                let operand = self.get_zpx();
                self.cmp(self.a, operand);
            },
            isa::Instruction::CPX_ABS =>
            {
                let operand = self.get_abs();
                self.cmp(self.x, operand);
            },
            isa::Instruction::CPX_IMM =>
            {
                let operand = self.get_imm();
                self.cmp(self.x, operand);
            },
            isa::Instruction::CPX_ZP =>
            {
                let operand = self.get_zp();
                self.cmp(self.x, operand);
            },
            isa::Instruction::CPY_ABS =>
            {
                let operand = self.get_abs();
                self.cmp(self.y, operand);
            },
            isa::Instruction::CPY_IMM =>
            {
                let operand = self.get_imm();
                self.cmp(self.y, operand);
            },
            isa::Instruction::CPY_ZP =>
            {
                let operand = self.get_zp();
                self.cmp(self.y, operand);
            },
            isa::Instruction::DEC_ABS =>
            {
                let value = self.get_abs_address() as usize;
                self.mem[value] = self.dec(self.mem[value]);
            },
            isa::Instruction::DEC_ABSX =>
            {
                let value = self.get_absx_address() as usize;
                self.mem[value] = self.dec(self.mem[value]);
            },
            isa::Instruction::DEC_ZP =>
            {
                let value = self.get_zp_address() as usize;
                self.mem[value] = self.dec(self.mem[value]);
            },
            isa::Instruction::DEC_ZPX =>
            {
                let value = self.get_zpx_address() as usize;
                self.mem[value] = self.dec(self.mem[value]);
            },
            isa::Instruction::DEX_IMP =>
            {
                self.x = self.x.wrapping_sub(1);

                self.set_zerof(self.x, true);
                self.set_negf(self.x, true);
            },
            isa::Instruction::DEY_IMP =>
            {
                self.y = self.y.wrapping_sub(1);

                self.set_zerof(self.y, true);
                self.set_negf(self.y, true);
            },
            isa::Instruction::EOR_ABS =>
            {
                let operand = self.get_abs();
                self.eor(operand);
            },
            isa::Instruction::EOR_ABSX =>
            {
                let operand = self.get_absx();
                self.eor(operand);
            },
            isa::Instruction::EOR_ABSY =>
            {
                let operand = self.get_absy();
                self.eor(operand);
            },
            isa::Instruction::EOR_IMM =>
            {
                let operand = self.get_imm();
                self.eor(operand);
            },
            isa::Instruction::EOR_INDX =>
            {
                let operand = self.get_indx();
                self.eor(operand);
            },
            isa::Instruction::EOR_INDY =>
            {
                let operand = self.get_indy();
                self.eor(operand);
            },
            isa::Instruction::EOR_ZP =>
            {
                let operand = self.get_zp();
                self.eor(operand);
            },
            isa::Instruction::EOR_ZPX =>
            {
                let operand = self.get_zpx();
                self.eor(operand);
            },
            isa::Instruction::INC_ABS =>
            {
                let value = self.get_abs_address() as usize;
                self.mem[value] = self.inc(self.mem[value]);
            },
            isa::Instruction::INC_ABSX =>
            {
                let value = self.get_absx_address() as usize;
                self.mem[value] = self.inc(self.mem[value]);
            },
            isa::Instruction::INC_ZP =>
            {
                let value = self.get_zp_address() as usize;
                self.mem[value] = self.inc(self.mem[value]);
            },
            isa::Instruction::INC_ZPX =>
            {
                let value = self.get_zpx_address() as usize;
                self.mem[value] = self.inc(self.mem[value]);
            },
            isa::Instruction::INX_IMP =>
            {
                self.x = self.x.wrapping_add(1);

                self.set_zerof(self.x, true);
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

                self.stack_push_16(self.pc);
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
                let lsb = (self.a << 7) >> 7;
                self.a = self.a >> 1;

                self.sr &= !Cpu::NegFlag;

                if lsb == 0
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
                let value = self.get_abs_address() as usize;
                self.mem[value] = self.lsr(self.mem[value]);
            },
            isa::Instruction::LSR_ABSX =>
            {
                let value = self.get_absx_address() as usize;
                self.mem[value] = self.lsr(self.mem[value]);
            },
            isa::Instruction::LSR_ZP =>
            {
                let value = self.get_zp_address() as usize;
                self.mem[value] = self.lsr(self.mem[value]);
            },
            isa::Instruction::LSR_ZPX =>
            {
                let value = self.get_zpx_address() as usize;
                self.mem[value] = self.lsr(self.mem[value]);
            },
            isa::Instruction::NOP_IMP =>
            {
                self.pc += 1;
            },
            isa::Instruction::ORA_ABS =>
            {
                let operand = self.get_abs();
                self.ora(operand);
            },
            isa::Instruction::ORA_ABSX =>
            {
                let operand = self.get_absx();
                self.ora(operand);
            },
            isa::Instruction::ORA_ABSY =>
            {
                let operand = self.get_absy();
                self.ora(operand);
            },
            isa::Instruction::ORA_IMM =>
            {
                let operand = self.get_imm();
                self.ora(operand);
            },
            isa::Instruction::ORA_INDX =>
            {
                let operand = self.get_indx();
                self.ora(operand);
            },
            isa::Instruction::ORA_INDY =>
            {
                let operand = self.get_indy();
                self.ora(operand);
            },
            isa::Instruction::ORA_ZP =>
            {
                let operand = self.get_zp();
                self.ora(operand);
            },
            isa::Instruction::ORA_ZPX =>
            {
                let operand = self.get_zpx();
                self.ora(operand);
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

                // Set carry
                self.sr &= hsb;
                self.set_zerof(self.a, true);
                self.set_negf(self.a, true);
            },
            isa::Instruction::ROL_ABS =>
            {
                let value = self.get_abs_address() as usize;
                self.mem[value] = self.rol(self.mem[value]);
            },
            isa::Instruction::ROL_ABSX =>
            {
                let value = self.get_absx_address() as usize;
                self.mem[value] = self.rol(self.mem[value]);
            },
            isa::Instruction::ROL_ZP =>
            {
                let value = self.get_absy_address() as usize;
                self.mem[value] = self.rol(self.mem[value]);
            },
            isa::Instruction::ROL_ZPX =>
            {
                let value = self.get_zpx_address() as usize;
                self.mem[value] = self.rol(self.mem[value]);
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
                let value = self.get_abs_address() as usize;
                self.mem[value] = self.rol(self.mem[value]);
            },
            isa::Instruction::ROR_ABSX =>
            {
                let value = self.get_absx_address() as usize;
                self.mem[value] = self.rol(self.mem[value]);
            },
            isa::Instruction::ROR_ZP =>
            {
                let value = self.get_zp_address() as usize;
                self.mem[value] = self.rol(self.mem[value]);
            },
            isa::Instruction::ROR_ZPX =>
            {
                let value = self.get_zpx_address() as usize;
                self.mem[value] = self.rol(self.mem[value]);
            },
            isa::Instruction::RTI_IMP =>
            {
                self.sp = self.stack_pop();
                self.pc = self.stack_pop_16();
            },
            isa::Instruction::RTS_IMP =>
            {
                let operand = self.stack_pop_16();
                self.pc = operand + 1;
            },
            isa::Instruction::SBC_ABS =>
            {
                let operand = self.get_abs();
                self.sbc(operand);
            },
            isa::Instruction::SBC_ABSX =>
            {
                let operand = self.get_absx();
                self.sbc(operand);
            },
            isa::Instruction::SBC_ABSY =>
            {
                let operand = self.get_absy();
                self.sbc(operand);
            },
            isa::Instruction::SBC_IMM =>
            {
                let operand = self.get_imm();
                self.sbc(operand);
            },
            isa::Instruction::SBC_INDX =>
            {
                let operand = self.get_indx();
                self.sbc(operand);
            },
            isa::Instruction::SBC_INDY =>
            {
                let operand = self.get_indy();
                self.sbc(operand);
            },
            isa::Instruction::SBC_ZP =>
            {
                let operand = self.get_zp();
                self.sbc(operand);
            },
            isa::Instruction::SBC_ZPX =>
            {
                let operand = self.get_zpx();
                self.sbc(operand);
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
                let address = self.get_absx_address() as usize;
                self.mem[address] = self.a;
                // println!("STA_ABSX: Storing {:#4x} to {:#4x}", self.a, address);
                // let val = self.a;
                // let operand = self.get_absx_ref();
                // *operand = val;
            },
            isa::Instruction::STA_ABSY =>
            {
                let val = self.a;
                let operand = self.get_absy_ref();
                *operand = val;
            },
            isa::Instruction::STA_INDX =>
            {
                let address = self.get_indx_address() as usize;
                self.mem[address] = self.a;
            },
            isa::Instruction::STA_INDY =>
            {
                let address = self.get_indy_address() as usize;
                self.mem[address] = self.a;
            },
            isa::Instruction::STA_ZP =>
            {
                let address = self.get_zp_address() as usize;
                self.mem[address] = self.a;
                // println!("STA_ZP: Storing {:#4x} to {:#4x}", self.a, address);
            },
            isa::Instruction::STA_ZPX =>
            {
                let address = self.get_zpy_address() as usize;
                self.mem[address] = self.a;
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
            _ => { self.print_regs();panic!("Opcode {:#4x} at pc {:#4x} not supported", opcode, self.pc - 1) }

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
                self.a = self.a.wrapping_add(operand);
                self.sr |= Cpu::CarryFlag;
            },
            Some(value) =>
            {
                self.a = value;
                self.sr &= !Cpu::CarryFlag;
            }
        }
        if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
        {
            self.a += 1;
        }


        self.set_vflag(prev_a, self.a);
        self.set_negf(self.a, true);
        self.set_zerof(self.a, true);
    }

    #[inline(always)]
    fn asl(&mut self, operand : u8) -> u8
    {
        let prev_val = operand;
        let res = prev_val << 1;

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

    #[inline(always)]
    fn bit(&mut self, operand: u8)
    {
        self.set_negf(operand, true);

        // Set overflow flag
        let overflow_flag = (operand << 1) >> 7;
        if overflow_flag == 0
        {
            self.sr |= Cpu::OverFlowFlag;
        }
        else
        {
            self.sr &= !Cpu::OverFlowFlag;
        }

        self.set_zerof(self.a & operand, true);
    }

    #[inline(always)]
    fn cmp(&mut self, left : u8, right : u8)
    {
        if left < right
        {
            self.sr |= Cpu::NegFlag;
            self.sr &= !Cpu::ZeroFlag;
            self.sr &= !Cpu::CarryFlag;
        }
        else if left == right 
        {
            self.sr |= Cpu::ZeroFlag;
            self.sr |= Cpu::CarryFlag;
            self.sr &= !Cpu::NegFlag;
        }
        else if left >= right
        {
            self.sr |= Cpu::CarryFlag;
            self.sr &= !Cpu::NegFlag;
            self.sr &= !Cpu::ZeroFlag;
        }
    }

    #[inline(always)]
    fn dec(&mut self, operand : u8) -> u8
    {
        let res = operand.wrapping_sub(1);

        self.set_zerof(res, true);
        self.set_negf(res, true);
        res
    }

    #[inline(always)]
    fn eor(&mut self, operand: u8)
    {
        self.a ^= operand;
        self.set_zerof(self.a, true);
        self.set_negf(self.a, true);
    }

    #[inline(always)]
    fn inc(&mut self, operand : u8) -> u8
    {
        let res = operand.wrapping_add(1);

        self.set_zerof(res, true);
        self.set_negf(res, true);
        res
    }

    #[inline(always)]
    fn lsr(&mut self, operand : u8) -> u8
    {
        let lsb = (operand << 7) >> 7;
        let res = operand >> 1;

        self.sr &= !Cpu::NegFlag;

        if lsb == 0
        {
            self.sr &= !Cpu::CarryFlag;
        }
        else
        {
            self.sr |= Cpu::CarryFlag;
        }

        self.set_zerof(res, true);
        res
    }

    #[inline(always)]
    fn ora(&mut self, operand: u8)
    {
        self.a |= operand;
        self.set_zerof(self.a, true);
        self.set_negf(self.a, true);
    }

    #[inline(always)]
    fn rol(&mut self, operand : u8) -> u8
    {
        let hsb = operand >> 7;

        let prev_carry = self.sr & Cpu::CarryFlag;

        let res = (operand << 1) | prev_carry;

        // Set carry
        self.sr &= hsb;
        self.set_zerof(res, true);
        self.set_negf(res, true);
        res
    }

    #[inline(always)]
    fn ror(&mut self, operand : u8) -> u8
    {
        let lsb = (operand << 7) >> 7;
        let prev_carry = self.sr & Cpu::CarryFlag;

        let res = (operand >> 1) | (prev_carry << 7);

        self.sr &= lsb;
        self.set_zerof(res, true);
        self.set_negf(res, true);
        res
    }

    #[inline(always)]
    fn sbc(&mut self, operand: u8)
    {
        let prev_a = self.a;
        let option = self.a.checked_sub(operand);

        match option
        {
            None => 
            {
                self.a = self.a.wrapping_sub(operand);
                if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
                {
                    self.a -= 1;
                }

                self.sr |= Cpu::CarryFlag;
            },
            Some(value) =>
            {
                self.a = value;
                if  self.sr & Cpu::CarryFlag == Cpu::CarryFlag
                {
                    self.a = self.a.wrapping_sub(1);
                }

                self.sr &= !Cpu::CarryFlag;
            }
        }

        self.set_vflag(prev_a, self.a);
        self.set_negf(self.a, true);
        self.set_zerof(self.a, true);
    }

    pub fn step(&mut self)
    {
        let opcode = self.mem[self.pc as usize];
        self.pc += 1;

        self.execute_instruction(opcode);
    }

    #[inline(always)]
    pub fn print_regs(&self)
    {
        println!("Reg\tValue");

        println!("A\t{:#4x}\t{:#8b}", self.a, self.a);
        println!("X\t{:#4x}\t{:#8b}", self.x, self.x);
        println!("Y\t{:#4x}\t{:#8b}", self.y, self.x);
        println!("SP\t{:#4x}\t{:#8b}", self.sp, self.sp);
        println!("PC\t{:#4x}", self.pc);
        println!("ST\t{:08b}\n-------", self.sr);
    }

    #[inline(always)]
    pub fn print_mem(&self, from:usize, to:usize)
    {
        let mut count = 0;
        print!("200: ");
        for i in from..to
        {
            print!("{:02x} ", self.mem[i]);
            if count == 15
            {
                print!("\n{:04x}: ", i);
                count = 0;
            }
            count += 1;
        }
    }
}
