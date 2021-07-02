// TODO : Is this the correct way of including the file?
use crate::isa::*;
use std::collections::HashMap;

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
    REGX,
    REGY,
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

    pub fn step(&mut self)
    {
        self.current_token += 1;
    }

    pub fn stepx(&mut self, steps: usize)
    {
        self.current_token += steps;
    }

    pub fn parse(&mut self) -> Vec<u8>
    {
        // TODO : Add lowecase versions of the commands
        let mut instruction_strings = vec!["ADC", "AND", "ASL", "BCC", "BCS", "BEQ", "BIT", "BMI", "BNE", "BPL", "BRK", "BVC", "CLC", "CLD", "CLI", "CLV", "CMP", "CPX", "CPY", "DEC", "DEX", "DEY", "EOR", "INC", "INX", "INY", "JMP", "JSR", "LDA", "LDX", "LDY", "LSR", "NOP", "ORA", "PHA", "PHP", "PLA", "PLP", "ROL", "ROR", "RTI", "RTS", "SBC", "SEC", "SED", "SEI", "STA", "STX", "STY", "TAX", "TAY", "TSX", "TXA", "TXS", "TYA",];
        instruction_strings.sort_unstable();

        // First pass : Update unknown tokens and read labels
        let mut labels = HashMap::new();
        let mut optional = self.current();
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
                    }
                    // If label
                    else if let Some(nt) = self.next()
                    {
                        if nt.ttype == TT::COLON
                        {
                            self.tokens[self.current_token].ttype = TT::LABEL;
                            labels.insert(self.tokens[self.current_token].tstring.clone(), 1);
                        }
                        else
                        {
                            let mut valid = true;
                            for tc in t.tstring.chars()
                            {
                                if tc < '0' ||  (tc > '9' && tc < 'A') || tc > 'F'
                                {
                                    valid = false;
                                }
                            }

                            if valid
                            {
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
                            if tc < '0' ||  (tc > '9' && tc < 'A') || tc > 'F'
                            {
                                valid = false;
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

        self.current_token = 0;

        println!("Labels found : {}", labels.len());

        let mut errors: Vec<String> = vec![];
        let mut hex_code: Vec<u8> = vec![];
        // Second pass : Create the instructions vector
        loop 
        {
            let mut op_code: u8 = 0;
            let t = self.current();

            let mode = 0;
            if let Some(t) = t
            {
                if t.ttype == TT::INSTRUCTION
                {
                    println!("Instruction {}", t.tstring);
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
                        continue;
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
                        continue;
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
                        continue;
                    }
                    if t.tstring == "BCC"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BCC_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "BCS"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BCS_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "BEQ"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BEQ_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "BIT"
                    {
                        match mode
                        {
                            Mode::ZP => hex_code.push(Instruction::BIT_ZP),
                            Mode::ABS => hex_code.push(Instruction::BIT_ABS),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "BMI"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BMI_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "BNE"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BNE_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "BPL"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BPL_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "BRK"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::BRK_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "BVC"
                    {
                        match mode
                        {
                            Mode::REL => hex_code.push(Instruction::BVC_REL),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "CLC"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::CLC_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "CLD"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::CLD_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "CLI"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::CLI_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "CLV"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::CLV_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
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
                        continue;
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
                        continue;
                    }
                    if t.tstring == "CPY"
                    {
                        match mode
                        {
                            Mode::IMM => hex_code.push(Instruction::CPY_IMM),
                            Mode::ZP => hex_code.push(Instruction::CPY_ZP),
                            Mode::ABS => hex_code.push(Instruction::CPY_ABS),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
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
                        continue;
                    }
                    if t.tstring == "DEX"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::DEX_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "DEY"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::DEY_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
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
                        continue;
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
                        continue;
                    }
                    if t.tstring == "INX"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::INX_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "INY"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::INY_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "JMP"
                    {
                        match mode
                        {
                            Mode::ABS => hex_code.push(Instruction::JMP_ABS),
                            Mode::IND => hex_code.push(Instruction::JMP_IND),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "JSR"
                    {
                        match mode
                        {
                            Mode::ABS => hex_code.push(Instruction::JSR_ABS),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
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
                        continue;
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
                        continue;
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
                        continue;
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
                        continue;
                    }
                    if t.tstring == "NOP"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::NOP_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
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
                        continue;
                    }
                    if t.tstring == "PHA"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::PHA_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "PHP"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::PHP_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "PLA"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::PLA_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "PLP"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::PLP_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
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
                        continue;
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
                        continue;
                    }
                    if t.tstring == "RTI"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::RTI_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "RTS"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::RTS_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
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
                        continue;
                    }
                    if t.tstring == "SEC"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::SEC_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "SED"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::SED_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "SEI"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::SEI_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
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
                        continue;
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
                        continue;
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
                        continue;
                    }
                    if t.tstring == "TAX"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TAX_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "TAY"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TAY_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "TSX"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TSX_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "TXA"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TXA_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "TXS"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TXS_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                    if t.tstring == "TYA"
                    {
                        match mode
                        {
                            Mode::IMP => hex_code.push(Instruction::TYA_IMP),
                            _ => panic!("Unknown addressing mode"),
                        }
                        continue;
                    }
                }

                self.step();
            }
            else
            {
                break;
            }
        }

        // Third pass
        loop 
        {
            let t = self.current();

            if let Some(t) = t
            {

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

        for c in code.chars()
        {
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

            if c == 'X'
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

            if c == 'Y'
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

            if c == '\n'
            {
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
                    if pt.ttype == TT::NL
                    {
                        line_no += 1;
                        continue;
                    }
                }

                let t = Token { ttype: TT::NL, tstring:"NL".to_string(), line_no:1};
                self.tokens.push(t);
                self.step();
                line_no += 1;

                continue;
            }

            unknown_chars.push(c);
            unknown_chars_size += 1;
        }

        self.current_token = 0;

    }

    pub fn print_tokens(&self)
    {
        for token in &self.tokens
        {
            println!("Type: {:?}, String : {}, Line No : {}", token.ttype, token.tstring, token.line_no);
        }
    }
}
