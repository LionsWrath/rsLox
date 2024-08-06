use crate::ast_expr::Expr;
use crate::token::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum Stmt {
    BLOCK(Block),
    EXPRESSION(Expression),
    PRINT(Print),
    VAR(Var),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
    pub expr: Box<Expr>,
}

impl Expression {
   pub fn new(expr: Box<Expr>) -> Self {
        Expression {
            expr,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Print {
    pub expr: Box<Expr>,
}

impl Print {
   pub fn new(expr: Box<Expr>) -> Self {
        Print {
            expr,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<Expr>>,
}

impl Var {
   pub fn new(name: Token, initializer: Option<Box<Expr>>) -> Self {
        Var {
            name,
            initializer,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
   pub fn new(statements: Vec<Stmt>) -> Self {
        Block {
            statements,
        }
    }
}


