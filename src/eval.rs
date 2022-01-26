use crate::ast::{BinOp, Expr, ExprKind};
use crate::error::Error;
use num_complex::Complex;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Default)]
pub struct Context {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub c: Value,
    pub z: Value,
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    fn get(&self, name: u8) -> Option<Value> {
        match name {
            b'w' => Some(self.width.into()),
            b'h' => Some(self.height.into()),
            b'x' => Some(self.x.into()),
            b'y' => Some(self.y.into()),
            b'c' => Some(self.c.into()),
            b'z' => Some(self.z.into()),
            _ => None,
        }
    }

    pub fn eval(&self, expr: &Expr) -> Result<Value, Error> {
        match expr.kind {
            ExprKind::Bin(op, ref left, ref right) => {
                let left = self.eval(left)?;
                let right = self.eval(right)?;

                match op {
                    BinOp::Add => Ok(left + right),
                    BinOp::Sub => Ok(left - right),
                    BinOp::Mul => Ok(left * right),
                    BinOp::Div => {
                        if right.v == 0.0.into() {
                            return Err(Error {
                                message: "Divide by zero",
                                position: expr.position,
                            });
                        }

                        Ok(left / right)
                    }
                }
            }
            ExprKind::Var(var) => self.get(var).ok_or_else(|| Error {
                message: "Undefined variable",
                position: expr.position,
            }),
            ExprKind::Real(num) => Ok(num.into()),
            ExprKind::Imag(num) => Ok(Complex { re: 0.0, im: num }.into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Value {
    /// Value of the expression
    pub v: Complex<f64>,
    /// Derivative of the expression
    pub d: Complex<f64>,
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value {
            v: value.into(),
            d: 0.0.into(),
        }
    }
}

impl From<Complex<f64>> for Value {
    fn from(value: Complex<f64>) -> Value {
        Value {
            v: value,
            d: 0.0.into(),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        Value {
            v: self.v + other.v,
            d: self.d + other.d,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        Value {
            v: self.v - other.v,
            d: self.d - other.d,
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        Value {
            v: self.v * other.v,
            d: self.v * other.d + other.v * self.d,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        Value {
            v: self.v / other.v,
            d: (self.d * other.v - other.d * self.v) / (other.d * other.d),
        }
    }
}
