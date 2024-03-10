use helix::{error::print_error, lexer::Lexer};

fn main() {
    let source = "1.23\n12..30 2.22";

    let lexer = Lexer::new(source, "stdin");
    let tokens = lexer.tokenize();

    print_error(tokens.unwrap_err(), source);
}
