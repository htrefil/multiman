use super::lex::TokenKind;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Bin(BinOp, Box<Expr>, Box<Expr>),
    Var(u8),
    Real(f64),
    Imag(f64),
}

#[derive(Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub position: usize,
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    pub fn from_token(kind: &TokenKind) -> Option<BinOp> {
        let op = match kind {
            TokenKind::Add => BinOp::Add,
            TokenKind::Sub => BinOp::Sub,
            TokenKind::Mul => BinOp::Mul,
            TokenKind::Div => BinOp::Div,
            _ => return None,
        };

        Some(op)
    }

    pub fn precedence(&self) -> u32 {
        match self {
            BinOp::Add | BinOp::Sub => 1,
            BinOp::Mul | BinOp::Div => 2,
        }
    }
}
