mod lexer;

fn main() {
    let mut lex = lexer::Lexer {tokens:Vec::new(), current_token:0};

    lex.tokenize("LDA #$01\n\
    STA $0200\n\
    LDA #$05\n\
    STA $0201\n\
    LDA #$08\n\
    labeltest:#$08\n\
    STA $0202\n".to_string());
    lex.print_tokens();
}
