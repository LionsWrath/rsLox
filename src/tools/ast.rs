use crate::token::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    LITERAL(Token),
    UNARY(
        Token,
        Box<Expr>
    ),
    BINARY(
        Box<Expr>,
        Token,
        Box<Expr>,
    ),
    GROUPING(Box<Expr>),
}
