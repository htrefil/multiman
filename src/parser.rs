use crate::ast::{BinOp, Expr, ExprKind};
use crate::error::Error;
use crate::lex::{Token, TokenKind};

pub fn parse(tokens: &[Token]) -> Result<Expr, Error> {
    let mut parser = Parser {
        tokens,
        idx: 0,
        last_position: 1,
    };
    let expr = parser.parse(1)?;

    if let Some(token) = parser.next() {
        return Err(Error {
            message: "Extra token",
            position: token.position,
        });
    }

    Ok(expr)
}

struct Parser<'a> {
    tokens: &'a [Token],
    idx: usize,
    last_position: usize,
}

impl<'a> Parser<'a> {
    fn parse_primary(&mut self, token: &Token) -> Result<Expr, Error> {
        let kind = match token.kind {
            TokenKind::LParen => {
                let expr = self.parse(1)?;

                match self.next().map(|token| &token.kind) {
                    Some(TokenKind::RParen) => return Ok(expr),
                    _ => return Err(self.error("Unclosed (")),
                }
            }
            TokenKind::Sub => {
                let left = Expr {
                    kind: ExprKind::Real(0.0),
                    position: token.position,
                };

                let right = self
                    .next()
                    .ok_or_else(|| self.error("Expected a token"))
                    .and_then(|token| self.parse_primary(token))?;

                ExprKind::Bin(BinOp::Sub, left.into(), right.into())
            }
            TokenKind::Real(num) => ExprKind::Real(num),
            TokenKind::Imag(num) => ExprKind::Imag(num),
            TokenKind::Ident(ident) => ExprKind::Var(ident),
            _ => return Err(self.error("Unexpected token")),
        };

        Ok(Expr {
            kind,
            position: token.position,
        })
    }

    fn parse(&mut self, min_precedence: u32) -> Result<Expr, Error> {
        let mut expr = self
            .next()
            .ok_or_else(|| self.error("Expected a token"))
            .and_then(|token| self.parse_primary(token))?;

        loop {
            let op = match self.peek().and_then(|token| BinOp::from_token(&token.kind)) {
                Some(op) => op,
                None => break,
            };

            if op.precedence() < min_precedence {
                return Ok(expr);
            }

            self.next();

            let right = self.parse(op.precedence() + 1)?;
            expr = Expr {
                position: self.last_position,
                kind: ExprKind::Bin(op, expr.into(), right.into()),
            };
        }

        Ok(expr)
    }

    fn error(&self, message: &'static str) -> Error {
        Error {
            message,
            position: self.last_position,
        }
    }

    fn peek(&self) -> Option<&'a Token> {
        if self.idx >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[self.idx])
    }

    fn next(&mut self) -> Option<&'a Token> {
        if self.idx >= self.tokens.len() {
            return None;
        }

        let token = &self.tokens[self.idx];
        self.last_position = token.position;
        self.idx += 1;

        Some(token)
    }
}
