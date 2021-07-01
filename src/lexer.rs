// TODO : Is this the correct way of including the file?
use crate::isa;
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

    pub fn parse(&mut self) -> Vec<u32>
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

        println!("Labels found : {}", labels.len());

        let mut errors: Vec<String> = vec![];
        // Second pass : Create the instructions vector
        loop 
        {
                let t = self.current();

                if let Some(t) = t
                {
                    if t.ttype == TT::INSTRUCTION
                    {
                        if t.tstring == "ADC"
                        {

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


        let v: Vec<u32> = Vec::new();
        v
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
