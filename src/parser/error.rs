

#[derive(Default, Debug)]
pub struct ParserError {
    pub msg: String,
    // pub pos: TokenPosition,
}

impl ParserError {
    pub fn new(msg: String) -> ParserError {
        ParserError { msg }
    }
}
