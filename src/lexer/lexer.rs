

use std::fmt::format;

use crate::token::{token::{Token, TokenType, look_up_ident}};

#[derive(Clone)]
pub struct Lexer {
    input: String,                      
    postion: usize,                     // 所输入字符中的当前位置(指向当前字符)
    read_postion: usize,                // 所输入字符中的当前读取位置(指向当前字符的后一个)
    ch: char,                           // 当前正在查看的字符
}

impl Lexer {

    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: String::from(input),
            postion: 0,
            read_postion: 0,
            ch: ' ',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_postion >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_postion).unwrap();
        }
        self.postion = self.read_postion;
        self.read_postion += 1;
    }

    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();

        let token = match self.ch {
            '\0' => Token { token_type: TokenType::Eof, literal: "".to_string() },
            ';'  => self.new_token(TokenType::Semicolon, self.ch),
            ','  => self.new_token(TokenType::Comma, self.ch),
            '('  => self.new_token(TokenType::LParen, self.ch),
            ')'  => self.new_token(TokenType::RParen, self.ch),
            '{'  => self.new_token(TokenType::LBrace, self.ch),
            '}'  => self.new_token(TokenType::RBrace, self.ch),
            '+'  => self.new_token(TokenType::Plus, self.ch),
            '-'  => self.new_token(TokenType::Minus, self.ch),
            '/'  => self.new_token(TokenType::Slash, self.ch),
            '*'  => self.new_token(TokenType::Asterisk, self.ch),
            '<'  => self.new_token(TokenType::Lt, self.ch),
            '>'  => self.new_token(TokenType::Gt, self.ch),
            '!'  => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let _literal = format!("{}{}", ch, self.ch);        // 所有权没有问题的吗
                    Token { token_type: TokenType::NotEq, literal: _literal }
                } else {
                    self.new_token(TokenType::Bang, self.ch)
                }
            },
            '='  => { 
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let _literal = format!("{}{}", ch, self.ch);        // 所有权没有问题的吗
                    Token { token_type: TokenType::Eq, literal: _literal }
                } else {
                    self.new_token(TokenType::Assign, self.ch)
                }
            },
            _    => { 
                if self.is_letter(self.ch) {
                    let temp = self.read_identifier();
                    return Token { token_type: look_up_ident(temp.clone()), literal: temp };
                } else if self.is_dight(self.ch) {
                    return Token{ token_type:TokenType::Int, literal:self.read_number() }
                } else {
                    self.new_token(TokenType::Illegal, self.ch)
                }
            },
        };
    
        self.read_char(); // 读取下一个字符
        token
    }

    fn new_token(&self, token_type: TokenType, ch: char) -> Token {
        Token {
            token_type: token_type,
            literal: String::from(ch),
        }
    }

    fn is_letter(&self, ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn read_identifier(&mut self) -> String{
        let position = self.postion;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.postion].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let postion = self.postion;
        while self.is_dight(self.ch) {
            self.read_char();
        }
        self.input[postion..self.postion].to_string()
    }

    fn is_dight(&self, ch: char) -> bool {
        ch <= '9' && ch >= '0'
    }

    fn peek_char(&self) -> char {
        if self.read_postion >= self.input.len() {
            return '\0';
        } else {
            return self.input.chars().nth(self.read_postion).unwrap();
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::token::token::TokenType;

    use super::*;

    #[test]
    fn test_next_token1() {
        let input = String::from("=+(){},;");
        let tests = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(tok.token_type, *expected_type, "tests[{}] - tokentype wrong. expected={:?}, got={:?}", i, expected_type, tok.token_type);
            assert_eq!(tok.literal, *expected_literal, "tests[{}] - literal wrong. expected={:?}, got={:?}", i, expected_literal, tok.literal);
        }
    }

    #[test]
    fn test_next_token2() {
        let input = String::from(
            "let five = 5;
             let ten = 10;
             let add = fn(x, y) {
                 x + y;
             };
             let result = add(five, ten);"
        );
        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];
        let mut lexer = Lexer::new(input);

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(tok.token_type, *expected_type, "tests[{}] - tokentype wrong. expected={:?}, got={:?}", i, expected_type, tok.token_type);
            assert_eq!(tok.literal, *expected_literal, "tests[{}] - literal wrong. expected={:?}, got={:?}", i, expected_literal, tok.literal);
        }
    }

    #[test]
    fn test_next_token3() {
        let input = String::from("
            let five = 5; 
            let ten = 10; 
            let add = fn(x, y) { 
                x + y; 
            }; 
            let result = add(five, ten);

            !-/*5; 
            5 < 10 > 5;

            if (5 < 10) { 
                return true; 
            } else { 
                return false; 
            }

            10 == 10; 
            10 != 9;
        ");

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(tok.token_type, *expected_type, "tests[{}] - tokentype wrong. expected={:?}, got={:?}", i, expected_type, tok.token_type);
            assert_eq!(tok.literal, *expected_literal, "tests[{}] - literal wrong. expected={:?}, got={:?}", i, expected_literal, tok.literal);
        }
    }
}