mod isa;
mod lexer;
mod cpu;

use std::collections::HashMap;
// use std::io::Write;
use std::env;
use std::fs;

fn main() {
    let mut lex = lexer::Lexer 
    {
        tokens:Vec::new(),
        current_token:0,
        labels:HashMap::new()
    };

    // Read file from disk
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        // TODO(James) : Better arg handling/Errors?
        println!("Example : e6502c source.asm out.bin");
        panic!();
    }

    let source_file_path = &args[1];
    let out_file_path = &args[2];

    let source = fs::read_to_string(source_file_path).expect("Error occured while reading file");
    // println!("{}",source);

    lex.tokenize(source);
    // lex.print_tokens();
    let out = lex.parse();
    // lex.print_tokens();
    // println!("{:?}", out);
    // lex.source_from_tokens();

    fs::write(out_file_path, out).expect("Unknown error while writinng to bin");

}
