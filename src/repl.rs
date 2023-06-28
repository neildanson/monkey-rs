use std::io::{self, Write};

use crate::{
    lexer::{self, Lexer},
    token::Token,
};

const PROMPT: &str = ">> ";
fn print_and_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

pub fn start() {
    println!("Welcome to Monkeylang");
    println!("#####################");
    println!("");
    loop {
        print_and_flush(PROMPT);
        for line in std::io::stdin().lines().take(1) {
            line.iter().for_each(|line| {
                let lexer = Lexer::new(line);
                for token in lexer {
                    match token {
                        Token::EOF => {
                            println!("{:?}", token);
                        }
                        _ => {
                            println!("{:?}", token);
                        }
                    }
                }
            });

            break;
        }
    }
    //}
}
