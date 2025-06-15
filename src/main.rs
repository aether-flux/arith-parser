use utils::{lexer::lexer, parser::parseExp};
use std::io::{self, Write};


mod utils;

fn main() {
    print!("Enter expression: ");
    io::stdout().flush();

    let mut e = String::new();
    io::stdin().read_line(&mut e);

    lexer(&e);
}
