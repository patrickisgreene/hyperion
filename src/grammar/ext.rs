use crate::LSystemBuilder;

use super::{parser::{parse_rule, parse_state}, Token};

impl LSystemBuilder<Token> {
    pub fn new_str(axiom: &str) -> Result<LSystemBuilder<Token>, nom::Err<nom::error::Error<&str>>> {
        let (_, state) = parse_state(axiom)?;
        Ok(LSystemBuilder::new(state))
    }

    pub fn rule_str(self, rule: &str) -> Result<LSystemBuilder<Token>, nom::Err<nom::error::Error<&str>>> {
        Ok(self.rule(parse_rule(rule)?.1))
    }
}