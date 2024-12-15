use std::{fmt, ops};

use crate::{Operator, Variables};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Num(f32),
    Var(char),
    Color(f32, f32, f32, f32),
    Expr(Box<Self>, Operator, Box<Self>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(x) => write!(f, "{x}"),
            Value::Var(x) => write!(f, "{x}"),
            Value::Color(r, g, b, a) => write!(f, "rgba({r}, {g}, {b}, {a})"),
            Value::Expr(a, op, b) => write!(f, "{a} {op} {b}"),
        }
    }
}

impl Value {

    /// Evaluate this value fetching variables from first the parameters or the variables
    /// argument.
    pub fn evaluate(&self, parameters: &Variables, variables: &Variables) -> Self {
        match self {
            Value::Var(x) => {
                if let Some(a) = parameters.get(x).cloned() {
                    a
                } else if let Some(b) = variables.get(x).cloned() {
                    b
                } else {
                    Value::Var(*x)
                }
            }
            Value::Expr(a, op, b) => {
                let a = a.evaluate(parameters, variables);
                let b = b.evaluate(parameters, variables);
                // TODO: Remove these unwraps.
                match op {
                    Operator::Add => a + b,
                    Operator::Sub => a - b,
                    Operator::Div => a / b,
                    Operator::Mul => a * b,
                    Operator::Exponent => Value::Num(a.to_float().unwrap().powf(b.to_float().unwrap()))
                }
            }
            s => s.clone(),
        }
    }

    /// Convert the object to a f32.
    /// 
    /// Returns `None` if the value is not `Value::Num`
    pub fn to_float(&self) -> Option<f32> {
        if let Value::Num(x) = self {
            Some(*x)
        } else {
            None
        }
    }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        if let Value::Num(a) = self {
            if let Value::Num(b) = rhs {
                Value::Num(a + b)
            } else {
                panic!("attempted to add non numeric value {rhs:?}");
            }
        } else {
            panic!("attempted to add non numeric value {self:?}");
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Value::Num(a) = self {
            if let Value::Num(b) = rhs {
                Value::Num(a - b)
            } else {
                panic!("attempted to subtract non numeric value {rhs:?}");
            }
        } else {
            panic!("attempted to subtract non numeric value {self:?}");
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        if let Value::Num(a) = self {
            if let Value::Num(b) = rhs {
                Value::Num(a * b)
            } else {
                panic!("attempted to multiply non numeric value {rhs:?}");
            }
        } else {
            panic!("attempted to multiply non numeric value {self:?}");
        }
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        if let Value::Num(a) = self {
            if let Value::Num(b) = rhs {
                Value::Num(a / b)
            } else {
                panic!("attempted to divide non numeric value {rhs:?}");
            }
        } else {
            panic!("attempted to divide non numeric value {self:?}");
        }
    }
}
