
use crate::token::token::Token;
use crate::ast::expression::Identifier;

use super::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(Returnstatement)
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(let_statement) => &let_statement.Token.literal,
            // Statement::Return(returnstatement) => &returnstatement..Token.literal,
            _ => "",
        }
    }
}


/** 
 * 一个用于标识符Identifier
 *  一个用于 let 语句中产生值的表达式Expression
 *  一个用于词法单元Token
 * */
#[derive(Debug)]
pub struct LetStatement {
    Token: Token,
    pub name: Identifier,
    // value: Expression,
}

impl LetStatement {
    fn statement_node(&self) {
        // do nothing
    }

    pub fn new(token: Token, name: Identifier) -> LetStatement {
        LetStatement {Token: token, name: name}
    }

    pub fn token_literal(&self) -> &str {
        &self.Token.literal
    }
}

#[derive(Debug)]
pub struct Returnstatement {
    // pub value: Box<Expression>
}

