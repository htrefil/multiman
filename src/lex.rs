use crate::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    Add,
    Sub,
    Mul,
    Div,
    Ident(u8),
    Real(f64),
    Imag(f64),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut chars = input.as_bytes().iter().enumerate();
    let mut last = None;

    while let Some((i, c)) = last.take().or_else(|| chars.next()) {
        let kind = match c {
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'+' => TokenKind::Add,
            b'-' => TokenKind::Sub,
            b'*' => TokenKind::Mul,
            b'/' => TokenKind::Div,
            b'w' | b'h' | b'x' | b'y' | b'c' | b'z' => TokenKind::Ident(*c),
            b'i' => TokenKind::Imag(1.0),
            b'0'..=b'9' => {
                let mut dot = false;
                let mut complex = false;
                let mut end = i + 1;

                while let Some((i, c)) = chars.next() {
                    match c {
                        b'.' => {
                            if dot {
                                last = Some((i, c));
                                break;
                            }

                            dot = true;
                        }
                        b'i' => {
                            complex = true;
                            end = i;
                            break;
                        }
                        b'0'..=b'9' => continue,
                        _ => {
                            last = Some((i, c));
                            end = i;
                            break;
                        }
                    }
                }

                let n = input[i..end].parse::<f64>().map_err(|_| Error {
                    message: "Invalid floating point literal",
                    position: i + 1,
                })?;

                if complex {
                    TokenKind::Imag(n)
                } else {
                    TokenKind::Real(n)
                }
            }
            b' ' | b'\t' => continue,
            _ => {
                return Err(Error {
                    message: "Unexpected character",
                    position: i + 1,
                });
            }
        };

        tokens.push(Token {
            kind: kind,
            position: i + 1,
        });
    }

    Ok(tokens)
}
