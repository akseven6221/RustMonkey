
use std::fmt::{Display, Formatter};

use crate::token::token::Token;
use crate::ast::expression::Identifier;

use super::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(Returnstatement)
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Let(let_statement) => write!(f, "{}", let_statement),
            Statement::Return(return_statement) => write!(f, "{}", return_statement),
        }
    }
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(let_statement) => &let_statement.token.literal,
            Statement::Return(return_statement) => &return_statement.Token.literal,
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
    token: Token,
    pub name: Identifier,
    value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

impl LetStatement {
    fn statement_node(&self) {
        // do nothing
    }

    pub fn new(token: Token, name: Identifier, exp: Expression) -> LetStatement {
        LetStatement {token: token, name: name, value: exp}
    }

    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

#[derive(Debug)]
pub struct Returnstatement {
    pub Token: Token,
    // pub value: Expression,

}

impl Display for Returnstatement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "return;")
    }
}

impl Returnstatement {
    pub fn new (t: Token) -> Self {
        Self {
            Token: t,
        }
    }

    pub fn token_literal(&self) -> &str {
        &self.Token.literal
    }
}
