
pub struct Parser {
    tokens: Vec<Tokens>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) {
        Parser {
            tokens
        }
    }
}
