use crate::{Alphabet, Context, Module, Rule, State, Variables};

#[derive(Debug, Clone)]
pub struct Rules<A: Alphabet> {
    inner: Vec<Rule<A>>,
}

impl<A: Alphabet> Default for Rules<A> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<A: Alphabet> Rules<A> {
    pub fn append(&mut self, rule: Rule<A>) {
        self.inner.push(rule);
    }

    pub fn process(
        &self,
        pattern: &Module<A>,
        context: Context<'_, A>,
        variables: &Variables,
    ) -> Option<State<A>> {
        for rule in &self.inner {
            let params = rule.params(pattern);
            if rule.match_pattern(pattern, &context, &params, variables) {
                return Some(rule.produce(&params, variables));
            }
        }
        None
    }
}
