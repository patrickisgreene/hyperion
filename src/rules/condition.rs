use crate::{Conditional, Value, Variables};

#[derive(Debug, PartialEq, Clone)]
pub enum ConditionalValue {
    Value(Value),
    Condition(Box<Condition>),
}

impl ConditionalValue {
    pub(crate) fn is_true(
        &self,
        parameters: &Variables,
        variables: &Variables,
    ) -> Result<bool, &Value> {
        match self {
            ConditionalValue::Value(val) => Err(val),
            ConditionalValue::Condition(cond) => Ok(cond.is_true(parameters, variables)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub a: ConditionalValue,
    pub cond: Conditional,
    pub b: ConditionalValue,
}

impl Condition {
    pub(crate) fn is_true(&self, parameters: &Variables, variables: &Variables) -> bool {
        match self.a.is_true(parameters, variables) {
            Ok(false) => false,
            Ok(true) => match self.cond {
                Conditional::And => self.b.is_true(parameters, variables).unwrap_or(false),
                Conditional::Or => true,
                Conditional::EqualTo => self.b.is_true(parameters, variables).unwrap_or(false),
                Conditional::GreaterThan => panic!("Called Greater than on boolean"),
                Conditional::LessThan => panic!("Called Less than on boolean"),
            },
            Err(val) => {
                //let a = val.evaluate(parameters, variables);
                match self.b.is_true(parameters, variables) {
                    Err(val_b) => match self.cond {
                        Conditional::And => val == val_b,
                        Conditional::Or => {
                            panic!("Calling or with a value argument is unsupported")
                        }
                        Conditional::EqualTo => val == val_b,
                        Conditional::GreaterThan => val > val_b,
                        Conditional::LessThan => val < val_b,
                    },
                    Ok(_) => panic!("Attempting to compare value and boolean"),
                }
            }
        }
    }
}
