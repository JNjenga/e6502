
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
        self.sr = 0;
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

    fn get_absy_ref(&mut self) -> &mut u8
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
        self.mem[usize::from(value)]
    }

    fn get_zp_ref(&mut self) -> &mut u8
    {
        let value = self.mem[usize::from(self.pc)];
        self.pc += 1;
        &mut self.mem[usize::from(value)]
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

    fn get_zpy(&mut self) -> u8
    {
        let value = self.y + self.mem[usize::from(self.pc)];
        self.pc += 1;
        self.mem[usize::from(value)]
    }

    fn execute_instruction(&mut self, opcode: u8)
    {
        match opcode
        {
            isa::Instruction::ADC_IMM =>
            {
                let operand = self.get_imm();

                let option = self.a.checked_add(operand);

                match option
                {
                    None => 
                    {
                        self.sr |= Cpu::OverFlowFlag;
                        self.sr |= Cpu::CarryFlag;
                    },
                    Some(value) =>
                    {
                        if value == 0
                        {
                            self.sr |= Cpu::ZeroFlag;
                        }

                        if value >> 7 == 0
                        {
                            self.sr |= Cpu::NegFlag;
                        }
                    }
                }
            },
            isa::Instruction::ADC_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => 
                    {
                        self.sr |= Cpu::OverFlowFlag;
                        self.sr |= Cpu::CarryFlag;
                    },
                    Some(value) =>
                    {
                        if value == 0
                        {
                            self.sr |= Cpu::ZeroFlag;
                        }

                        if value >> 7 == 0
                        {
                            self.sr |= Cpu::NegFlag;
                        }
                    }
                }
            },
            isa::Instruction::ADC_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => 
                    {
                        self.sr |= Cpu::OverFlowFlag;
                        self.sr |= Cpu::CarryFlag;
                    },
                    Some(value) =>
                    {
                        if value == 0
                        {
                            self.sr |= Cpu::ZeroFlag;
                        }

                        if value >> 7 == 0
                        {
                            self.sr |= Cpu::NegFlag;
                        }
                    }
                }
                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ADC_ABSY =>
            {
                let operand = self.get_absy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => 
                    {
                        self.sr |= Cpu::OverFlowFlag;
                        self.sr |= Cpu::CarryFlag;
                    },
                    Some(value) =>
                    {
                        if value == 0
                        {
                            self.sr |= Cpu::ZeroFlag;
                        }

                        if value >> 7 == 0
                        {
                            self.sr |= Cpu::NegFlag;
                        }
                    }
                }
            },
            isa::Instruction::ADC_INDX =>
            {
                let operand = self.get_indx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => 
                    {
                        self.sr |= Cpu::OverFlowFlag;
                        self.sr |= Cpu::CarryFlag;
                    },
                    Some(value) =>
                    {
                        if value == 0
                        {
                            self.sr |= Cpu::ZeroFlag;
                        }

                        if value >> 7 == 0
                        {
                            self.sr |= Cpu::NegFlag;
                        }
                    }
                }
            },
            isa::Instruction::ADC_INDY =>
            {
                let operand = self.get_indy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => 
                    {
                        self.sr |= Cpu::OverFlowFlag;
                        self.sr |= Cpu::CarryFlag;
                    },
                    Some(value) =>
                    {
                        if value == 0
                        {
                            self.sr |= Cpu::ZeroFlag;
                        }

                        if value >> 7 == 0
                        {
                            self.sr |= Cpu::NegFlag;
                        }
                    }
                }
            },
            isa::Instruction::ADC_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => 
                    {
                        self.sr |= Cpu::OverFlowFlag;
                        self.sr |= Cpu::CarryFlag;
                    },
                    Some(value) =>
                    {
                        if value == 0
                        {
                            self.sr |= Cpu::ZeroFlag;
                        }

                        if value >> 7 == 0
                        {
                            self.sr |= Cpu::NegFlag;
                        }
                    }
                }
            },
            isa::Instruction::ADC_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => 
                    {
                        self.sr |= Cpu::OverFlowFlag;
                        self.sr |= Cpu::CarryFlag;
                    },
                    Some(value) =>
                    {
                        if value == 0
                        {
                            self.sr |= Cpu::ZeroFlag;
                        }

                        if value >> 7 == 0
                        {
                            self.sr |= Cpu::NegFlag;
                        }
                    }
                }
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
                let operand = self.get_abs();
                self.a <<= 1;
            },
            isa::Instruction::ASL_ABS =>
            {
                let operand = self.get_abs_ref();
                *operand <<= 1;
            },
            isa::Instruction::ASL_ABSX =>
            {
                let operand = self.get_absx_ref();
                *operand <<= 1;
            },
            isa::Instruction::ASL_ZP =>
            {
                let operand = self.get_zp_ref();
                *operand <<= 1;
            },
            isa::Instruction::ASL_ZPX =>
            {
                let operand = self.get_zpx_ref();
                *operand <<= 1;
            },
            isa::Instruction::BCC_REL =>
            {
                let carry_flag = (self.sr << 7) >> 7;
                if carry_flag == 0
                {
                    let operand = self.get_rel();
                    self.pc = operand;
                }
            },
            isa::Instruction::BCS_REL =>
            {
                let carry_flag = (self.sr << 7) >> 7;
                if carry_flag == 1
                {
                    let operand = self.get_rel();
                    self.pc = operand;
                }
            },
            isa::Instruction::BEQ_REL =>
            {
                let zero_flag = (self.sr << 6) >> 7;
                if zero_flag == 1
                {
                    let operand = self.get_rel();
                    self.pc = operand;
                }
            },
            isa::Instruction::BIT_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::BIT_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::BMI_REL =>
            {
                let operand = self.get_rel();
            },
            isa::Instruction::BNE_REL =>
            {
                let operand = self.get_rel();
            },
            isa::Instruction::BPL_REL =>
            {
                let operand = self.get_rel();
            },
            isa::Instruction::BVC_REL =>
            {
                let operand = self.get_rel();
            },
            isa::Instruction::BVS_REL =>
            {
                let operand = self.get_rel();
            },
            isa::Instruction::CMP_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CMP_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CMP_ABSY =>
            {
                let operand = self.get_absy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CMP_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CMP_INDX =>
            {
                let operand = self.get_indx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CMP_INDY =>
            {
                let operand = self.get_indy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CMP_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CMP_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CPX_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CPX_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CPX_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CPY_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CPY_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::CPY_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::DEC_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::DEC_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::DEC_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::DEC_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::EOR_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::EOR_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::EOR_ABSY =>
            {
                let operand = self.get_absy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::EOR_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::EOR_INDX =>
            {
                let operand = self.get_indx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::EOR_INDY =>
            {
                let operand = self.get_indy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::EOR_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::EOR_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::INC_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::INC_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::INC_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::INC_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::JMP_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::JMP_IND =>
            {
                let operand = self.get_ind();

            },
            isa::Instruction::JSR_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDA_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDA_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDA_ABSY =>
            {
                let operand = self.get_absy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDA_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDA_INDX =>
            {
                let operand = self.get_indx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDA_INDY =>
            {
                let operand = self.get_indy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDA_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDA_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDX_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDX_ABSY =>
            {
                let operand = self.get_absy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDX_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDX_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDX_ZPY =>
            {
                let operand = self.get_zpy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDY_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDY_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDY_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDY_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LDY_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LSR_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LSR_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LSR_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::LSR_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ORA_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ORA_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ORA_ABSY =>
            {
                let operand = self.get_absy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ORA_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ORA_INDX =>
            {
                let operand = self.get_indx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ORA_INDY =>
            {
                let operand = self.get_indy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ORA_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ORA_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ROL_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ROL_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ROL_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ROL_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ROR_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ROR_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ROR_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::ROR_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::SBC_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::SBC_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::SBC_ABSY =>
            {
                let operand = self.get_absy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::SBC_IMM =>
            {
                let operand = self.get_imm();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::SBC_INDX =>
            {
                let operand = self.get_indx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::SBC_INDY =>
            {
                let operand = self.get_indy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::SBC_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::SBC_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STA_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STA_ABSX =>
            {
                let operand = self.get_absx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STA_ABSY =>
            {
                let operand = self.get_absy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STA_INDX =>
            {
                let operand = self.get_indx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STA_INDY =>
            {
                let operand = self.get_indy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STA_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STA_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STX_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STX_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STX_ZPY =>
            {
                let operand = self.get_zpy();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STY_ABS =>
            {
                let operand = self.get_abs();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STY_ZP =>
            {
                let operand = self.get_zp();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            isa::Instruction::STY_ZPX =>
            {
                let operand = self.get_zpx();
                let option = self.a.checked_add(operand);

                match option
                {
                    None => { self.sr |= Cpu::OverFlowFlag; },
                    _ => {}
                }
            },
            _ => panic!("Opcode not supported")

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
        println!("ST\t{:04x}", self.sr);
    }
}
