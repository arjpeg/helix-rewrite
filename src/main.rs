use helix::{error::print_error, prelude::*};

fn main() {
    let source = "1..23\n12..30 2.22\n333..2";

    let tokens: Vec<_> = Lexer::new(source, "stdin").into_iter().collect();

    // print out all errors
    if tokens.iter().any(Result::is_err) {
        for token in &tokens {
            let Err(error) = token else {
                continue;
            };

            print_error(error, source);
        }

        std::process::exit(1);
    }

    dbg!(tokens);
    // print_error(tokens.unwrap_err(), source);
}
