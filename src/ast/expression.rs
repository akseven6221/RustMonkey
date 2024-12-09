

use crate::token::token::Token;


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
    Token: Token,
    pub value: String,
}

impl Identifier {
    fn identifier_node(&self) {
        // do nothing
    }

    pub fn token_literal(&self) -> &str {
        &self.Token.literal
    }
    
}