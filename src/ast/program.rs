use super::statement::Statement;


#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {


    fn token_literal (&self) -> &str {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }

}