
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

    fn expression() {
        equality()
    }

    fn equality() {
        unimplemented!()
    }
}
