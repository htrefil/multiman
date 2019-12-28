use crate::ast::{BinOp, Expr, ExprKind};
use crate::error::Error;
use num_complex::Complex;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

pub struct Context {
    vars: HashMap<&'static str, Value>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            vars: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &'static str, value: impl Into<Value>) {
        self.vars.insert(name, value.into());
    }

    pub fn update(&mut self, name: &str, value: impl Into<Value>) {
        *self.vars.get_mut(name).unwrap() = value.into();
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
                        let zero = match right {
                            Value::Real(num) => num == 0.0,
                            Value::Complex(num) => num.re == 0.0 && num.im == 0.0,
                        };

                        if zero {
                            return Err(Error {
                                message: "Divide by zero",
                                position: expr.position,
                            });
                        }

                        Ok(left / right)
                    }
                }
            }
            ExprKind::Var(ref var) => self.vars.get(var.as_str()).cloned().ok_or_else(|| Error {
                message: "Undefined variable",
                position: expr.position,
            }),
            ExprKind::Real(num) => Ok(Value::Real(num)),
            ExprKind::Imag(num) => Ok(Value::Complex(Complex { re: 0.0, im: num })),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Real(f64),
    Complex(Complex<f64>),
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::Real(value)
    }
}

impl From<Complex<f64>> for Value {
    fn from(value: Complex<f64>) -> Value {
        Value::Complex(value)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Real(a), Value::Real(b)) => Value::Real(a + b),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a + b),
            (Value::Real(a), Value::Complex(b)) | (Value::Complex(b), Value::Real(a)) => {
                Value::Complex(a + b)
            }
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Real(a), Value::Real(b)) => Value::Real(a - b),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a - b),
            (Value::Real(a), Value::Complex(b)) | (Value::Complex(b), Value::Real(a)) => {
                Value::Complex(a - b)
            }
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Real(a), Value::Real(b)) => Value::Real(a * b),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a * b),
            (Value::Real(a), Value::Complex(b)) | (Value::Complex(b), Value::Real(a)) => {
                Value::Complex(a * b)
            }
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Real(a), Value::Real(b)) => Value::Real(a / b),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a / b),
            (Value::Real(a), Value::Complex(b)) | (Value::Complex(b), Value::Real(a)) => {
                Value::Complex(a / b)
            }
        }
    }
}
