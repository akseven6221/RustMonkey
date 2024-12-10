

pub(crate) mod program;
pub(crate) mod expression;
pub(crate) mod statement;

#[cfg(test)]

mod tests {
    use crate::{ast::{statement::Statement, expression::{Identifier, Expression}}, token::token::{Token, TokenType}};
    use super::{statement::LetStatement, program::Program};


    #[test]
    fn test_fmt() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement::new(
                Token::new(TokenType::Let, String::from("let")),
                Identifier::new(
                    Token::new(TokenType::Ident, String::from("myVar")),
                    String::from("myVar"),
                ),
                Expression::Ident(Identifier::new(
                    Token::new(TokenType::Ident, String::from("anotherVar")),
                    String::from("anotherVar"),
                )),
            ))],
        };
        let program = format!("{}", program);
        assert_eq!(program, "let myVar = anotherVar;");
    }
}