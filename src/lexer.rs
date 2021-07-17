// TODO : Is this the correct way of including the file?
use crate::isa::*;
use std::collections::HashMap;
use std::num::ParseIntError;


#[allow(dead_code)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub enum TokenType
{
    WHITE,
    HASH,
    COLON,
    DOLLAR,
    PERCENT,
    COMMA,
    BRACKETOPEN,
    BRACKETCLOSE,
    NL,
    UNKNOWN,
    INSTRUCTION,
    NUMBER,
    OPERAND,
    LABEL,
    LABEL_OPERAND,
    REGX,
    REGY,
    REGA,
    EOF,

}

type TT = TokenType;

pub struct Token
{
    pub ttype : TokenType ,
    pub tstring: String,
    pub line_no : u32,
}

pub struct Lexer
{
    pub tokens : Vec<Token>,
    pub current_token: usize,
    pub labels : HashMap<String, u16>,
}

#[allow(dead_code)]
impl Lexer
{
    pub fn previous(&self) -> Option<&Token>
    {
        if self.current_token < 1 {
            return None;
        }

        Some(&self.tokens[self.current_token - 1])
    }


    pub fn current(&self) -> Option<&Token>
    {
        if self.current_token >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[self.current_token])
    }

    pub fn next(&self) -> Option<&Token>
    {
        if self.current_token+1 >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[self.current_token+1])
    }

    pub fn nextx(&self, offset:usize) -> Option<&Token>
    {
        if self.current_token+offset >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[self.current_token+offset])
    }

    pub fn step(&mut self)
    {
        self.current_token += 1;
    }

    pub fn stepx(&mut self, steps: usize)
    {
        self.current_token += steps;
    }

    pub fn get_operand_u8(&self) -> Result<u8, ParseIntError>
    {
        if let Some(nt) = self.next()
        {
            if nt.ttype == TT::DOLLAR
            {
                if let Some(nt2) = self.nextx(2)
                {
                    return u8::from_str_radix(&nt2.tstring, 16);
                }

                panic!("INVALID hex value after {} at {}", nt.tstring, nt.line_no);
            }
            if nt.ttype == TT::PERCENT
            {
                if let Some(nt2) = self.nextx(2)
                {
                    return u8::from_str_radix(&nt2.tstring, 2);
                }

                panic!("INVALID bin value after {} at {}", nt.tstring, nt.line_no);
            }

            if nt.ttype == TT::LABEL_OPERAND
            {
                return u8::from_str_radix(&nt.tstring, 16);
            }

            return u8::from_str_radix(&nt.tstring, 10);
        }

        return u8::from_str_radix("raise err", 10);

    }

    pub fn get_operand_u16(&self) -> Result<u16, ParseIntError>
    {
        if let Some(nt) = self.next()
        {
            if nt.ttype == TT::DOLLAR
            {
                if let Some(nt2) = self.nextx(2)
                {
                    return u16::from_str_radix(&nt2.tstring, 16);
                }
            }
            if nt.ttype == TT::PERCENT
            {
                if let Some(nt2) = self.nextx(2)
                {
                    return u16::from_str_radix(&nt2.tstring, 2);
                }
            }

            if nt.ttype == TT::LABEL_OPERAND
            {
                return u16::from_str_radix(&nt.tstring, 16);
            }

            return u16::from_str_radix(&nt.tstring, 10);
        }

        return u16::from_str_radix("raise err", 10);

    }

    pub fn next_mode(&self) -> u32
    {
        if let Some(nt) = self.next()
        {
            if nt.ttype == TT::EOF
            {
                return Mode::IMP;
            }
            if nt.ttype == TT::LABEL
            {
                return Mode::IMP;
            }

            if nt.ttype == TT::HASH
            {
                return Mode::IMM;
            }
            if nt.ttype == TT::REGA
            {
                return Mode::ACC;
            }

            if nt.ttype == TT::BRACKETOPEN
            {
                if let Some(nt2) = self.nextx(2)
                {
                    // println!("Type: {:?}, String : {:?}, Line No : {}", nt2.ttype, nt2.tstring, nt2.line_no);
                    if nt2.ttype == TT::UNKNOWN || nt2.ttype == TT::NUMBER
                    {
                        if let Some(nt3) = self.nextx(3)
                        {
                            if nt3.ttype == TT::BRACKETCLOSE
                            {
                                return Mode::INDY;
                            }
                            else
                            {
                                if let Some(nt4) = self.nextx(4)
                                {
                                    if nt4.ttype == TT::BRACKETCLOSE
                                    {
                                        return Mode::IND;
                                    }
                                    else if nt4.ttype == TT::REGX
                                    {
                                        return Mode::INDX;
                                    }

                                    return Mode::UNKNOWN;
                                }
                            }
                        }
                    }
                    else if nt2.ttype == TT::DOLLAR || nt2.ttype == TT::PERCENT
                    {
                        if let Some(nt4) = self.nextx(4)
                        {
                            if nt4.ttype == TT::BRACKETCLOSE
                            {
                                return Mode::INDY;
                            }
                            else
                            {
                                if let Some(nt5) = self.nextx(5)
                                {
                                    if nt5.ttype == TT::BRACKETCLOSE
                                    {
                                        return Mode::IND;
                                    }
                                    else if nt5.ttype == TT::REGX
                                    {
                                        return Mode::INDX;
                                    }

                                    return Mode::UNKNOWN;
                                }
                            }
                        }

                    }
                    return Mode::UNKNOWN;
                }
                return Mode::UNKNOWN;
            }
            if let Some(t) = self.current()
            {
                if t.tstring == "BCC"
                    || t.tstring == "BCS"
                        || t.tstring == "BEQ"
                        || t.tstring == "BMI"
                        || t.tstring == "BNE"
                        || t.tstring == "BPL"
                        || t.tstring == "BVC"
                        || t.tstring == "BVS"
                        {
                            return Mode::REL;
                        }
            }
            if nt.ttype == TT::LABEL_OPERAND
            {
                if nt.ttype == TT::BRACKETOPEN
                {

                    return Mode::IND;
                }
                else
                {
                    return Mode::ABS;
                }
            }


            // OPC $4400,X
            if nt.ttype == TT::DOLLAR || nt.ttype == TT::PERCENT
            {
                if let Some(nt2) = self.nextx(2)
                {
                    if nt2.tstring.len() <= 2 && nt2.ttype == TT::UNKNOWN
                    {
                        if let Some(nt3) = self.nextx(3)
                        {
                            if nt3.ttype == TT::COMMA
                            {
                                if let Some(nt4) = self.nextx(4)
                                {
                                    if nt4.ttype == TT::REGX
                                    {
                                        return Mode::ZPX;
                                    }
                                    if nt4.ttype == TT::REGY
                                    {
                                        return Mode::ZPY;
                                    }
                                    return Mode::UNKNOWN;
                                }
                            }
                            else
                            {
                                return Mode::ZP;
                            }
                        }
                        else
                        {
                            return Mode::ZP;
                        }
                    }
                    if nt2.tstring.len() >= 3
                    {
                        if let Some(nt3) = self.nextx(3)
                        {
                            if nt3.ttype == TT::COMMA
                            {
                                if let Some(nt4) = self.nextx(4)
                                {
                                    if nt4.ttype == TT::REGX
                                    {
                                        return Mode::ABSX;
                                    }
                                    if nt4.ttype == TT::REGY
                                    {
                                        return Mode::ABSY;
                                    }
                                }

                                return Mode::UNKNOWN;
                            }

                            return Mode::ABS;
                        }
                        else
                        {
                            return Mode::ABS;
                        }
                    }
                }
            }

            if nt.tstring.len() <= 3 && (nt.ttype == TT::UNKNOWN || nt.ttype == TT::NUMBER)
            {
                if let Some(nt2) = self.nextx(2)
                {
                    if nt2.ttype == TT::COMMA
                    {
                        if let Some(nt3) = self.nextx(3)
                        {
                            if nt3.ttype == TT::REGX
                            {
                                return Mode::ZPX;
                            }
                            if nt3.ttype == TT::REGY
                            {
                                return Mode::ZPY;
                            }
                            return Mode::UNKNOWN;
                        }
                    }
                    else
                    {
                        return Mode::ZP;
                    }
                }
                else
                {
                    return Mode::ZP;
                }
            }
            if nt.tstring.len() >= 4
            {
                if let Some(nt2) = self.nextx(2)
                {
                    if nt2.ttype == TT::COMMA
                    {
                        if let Some(nt3) = self.nextx(3)
                        {
                            if nt3.ttype == TT::REGX
                            {
                                return Mode::ABSX;
                            }
                            if nt3.ttype == TT::REGY
                            {
                                return Mode::ABSY;
                            }
                        }

                        return Mode::UNKNOWN;
                    }

                    return Mode::ABS;
                }
                else
                {
                    return Mode::ABS;
                }
            }
            if nt.ttype == TT::NUMBER
            {
                let op = u16::from_str_radix(&nt.tstring, 10).unwrap();
                if op < 256
                {
                    return Mode::ZP;
                }

                return Mode::ABS;

            }

            if nt.ttype == TT::DOLLAR
            {
                if let Some(nt2) = self.nextx(2)
                {
                    let op = u16::from_str_radix(&nt2.tstring, 16).unwrap();
                    if op < 256
                    {
                        return Mode::ZP;
                    }

                    return Mode::ABS;
                }
                return Mode::UNKNOWN;

            }

            if nt.ttype == TT::PERCENT
            {
                if let Some(nt2) = self.nextx(2)
                {
                    let op = u16::from_str_radix(&nt2.tstring, 2);

                    if op.unwrap() < 256
                    {
                        return Mode::ZP;
                    }

                    return Mode::ABS;

                }

                return Mode::UNKNOWN;

            }
            return Mode::IMP;
        }

        return Mode::IMP;
    }

    pub fn parse(&mut self) -> Vec<u8>
    {
        let mut instruction_strings = vec!["ADC", "AND", "ASL", "BCC", "BCS", "BEQ", "BIT", "BMI", "BNE", "BPL", "BRK", "BVC","BVS", "CLC", "CLD", "CLI", "CLV", "CMP", "CPX", "CPY", "DEC", "DEX", "DEY", "EOR", "INC", "INX", "INY", "JMP", "JSR", "LDA", "LDX", "LDY", "LSR", "NOP", "ORA", "PHA", "PHP", "PLA", "PLP", "ROL", "ROR", "RTI", "RTS", "SBC", "SEC", "SED", "SEI", "STA", "STX", "STY", "TAX", "TAY", "TSX", "TXA", "TXS", "TYA","adc","and","asl","bcc","bcs","beq","bit","bmi","bne","bpl","brk","bvc","bvs","clc","cld","cli","clv","cmp","cpx","cpy","dec","dex","dey","eor","inc","inx","iny","jmp","jsr","lda","ldx","ldy","lsr","nop","ora","pha","php","pla","plp","rol","ror","rti","rts","sbc","sec","sed","sei","sta","stx","sty","tax","tay","tsx","txa","txs","tya"];
        instruction_strings.sort_unstable();

        // First pass : Update unknown tokens and read labels
        loop 
        {
            let t = self.current();

            if let Some(t) = t
            {
                if t.ttype == TT::UNKNOWN
                {
                    // If instruction
                    if instruction_strings.contains(&&t.tstring[..])
                    { 
                        self.tokens[self.current_token].ttype = TT::INSTRUCTION;
                        self.tokens[self.current_token].tstring =
                            self.tokens[self.current_token].tstring.to_uppercase();
                    }
                    // If label
                    else if let Some(nt) = self.next()
                    {
                        if nt.ttype == TT::COLON
                        {
                            self.tokens[self.current_token].ttype = TT::LABEL;
                            self.labels.insert(self.tokens[self.current_token].tstring.clone(), 0);
                        }
                        else
                        {
                            let mut valid = true;
                            for tc in t.tstring.chars()
                            {
                                if tc < '0'
                                {
                                    valid = false;
                                    break;
                                }
                                else if tc > 'f'
                                {
                                    valid = false;
                                    break;
                                }
                                else if tc < 'a' && tc > 'A'
                                {
                                    valid = false;
                                    break;
                                }
                                else if tc < 'F' && tc > 'A'
                                {
                                    valid = false;
                                    break;
                                }
                            }

                            if valid
                            {
                                // println!("Type: {:?}, String : {:?}, Line No : {}", nt.ttype, nt.tstring, nt.line_no);
                                self.tokens[self.current_token].ttype = TT::NUMBER;
                            }
                        }
                    }
                    // Should we check if operand is a valid number type
                    else
                    {
                        let mut valid = true;
                        for tc in t.tstring.chars()
                        {
                            if tc < '0'
                            {
                                valid = false;
                                break;
                            }
                            else if tc > 'f'
                            {
                                valid = false;
                                break;
                            }
                            else if tc < 'a' && tc > 'A'
                            {
                                valid = false;
                                break;
                            }
                            else if tc < 'F' && tc > 'A'
                            {
                                valid = false;
                                break;
                            }
                        }

                        if valid
                        {
                            self.tokens[self.current_token].ttype = TT::NUMBER;
                        }
                    }
                }

                self.step();
            }
            else
            {
                break;
            }
        }


        // Add labels to labels table
        self.current_token = 0;
        let mut mem_index = 0;
        loop 
        {
            if let Some(t) = self.current()
            {
                if t.ttype == TT::LABEL
                {
                    match self.labels.get(&t.tstring)
                    {
                        Some(_) =>
                        {
                            println!("Label {:?} at line {} already defined", t.tstring, t.line_no);
                            panic!();
                        },
                        None =>
                        {
                            // TODO(James) : This seems messed up
                            self.labels.insert(self.tokens[self.current_token].tstring.clone(), mem_index);
                            self.step();
                            continue;
                        }
                    }
                }

                if t.ttype == TT::INSTRUCTION
                {
                    mem_index += 1;
                    let mode = self.next_mode();
                    match mode 
                    {

                        Mode::ABS => mem_index+=2,
                        Mode::ABSX => mem_index+=2,
                        Mode::ABSY => mem_index+=2,
                        Mode::IMM => mem_index+=1,
                        Mode::IMP => {},
                        Mode::IND => mem_index += 2,
                        Mode::INDX => mem_index += 1,
                        Mode::INDY => mem_index += 1,
                        Mode::REL => mem_index += 1,
                        Mode::ZP => mem_index += 1,
                        Mode::ZPX => mem_index += 1,
                        Mode::ZPY => mem_index += 1,
                        _ =>
                        {
                            panic!("Unknown addressing mode")
                        },
                    }
                    self.step();
                    continue;
                }

                self.step();

            }
            else 
            {
                break;
            }
        }

        self.current_token = 0;

        // Replace labels with correct index
        loop
        {
            if let Some(t) = self.current()
            {
                // Replace labels with values
                if t.ttype == TT::UNKNOWN
                {
                    match self.labels.get(&t.tstring)
                    {
                        Some(&value) => { 
                            self.tokens[self.current_token].ttype = TT::LABEL_OPERAND;
                            // println!("Label replace: {} set to value :{} line_no {}", self.tokens[self.current_token].tstring, value, self.tokens[self.current_token].line_no);
                            self.tokens[self.current_token].tstring = format!("{:x}",value);
                        },
                        None =>
                        {
                            println!("Unknown token {:?} at line {}", t.tstring, t.line_no);
                            panic!();
                        },
                    }
                    self.step();
                    continue;
                }
                self.step();
                continue;
            }
            else 
            {
                break;
            }
        }
        self.current_token = 0;

        let mut hex_code: Vec<u8> = vec![];
        // Second pass : Create the instructions vector
        loop 
        {
            let mut t = self.current();

            if let Some(t) = t
            {
                if t.ttype == TT::INSTRUCTION
                {
                    let mode = self.next_mode();
                    // println!("Instruction: {} Mode: {} ", t.tstring, mode);
                    if t.tstring == "ADC"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::ADC_IMM),
                            Mode::ZP => hex_code.push(Instruction::ADC_ZP),
                            Mode::ZPY => hex_code.push(Instruction::ADC_ZPX),
                            Mode::ABS => hex_code.push(Instruction::ADC_ABS),
                            Mode::ABSX => hex_code.push(Instruction::ADC_ABSX),
                            Mode::ABSY => hex_code.push(Instruction::ADC_ABSY),
                            Mode::INDX => hex_code.push(Instruction::ADC_INDX),
                            Mode::INDY => hex_code.push(Instruction::ADC_INDY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "AND"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::AND_IMM),
                            Mode::ZP => hex_code.push(Instruction::AND_ZP),
                            Mode::ZPX => hex_code.push(Instruction::AND_ZPX),
                            Mode::ABS => hex_code.push(Instruction::AND_ABS),
                            Mode::ABSX => hex_code.push(Instruction::AND_ABSX),
                            Mode::ABSY => hex_code.push(Instruction::AND_ABSY),
                            Mode::INDX => hex_code.push(Instruction::AND_INDX),
                            Mode::INDY => hex_code.push(Instruction::AND_INDY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "ASL"
                    {
                        match mode
                        {
                            Mode::ACC => hex_code.push(Instruction::ASL_ACC),
                            Mode::ZP => hex_code.push(Instruction::ASL_ZP),
                            Mode::ZPX => hex_code.push(Instruction::ASL_ZPX),
                            Mode::ABS => hex_code.push(Instruction::ASL_ABS),
                            Mode::ABSX => hex_code.push(Instruction::ASL_ABSX),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BCC"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BCC_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BCS"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BCS_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BEQ"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BEQ_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BIT"
                    {
                        match mode
                        {
                            Mode::ZP => hex_code.push(Instruction::BIT_ZP),
                            Mode::ABS => hex_code.push(Instruction::BIT_ABS),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BMI"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BMI_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BNE"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BNE_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BPL"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BPL_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BRK"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::BRK_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BVC"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BVC_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "BVS"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BVS_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "CLC"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::CLC_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "CLD"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::CLD_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "CLI"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::CLI_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "CLV"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::CLV_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "CMP"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::CMP_IMM),
                            Mode::ZP => hex_code.push(Instruction::CMP_ZP),
                            Mode::ZPX => hex_code.push(Instruction::CMP_ZPX),
                            Mode::ABS => hex_code.push(Instruction::CMP_ABS),
                            Mode::ABSX => hex_code.push(Instruction::CMP_ABSX),
                            Mode::ABSY => hex_code.push(Instruction::CMP_ABSY),
                            Mode::INDX => hex_code.push(Instruction::CMP_INDX),
                            Mode::INDY => hex_code.push(Instruction::CMP_INDY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "CPX"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::CPX_IMM),
                            Mode::ZP => hex_code.push(Instruction::CPX_ZP),
                            Mode::ABS => hex_code.push(Instruction::CPX_ABS),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "DEC"
                    {
                        match mode
                        {
                            Mode::ZP => hex_code.push(Instruction::DEC_ZP),
                            Mode::ZPX => hex_code.push(Instruction::DEC_ZPX),
                            Mode::ABS => hex_code.push(Instruction::DEC_ABS),
                            Mode::ABSX => hex_code.push(Instruction::DEC_ABSX),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "DEX"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::DEX_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "DEY"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::DEY_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "EOR"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::EOR_IMM),
                            Mode::ZP => hex_code.push(Instruction::EOR_ZP),
                            Mode::ZPX => hex_code.push(Instruction::EOR_ZPX),
                            Mode::ABS => hex_code.push(Instruction::EOR_ABS),
                            Mode::ABSX => hex_code.push(Instruction::EOR_ABSX),
                            Mode::ABSY => hex_code.push(Instruction::EOR_ABSY),
                            Mode::INDX => hex_code.push(Instruction::EOR_INDX),
                            Mode::INDY => hex_code.push(Instruction::EOR_INDY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "INC"
                    {
                        match mode
                        {
                            Mode::ZP => hex_code.push(Instruction::INC_ZP),
                            Mode::ZPX => hex_code.push(Instruction::INC_ZPX),
                            Mode::ABS => hex_code.push(Instruction::INC_ABS),
                            Mode::ABSX => hex_code.push(Instruction::INC_ABSX),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "INX"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::INX_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "INY"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::INY_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "JMP"
                    {
                        match mode
                        {
                            Mode::ABS => hex_code.push(Instruction::JMP_ABS),
                            Mode::IND => hex_code.push(Instruction::JMP_IND),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "JSR"
                    {
                        match mode
                        {
                            Mode::ABS => hex_code.push(Instruction::JSR_ABS),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "LDA"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::LDA_IMM),
                            Mode::ZP => hex_code.push(Instruction::LDA_ZP),
                            Mode::ZPX => hex_code.push(Instruction::LDA_ZPX),
                            Mode::ABS => hex_code.push(Instruction::LDA_ABS),
                            Mode::ABSX => hex_code.push(Instruction::LDA_ABSX),
                            Mode::ABSY => hex_code.push(Instruction::LDA_ABSY),
                            Mode::INDX => hex_code.push(Instruction::LDA_INDX),
                            Mode::INDY => hex_code.push(Instruction::LDA_INDY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "LDX"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::LDX_IMM),
                            Mode::ZP => hex_code.push(Instruction::LDX_ZP),
                            Mode::ZPY => hex_code.push(Instruction::LDX_ZPY),
                            Mode::ABS => hex_code.push(Instruction::LDX_ABS),
                            Mode::ABSY => hex_code.push(Instruction::LDX_ABSY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "LDY"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::LDY_IMM),
                            Mode::ZP => hex_code.push(Instruction::LDY_ZP),
                            Mode::ZPX => hex_code.push(Instruction::LDY_ZPX),
                            Mode::ABS => hex_code.push(Instruction::LDY_ABS),
                            Mode::ABSX => hex_code.push(Instruction::LDY_ABSX),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "LSR"
                    {
                        match mode
                        {
                            Mode::ACC => hex_code.push(Instruction::LSR_ACC),
                            Mode::ZP => hex_code.push(Instruction::LSR_ZP),
                            Mode::ZPX => hex_code.push(Instruction::LSR_ZPX),
                            Mode::ABS => hex_code.push(Instruction::LSR_ABS),
                            Mode::ABSX => hex_code.push(Instruction::LSR_ABSX),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "NOP"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::NOP_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "ORA"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::ORA_IMM),
                            Mode::ZP => hex_code.push(Instruction::ORA_ZP),
                            Mode::ZPX => hex_code.push(Instruction::ORA_ZPX),
                            Mode::ABS => hex_code.push(Instruction::ORA_ABS),
                            Mode::ABSX => hex_code.push(Instruction::ORA_ABSX),
                            Mode::ABSY => hex_code.push(Instruction::ORA_ABSY),
                            Mode::INDX => hex_code.push(Instruction::ORA_INDX),
                            Mode::INDY => hex_code.push(Instruction::ORA_INDY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "PHA"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::PHA_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "PHP"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::PHP_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "PLA"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::PLA_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "PLP"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::PLP_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "ROL"
                    {
                        match mode
                        {
                            Mode::ACC => hex_code.push(Instruction::ROL_ACC),
                            Mode::ZP => hex_code.push(Instruction::ROL_ZP),
                            Mode::ZPX => hex_code.push(Instruction::ROL_ZPX),
                            Mode::ABS => hex_code.push(Instruction::ROL_ABS),
                            Mode::ABSX => hex_code.push(Instruction::ROL_ABSX),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "ROR"
                    {
                        match mode
                        {
                            Mode::ACC => hex_code.push(Instruction::ROR_ACC),
                            Mode::ZP => hex_code.push(Instruction::ROR_ZP),
                            Mode::ZPX => hex_code.push(Instruction::ROR_ZPX),
                            Mode::ABS => hex_code.push(Instruction::ROR_ABS),
                            Mode::ABSX => hex_code.push(Instruction::ROR_ABSX),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "RTI"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::RTI_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "RTS"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::RTS_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "SBC"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::SBC_IMM),
                            Mode::ZP => hex_code.push(Instruction::SBC_ZP),
                            Mode::ZPX => hex_code.push(Instruction::SBC_ZPX),
                            Mode::ABS => hex_code.push(Instruction::SBC_ABS),
                            Mode::ABSX => hex_code.push(Instruction::SBC_ABSX),
                            Mode::ABSY => hex_code.push(Instruction::SBC_ABSY),
                            Mode::INDX => hex_code.push(Instruction::SBC_INDX),
                            Mode::INDY => hex_code.push(Instruction::SBC_INDY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "SEC"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::SEC_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "SED"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::SED_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "SEI"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::SEI_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "STA"
                    {
                        match mode
                        {
                            Mode::ZP => hex_code.push(Instruction::STA_ZP),
                            Mode::ZPX => hex_code.push(Instruction::STA_ZPX),
                            Mode::ABS => hex_code.push(Instruction::STA_ABS),
                            Mode::ABSX => hex_code.push(Instruction::STA_ABSX),
                            Mode::ABSY => hex_code.push(Instruction::STA_ABSY),
                            Mode::INDX => hex_code.push(Instruction::STA_INDX),
                            Mode::INDY => hex_code.push(Instruction::STA_INDY),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "STX"
                    {
                        match mode
                        {
                            Mode::ZP => hex_code.push(Instruction::STX_ZP),
                            Mode::ZPY => hex_code.push(Instruction::STX_ZPY),
                            Mode::ABS => hex_code.push(Instruction::STX_ABS),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "STY"
                    {
                        match mode
                        {
                            Mode::ZP => hex_code.push(Instruction::STY_ZP),
                            Mode::ZPX => hex_code.push(Instruction::STY_ZPX),
                            Mode::ABS => hex_code.push(Instruction::STY_ABS),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "TAX"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TAX_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "TAY"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TAY_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "TSX"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TSX_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "TXA"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TXA_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "TXS"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TXS_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                    if t.tstring == "TYA"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TYA_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                    }

                    // Get operand
                    if mode != Mode::IMP
                    {
                        match mode 
                        {
                            Mode::ABS | Mode::ABSX | Mode::ABSY | Mode::IND => 
                            {
                                let operand = self.get_operand_u16();
                                let hsb:u8 = (operand << 8) as u8;
                                let lsb:u8 = operand as u8;
                                hex_code.push(hsb);
                                hex_code.push(lsb);
                            },
                            Mode::IMM | Mode::INDX | Mode::INDY =>
                            {
                                self.step();
                                let operand = self.get_operand_u8();
                                hex_code.push(operand);
                            },
                            Mode::REL | Mode::ZP | Mode::ZPX | Mode::ZPY =>
                            {
                                let operand = self.get_operand_u8();
                                hex_code.push(operand);
                            },
                            _ => panic!("Unknown addressing mode"),
                        }
                    }
                }

                self.step();
            }
            else
            {
                break;
            }
        }

        hex_code

    }

    pub fn tokenize(&mut self, code: String)
    {
        let mut line_no = 1;

        // TODO : Figure this out
        // unknown_chars.len() had some borrow/copy problems that I don't know how to fix
        // so instead I used another variable to track the size of the unkown characters

        let mut unknown_chars_size = 0;
        let mut unknown_chars : Vec<char> = Vec::new();//String::new() ;
        let mut is_comment : bool = false;

        for c in code.chars()
        {
            if !is_comment
            {
                if c == ';'
                {
                    is_comment = true;
                    continue;
                }

                if c == ' '
                {
                    // If unknown characters is not empty, then add the token
                    // This check should appear after every known token
                    if unknown_chars_size > 0
                    {
                        let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(t);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let pt = self.previous();

                    if let Some(pt) = pt
                    {
                        if pt.ttype == TT::WHITE
                        {
                            continue;
                        }
                    }

                    // THINKABOUT : Do we really need to record whitespaces?
                    //
                    // let t = Token { ttype: TT::WHITE, tstring:" ".to_string(), line_no:line_no};
                    // self.tokens.push(t);
                    // self.step();

                    continue;
                }

                if c == 'X' || c == 'x'
                {
                    let pt = self.previous();

                    if let Some(pt) = pt
                    {
                        if pt.ttype == TT::WHITE
                            || pt.ttype == TT::COMMA
                            {
                                let t = Token { ttype: TT::REGX, tstring:"X".to_string(), line_no:line_no};
                                self.tokens.push(t);
                                self.step();
                                continue;
                            }
                    }
                }

                if c == 'Y' || c == 'y'
                {
                    let pt = self.previous();

                    if let Some(pt) = pt
                    {
                        if pt.ttype == TT::WHITE
                            || pt.ttype == TT::COMMA
                            {
                                let t = Token { ttype: TT::REGY, tstring:"Y".to_string(), line_no:line_no};
                                self.tokens.push(t);
                                self.step();
                                continue;
                            }
                    }
                }

                if c == '#'
                {
                    if unknown_chars_size > 0
                    {
                        let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(t);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let t = Token { ttype: TT::HASH, tstring:"#".to_string(), line_no:line_no};
                    self.tokens.push(t);
                    self.step();
                    continue;
                }

                if c == ':'
                {
                    if unknown_chars_size > 0
                    {
                        let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(t);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let t = Token { ttype: TT::COLON, tstring:":".to_string(), line_no:line_no};
                    self.tokens.push(t);
                    self.step();
                    continue;
                }

                if c == '$'
                {
                    if unknown_chars_size > 0
                    {
                        let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(t);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let t = Token { ttype: TT::DOLLAR, tstring:"$".to_string(), line_no:line_no};
                    self.tokens.push(t);
                    self.step();
                    continue;
                }

                if c == '%'
                {
                    if unknown_chars_size > 0
                    {
                        let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(t);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let t = Token { ttype: TT::PERCENT, tstring:"%".to_string(), line_no:line_no};
                    self.tokens.push(t);
                    self.step();
                    continue;
                }

                if c == ','
                {
                    if unknown_chars_size > 0
                    {
                        let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(t);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let t = Token { ttype: TT::COMMA, tstring:",".to_string(), line_no:line_no};
                    self.tokens.push(t);
                    self.step();
                    continue;
                }

                if c == '('
                {
                    if unknown_chars_size > 0
                    {
                        let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(t);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let t = Token { ttype: TT::BRACKETOPEN, tstring:"(".to_string(), line_no:line_no};
                    self.tokens.push(t);
                    self.step();
                    continue;
                }

                if c == ')'
                {
                    if unknown_chars_size > 0
                    {
                        let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(t);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let t = Token { ttype: TT::BRACKETCLOSE, tstring:")".to_string(), line_no:line_no};
                    self.tokens.push(t);
                    self.step();
                    continue;
                }

                if c == '\r' { continue; }

                if c == '\n'
                {
                    if unknown_chars_size > 0
                    {
                        let token = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
                        self.tokens.push(token);
                        self.step();
                        unknown_chars.clear();
                        unknown_chars_size = 0;
                    }

                    let pt = self.previous();

                    if let Some(pt) = pt
                    {
                        if pt.ttype == TT::NL
                        {
                            line_no += 1;
                            continue;
                        }
                    }

                    // let t = Token { ttype: TT::NL, tstring:"NL".to_string(), line_no:1};
                    // self.tokens.push(t);
                    // self.step();
                    line_no += 1;

                    continue;
                }

                unknown_chars.push(c);
                unknown_chars_size += 1;

            }
            else 
            {
                if c == '\n'
                {
                    line_no += 1;
                    is_comment = false;
                }
            }
        }
        if unknown_chars_size > 0
        {
            let t = Token { ttype: TT::UNKNOWN, tstring:unknown_chars.iter().collect(), line_no:line_no};
            self.tokens.push(t);
            self.step();
            unknown_chars.clear();
        }


        let t = Token { ttype: TT::EOF, tstring:"".to_string(), line_no:line_no};
        self.tokens.push(t);
        self.step();
        self.current_token = 0;
    }

    pub fn print_tokens(&self)
    {
        for token in &self.tokens
        {
            println!("#: {} Type: {:?} : {:?}", token.line_no, token.ttype, token.tstring);
        }
    }

    pub fn source_from_tokens(&self)
    {
        let mut current_line = self.tokens.first().unwrap().line_no;
        for t in &self.tokens
        {
            if t.line_no > current_line
            {
                println!();
                current_line = t.line_no;
            }
            print!("{} ", t.tstring);

        }
    }
}
