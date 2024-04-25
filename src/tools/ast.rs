use crate::token::Token;

#[derive(Clone, PartialEq, Debug)]
pub struct Unary {
    pub op: Token,
    pub rhs: Box<Expr>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Binary {
    pub op: Token,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    LITERAL(Token),
    UNARY(Unary),
    BINARY(Binary),
    GROUPING(Grouping),
}
