


use crate::{token::token::{Token, TokenType}, 
            lexer::lexer::Lexer, ast::{program::Program, statement::{LetStatement, Statement, Returnstatement}, expression::{Identifier, Expression}}, 
            
};

use super::error::ParserError;

#[derive(Clone)]
struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    
    fn new(l: Lexer) -> Parser {
        let mut p = Parser {l: l, cur_token: Token::default(), peek_token: Token::default() };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parser_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program::default();
        
        while !self.cur_token_is(TokenType::Eof) {
            let stmt = self.parser_statement()?;
            program.statements.push(stmt);
            self.next_token();
        }
        Ok(program)
    }

    fn parser_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cur_token.token_type {
            TokenType::Let      => self.parser_let_statement(),
            TokenType::Return   => self.parser_return_statement(),
            _                   => Err(ParserError::new("expected let or return statement".to_string())),
        }
    }

    fn parser_let_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.cur_token.clone();

        self.expect_peek(TokenType::Ident)?;

        let name = Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());

        self.expect_peek(TokenType::Assign)?;

        // TODO: 跳过对表达式的处理，直到遇见分号
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Ok(Statement::Let(LetStatement::new(token, name, Expression::Ident(Identifier::default()))));
    }

    fn parser_return_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.cur_token.clone();

        self.next_token();

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Return(Returnstatement::new(token)))
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> Result<(), ParserError> {
        if self.peek_token_is(token_type) {
            self.next_token();
            Ok(())
        } else {
            Err(ParserError::new(
                format!(
                    "expected next token to be '{}', but got '{}' instead",
                    token_type, self.peek_token.token_type
                )
            ))
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::Lexer, parser::{parser::Parser, error::{self, ParserError}}, ast::{statement::Statement, program::{self, Program}}};


    fn test_let_statement(stmt: &Statement, expected_identifier: &str) -> bool {
        match stmt {
            Statement::Let(let_stmt) => {
                if let_stmt.token_literal() != "let" {
                    eprintln!("let_stmt.token_literal() not 'let'. got={}", let_stmt.token_literal());
                    return false;
                }
    
                if let_stmt.name.value != expected_identifier {
                    eprintln!("let_stmt.name.value not '{}'. got={}", expected_identifier, let_stmt.name.value);
                    return false;
                }
    
                if let_stmt.name.token_literal() != expected_identifier {
                    eprintln!("let_stmt.name.token_literal() not '{}'. got={}", expected_identifier, let_stmt.name.token_literal());
                    return false;
                }
    
                true
            }
            _ => {
                eprintln!("stmt is not a LetStatement. got={:?}", stmt);
                false
            }
        }
    }

    #[test]
    fn test_parser() -> Result<(), ParserError>{
        let l = Lexer::new(String::from("
            let x = 5;
            let y = 10; 
            let foobar = 838383;
        "));
        let mut p = Parser::new(l);
        let program = p.parser_program()?;
    
        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got={}", program.statements.len());
        }
    
        let tests = vec![
            "x",
            "y",
            "foobar",
        ];

        let mut flag = false;

        let mut errors = String::new();

        for (i, &expected_identifier) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            if !test_let_statement(stmt, expected_identifier) {
                flag = true;
                errors.push_str(&format!("test_let_statement failed for identifier: {}  ", expected_identifier));
            }
        }
        if flag {
            return Err(ParserError::new(errors));
        }
        Ok(())
    }

    #[test]
    fn test_return_statement() -> Result<(), ParserError>{
        let l = Lexer::new(String::from("
            return 5; 
            return 10; 
            return 993 322;
        "));
        let mut p = Parser::new(l);
        let program = p.parser_program()?;
        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got={}", program.statements.len());
        }

        for stmt in program.statements {
            if let Statement::Return(return_stmt) = stmt {
                if return_stmt.token_literal() != "return" {
                    return Err(ParserError::new("return_stmt.token_literal() not 'return'. got=".to_string() + &return_stmt.token_literal()));
                }
            }
        }
        
        Ok(())

    }

}