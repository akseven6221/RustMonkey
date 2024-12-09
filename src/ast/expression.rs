

use crate::token::{token::Token, self};


pub enum Expression {
    Ident(Identifier),
}

impl Expression {
    // fn new() -> Identifier {
    //     Identifier {Token: , value: "", }
    // }
}

#[derive(Default, Debug)]
pub struct Identifier {
    token: Token,
    pub value: String,
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