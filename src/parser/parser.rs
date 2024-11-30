


use crate::{token::token::Token, 
            lexer::lexer::Lexer, 
            ast::ast::Program
};

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

    fn parser_program(&self) -> Program {
        Program{ Statements: todo!() }
    }

}


#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::Lexer, parser::parser::Parser};

    // #[test]
    // fn test_parser() {
    //     let l = Lexer::new(String::from("
    //         let x = 5;
    //         let y = 10; 
    //         let foobar = 838383;
    //     "));
    //     let mut p = Parser::new(l);
    //     let program = p.parser_program();

    //     if program.is_none() {
    //         panic!("parse_program() returned None");
    //     }
    
    //     let program = program.unwrap();
    
    //     if program.statements.len() != 3 {
    //         panic!("program.statements does not contain 3 statements. got={}", program.statements.len());
    //     }
    
    //     let tests = vec![
    //         "x",
    //         "y",
    //         "foobar",
    //     ];
    
    //     for (i, &expected_identifier) in tests.iter().enumerate() {
    //         let stmt = &program.statements[i];
    //         if !test_let_statement(stmt, expected_identifier) {
    //             return;
    //         }
    //     }
    // }
    
    // #[test]
    // fn test_let_statement(stmt: &Statement, expected_identifier: &str) -> bool {
    //     match stmt {
    //         Statement::Let(let_stmt) => {
    //             if let_stmt.token_literal() != "let" {
    //                 eprintln!("let_stmt.token_literal() not 'let'. got={}", let_stmt.token_literal());
    //                 return false;
    //             }
    
    //             if let_stmt.name.value != expected_identifier {
    //                 eprintln!("let_stmt.name.value not '{}'. got={}", expected_identifier, let_stmt.name.value);
    //                 return false;
    //             }
    
    //             if let_stmt.name.token_literal() != expected_identifier {
    //                 eprintln!("let_stmt.name.token_literal() not '{}'. got={}", expected_identifier, let_stmt.name.token_literal());
    //                 return false;
    //             }
    
    //             true
    //         }
    //         _ => {
    //             eprintln!("stmt is not a LetStatement. got={:?}", stmt);
    //             false
    //         }
    //     }
    // }

}