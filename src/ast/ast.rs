use crate::token::Token;

#[derive(Clone, PartialEq, Debug)]
pub struct Unary {
    pub op: Token,
    pub rhs: Box<Expr>
}

impl Unary {
    pub fn new(op: Token, rhs: Box<Expr>) -> Self {
        Unary {
            op,
            rhs,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Binary {
    pub op: Token,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl Binary {
    pub fn new(op: Token, lhs: Box<Expr>, rhs: Box<Expr>) -> Self {
        Binary {
            op,
            lhs,
            rhs,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

impl Grouping {
    pub fn new(expr: Box<Expr>) -> Grouping {
        Grouping {
            expr
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Comma {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl Comma {
    pub fn new(lhs: Box<Expr>, rhs: Box<Expr>) -> Self {
        Comma {
            lhs,
            rhs,
        }       
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Ternary {
    pub cond: Box<Expr>,
    pub then_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
}

impl Ternary {
    pub fn new(cond: Box<Expr>, then_expr: Box<Expr>, else_expr: Box<Expr>) -> Self {
        Ternary {
            cond,
            then_expr,
            else_expr,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    BOOL(bool),
    NUMBER(f64),
    STRING(String),
    NIL,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    COMMA(Comma),
    LITERAL(Literal),
    UNARY(Unary),
    BINARY(Binary),
    GROUPING(Grouping),
    TERNARY(Ternary),
}
