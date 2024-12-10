
use crate::token::{token::Token, self};
use std::fmt::{self, Formatter, Display};


#[derive(Debug)]
pub enum Expression {
    Ident(Identifier),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Expression::Ident(i) => write!(f, "{}", i),
            // _ => write!(f, ""),
        }
    }

}

impl Expression {
    fn token_literal(&self) -> &str {
        match self {
            Expression::Ident(i) => &i.token.literal,
        }
    }


}

#[derive(Default, Debug)]
pub struct Identifier {
    token: Token,
    pub value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.value)
    }
}

impl Identifier {
    pub fn new (t: Token, v: String) -> Self {
        Self {
            token: t,
            value: v,
        }
    }

    fn identifier_node(&self) {
        // do nothing
    }

    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
    
}