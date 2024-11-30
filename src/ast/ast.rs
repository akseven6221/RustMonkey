

use crate::token::token::Token;


/** let <标识符> = <表达式>;
 *  let x = 5
 *
 *  
*/ 


pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement {
    fn statement_node(&self);
}

pub trait Expression {
    fn expression_node(&self);
}

pub(crate) struct Program {
    Statements: Vec<Box<dyn Statement>>,             // 表示每个元素都实现了Statement trait
}

struct LetStatement {
    Token: Token,
    name: Identifier,
    value: dyn Expression,
}

struct Identifier {
    Token: Token,
    value: String,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(first_statement) = self.Statements.first() {
            // 使用 `as_ref` 将 `&Box<dyn Statement>` 转换为 `&dyn Statement`
            // 然后调用 `Node` trait 的 `token_literal` 方法
            first_statement.as_ref().token_literal()
        } else {
            String::from("")
        }
    }
}

impl Statement for LetStatement {
    
    fn statement_node(&self) {
        // do nothing
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.Token.literal.clone()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.Token.literal.clone()
    }
}