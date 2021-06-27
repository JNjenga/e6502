#[allow(dead_code)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub enum TokenType
{
    WHITE,
    NL,
    EOF,
    NUMBER,
    HASH,
    DOLLAR,
    PERCENT,
    COLON,
    COMMA,
    INSTRUCTION,
    OPERAND,
    UNKNOWN,

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

    pub fn current() -> Token 
    {
        let tstring: String = "Hello".to_string();
        Token { ttype: TT::NL, tstring:tstring, line_no:1}
    }

    pub fn next() -> Token 
    {
        let tstring: String = "Hello".to_string();
        Token { ttype: TT::NL, tstring:tstring, line_no:1}
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
        let v: Vec<u32> = Vec::new();
        v
    }

    pub fn tokenize(&mut self, code: String)
    {
        let mut line_no = 1;
        let mut unknown_chars_size = 0;
        // let mut unknown_string : String = "".to_string();//String::new() ;
        let mut unknown_chars : Vec<char> = Vec::new();//String::new() ;

        for c in code.chars()
        {
            if c == ' '
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
                    if pt.ttype == TT::WHITE
                    {
                        continue;
                    }
                }

                let t = Token { ttype: TT::WHITE, tstring:" ".to_string(), line_no:line_no};

                self.tokens.push(t);
                self.step();

                continue;
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
