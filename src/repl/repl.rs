

use crate::lexer::lexer::{self, Lexer};
use crate::token::token::TokenType;

use std::io::{self, Write};
use std::io::BufRead;
use std::io::BufReader;

const PROMPT: &str = ">> ";

pub fn start<R: io::Read, W: io::Write>(input: R, output: &mut W) {
    let mut scanner = io::BufReader::new(input);
    let mut line = String::new();

    loop {
        write!(output, ">> ").unwrap();
        output.flush().unwrap();

        line.clear();
        match scanner.read_line(&mut line) {
            Ok(0) => return, // 读取到EOF
            Ok(_) => {
                let mut lexer = Lexer::new(line.trim().to_string());
                loop {
                    let tok = lexer.next_token();
                    writeln!(output, "{:?}", tok).unwrap();
                    if tok.token_type == TokenType::Eof {
                        break;
                    }
                }
            }
            Err(_) => return,
        }
    }
}