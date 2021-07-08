mod isa;
mod lexer;
mod cpu;

use std::collections::HashMap;
use std::env;
// use std::io::Write;
use std::fs;

fn main() {
    let mut lex = lexer::Lexer 
    {
        tokens:Vec::new(),
        current_token:0,
        labels:HashMap::new()
    };

    lex.tokenize("
    start:
     jmp start
     bne start
     cmp 16
     txa
     sta $200, X
     sta $300, y
     sta $400, y
     sta $500, y
     iny
     tya
     cmp 16
     iny
     jmp start
    do:
     iny
     iny
     iny
     iny
    jmp start".to_string().to_uppercase());
    let out = lex.parse();
    println!("{:?}", out);

    fs::write("out.bin", out).expect("Unknown error while writinng to bin");
}
