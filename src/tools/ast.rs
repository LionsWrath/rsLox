use crate::token::Token;

pub enum Literal {
    Object(Token),
}

pub enum UnaryOperator {
    MINUS,
    NEGATION,
}

pub enum BinaryOperator {
    EQUAL,
    DIFF, 
}

pub enum Expr {
    LITERAL(Literal),
    UNARY {
        op: UnaryOperator,
        rhs: Box<Expr>,
    },
    BINARY {
        lhs: Box<Expr>,
        op: BinaryOperator,
        rhs: Box<Expr>,
    },
    GROUPING {
        expr: Box<Expr>,
    },
}
