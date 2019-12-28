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
    Ident(String),
    Real(f64),
    Imag(f64),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().enumerate().map(|(i, c)| (i + 1, c));
    let mut last = None;

    while let Some((i, c)) = last.take().or_else(|| chars.next()) {
        let kind = match c {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '+' => TokenKind::Add,
            '-' => TokenKind::Sub,
            '*' => TokenKind::Mul,
            '/' => TokenKind::Div,
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut data = c.to_string();

                while let Some((i, c)) = chars.next() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => data.push(c),
                        _ => {
                            last = Some((i, c));
                            break;
                        }
                    }
                }

                if data == "i" {
                    TokenKind::Imag(1.0)
                } else {
                    TokenKind::Ident(data)
                }
            }
            '0'..='9' => {
                let mut dot = false;
                let mut complex = false;
                let mut data = c.to_string();

                while let Some((i, c)) = chars.next() {
                    match c {
                        '.' => {
                            if dot {
                                last = Some((i, c));
                                break;
                            }

                            dot = true;
                            data.push(c);
                        }
                        'i' => {
                            complex = true;
                            break;
                        }
                        '0'..='9' => data.push(c),
                        _ => {
                            last = Some((i, c));
                            break;
                        }
                    }
                }

                let n = data.parse::<f64>().map_err(|_| Error {
                    message: "Invalid floating point literal",
                    position: i,
                })?;

                if complex {
                    TokenKind::Imag(n)
                } else {
                    TokenKind::Real(n)
                }
            }
            ' ' | '\t' => continue,
            _ => {
                return Err(Error {
                    message: "Unexpected character",
                    position: i,
                });
            }
        };

        tokens.push(Token {
            kind: kind,
            position: i,
        });
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tokens = tokenize("( ) + - * / Long_ident 123 123.456 123i 123.456i")
            .unwrap()
            .into_iter();

        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::LParen,
                position: 1
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::RParen,
                position: 3
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Add,
                position: 5
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Sub,
                position: 7
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Mul,
                position: 9
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Div,
                position: 11
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Ident("Long_ident".into()),
                position: 13
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Real(123.0),
                position: 24
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Real(123.456),
                position: 28
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Imag(123.0),
                position: 36
            }
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::Imag(123.456),
                position: 41
            }
        );
    }
}
