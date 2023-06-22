use std::io::{self, Write};

use crate::{lexer::Lexer, token::TokenType};

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
            let mut lexer = Lexer::new(line.unwrap()); //Remove unwrap
            loop {
                let token = lexer.next_token();
                match token.token_type {
                    TokenType::EOF => {
                        println!("{:?}", token);
                        break;
                    }
                    _ => {
                        println!("{:?}", token);
                    }
                }
            }

            break;
        }
    }
    //}
}
