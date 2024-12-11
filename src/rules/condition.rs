use crate::{Conditional, Value, Variables};

#[derive(Debug, Clone)]
pub struct Condition {
    pub a: Value,
    pub cond: Conditional,
    pub b: Value,
}

impl Condition {
    pub fn is_true(&self, parameters: &Variables, variables: &Variables) -> bool {
        let a = self.a.evaluate(parameters, variables);
        let b = self.b.evaluate(parameters, variables);
        match self.cond {
            Conditional::EqualTo => a == b,
            Conditional::GreaterThan => a > b,
            Conditional::LessThan => a < b,
        }
    }
}
