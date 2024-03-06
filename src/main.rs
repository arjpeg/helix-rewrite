use helix::lexer::Lexer;

fn main() {
    let lexer = Lexer::new("3");
    let tokens = lexer.tokenize();

    dbg!(tokens);
}
