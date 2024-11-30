

use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    Eof,
    // 标识符+字面量
    Ident, // add, foobar, x, y, ...
    Int,   // 1343456
    // 运算符
    Assign, // =
    Plus,   // +
    // 分隔符
    Comma,      // ,
    Semicolon,  // ;
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    // 关键字
    Function, // FUNCTION
    Let,      // LET

    Minus,
    Bang,
    Asterisk,       //"*"
    Slash,          //"/"
    Lt,             //"<"
    Gt,             //">"
    Eq,
    NotEq,
    String,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("fn".to_string(), TokenType::Function);
    map.insert("let".to_string(), TokenType::Let);
    map.insert("true".to_string(), TokenType::True);
    map.insert("false".to_string(), TokenType::False);
    map.insert("if".to_string(), TokenType::If);
    map.insert("else".to_string(), TokenType::Else);
    map.insert("return".to_string(), TokenType::Return);
    map
});

pub fn look_up_ident(ident: String) -> TokenType {
    if let Some(tok) = KEYWORDS.get(&ident) {        
        return *tok;
    }
    TokenType::Ident
}

impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: TokenType::Illegal,
            literal: String::from(""),
        }
    }
}