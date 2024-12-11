use crate::{Alphabet, Axiom, LSystem, Rule, Rules, Value, Variables};

pub struct LSystemBuilder<A: Alphabet> {
    axiom: Axiom<A>,
    rules: Rules<A>,
    variables: Variables,
}

impl<A: Alphabet> LSystemBuilder<A> {
    pub fn new<I: Into<Axiom<A>>>(axiom: I) -> LSystemBuilder<A> {
        LSystemBuilder {
            axiom: axiom.into(),
            rules: Default::default(),
            variables: Default::default(),
        }
    }

    /// Insert a variable into the variable cache.
    pub fn variable(mut self, key: char, val: Value) -> Self {
        self.variables.insert(key, val);
        self
    }

    pub fn rule(mut self, rule: Rule<A>) -> Self {
        self.rules.append(rule);
        self
    }

    pub fn build(self) -> LSystem<A> {
        LSystem::new(self.axiom, self.rules, self.variables)
    }
}
