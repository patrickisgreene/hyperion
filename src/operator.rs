use std::fmt;

/// Mathamatical operator that can be used on parameters.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exponent
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Exponent => write!(f, "^")
        }
    }
}
