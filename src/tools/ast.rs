use crate::token::Token;

pub enum Literal {
    NUMBER(Token::NUMBER),
    STRING(Token::STRING),

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
