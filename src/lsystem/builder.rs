use crate::{Alphabet, Axiom, LSystem, Rule, Rules, Value, Variables};

/// Builder struct to create a LSystem.
pub struct LSystemBuilder<A: Alphabet> {
    axiom: Axiom<A>,
    rules: Rules<A>,
    variables: Variables,
}

impl<A: Alphabet> LSystemBuilder<A> {

    /// Create a new LSystemBuilder from a starting Axiom.
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

    /// Push a rule onto the rule cache.
    pub fn rule(mut self, rule: Rule<A>) -> Self {
        self.rules.append(rule);
        self
    }

    /// Finishing building and return the created LSystem.
    pub fn build(self) -> LSystem<A> {
        LSystem::new(self.axiom, self.rules, self.variables)
    }
}
