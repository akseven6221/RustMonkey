use std::env;
use std::io;
use std::process;


mod token;
mod lexer;
mod repl;
mod ast;
mod parser;

use crate::repl::repl::start;

fn main() {
    // let username = match env::var("USER") {
    //     Ok(name) => name,
    //     Err(_) => {
    //         eprintln!("Failed to get current user");
    //         process::exit(1);
    //     }
    // };

    let username = "alioth";
    println!("Hello {}! This is the Monkey programming language!", username);
    println!("Feel free to type in commands");

    // 启动 REPL
    start(&mut io::stdin().lock(), &mut io::stdout());
}
