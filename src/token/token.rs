

use std::{collections::HashMap, fmt::{Display, Formatter}};
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

    EmptyLine,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TokenType::Illegal => write!(f, "Illegal"),
            TokenType::Eof => write!(f, "Eof"),

            // Identifiers + literals
            TokenType::Ident => write!(f, "Ident"),
            TokenType::Int => write!(f, "int"),
            TokenType::String => write!(f, "string"),

            // Operators
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),

            // TokenType::And => write!(f, "&&"),
            // TokenType::Or => write!(f, "||"),
            TokenType::Lt => write!(f, "<"),
            TokenType::Gt => write!(f, ">"),
            // TokenType::LtEq => write!(f, "<="),
            // TokenType::GtEq => write!(f, ">="),

            TokenType::Eq => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),

            TokenType::Comma => write!(f, ","),
            // TokenType::Colon => write!(f, ":"),
            TokenType::Semicolon => write!(f, ";"),

            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            // TokenType::LBracket => write!(f, "["),
            // TokenType::RBracket => write!(f, "]"),

            // Keywords
            TokenType::Function => write!(f, "fn"),
            TokenType::Let => write!(f, "let"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Return => write!(f, "return"),

            // Other
            TokenType::EmptyLine => writeln!(f),
        }
    }
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