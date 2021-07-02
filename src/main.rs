mod isa;
mod lexer;
mod cpu;

use std::io::Write; // bring trait into scope
use std::fs;

fn main() {
    let mut lex = lexer::Lexer {tokens:Vec::new(), current_token:0};

    lex.tokenize("
    start:
     txa
     sta $200, y
     sta $300, y
     sta $400, y
     sta $500, y
     iny
     tya
     cmp 16
     bne do
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
