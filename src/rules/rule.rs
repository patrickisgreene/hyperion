use std::collections::HashMap;
use rand::{thread_rng, Rng};

use crate::{Alphabet, Condition, Context, Module, State, Value, Variables};

#[derive(Debug, Clone)]
pub struct Rule<A: Alphabet> {
    pub pattern: Module<A>,
    pub tokens: State<A>,
    pub probability: f32,
    pub previous: Option<A>,
    pub next: Option<A>,
    pub condition: Option<Condition>,
}

impl<A: Alphabet> Rule<A> {
    pub fn new<P: Into<Module<A>>, I: Into<Module<A>>, Iter: IntoIterator<Item = I>>(
        pattern: P,
        tokens: Iter,
    ) -> Rule<A> {
        Rule {
            tokens: State {
                inner: tokens
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<Module<A>>>(),
            },
            pattern: pattern.into(),
            probability: 1.0,
            next: None,
            previous: None,
            condition: None,
        }
    }
    pub fn with_previous(mut self, prev: Option<A>) -> Self {
        self.previous = prev;
        self
    }

    pub fn with_next(mut self, next: Option<A>) -> Self {
        self.next = next;
        self
    }

    pub fn with_probability(mut self, probability: f32) -> Self {
        self.probability = probability;
        self
    }
    pub fn with_condition(mut self, condition: Option<Condition>) -> Self {
        self.condition = condition;
        self
    }
    pub fn match_pattern(
        &self,
        pattern: &Module<A>,
        context: &Context<'_, A>,
        params: &Variables,
        variables: &Variables,
    ) -> bool {
        self.has_prefix(context)
            && self.has_suffix(context)
            && pattern.token == self.pattern.token
            && thread_rng().gen_range(0.0..1.0) < self.probability
            && self
                .condition
                .as_ref()
                .map(|x| x.is_true(params, variables))
                .unwrap_or(true)
    }

    pub fn produce(&self, params: &Variables, variables: &Variables) -> State<A> {
        let inner = self
            .tokens
            .clone()
            .inner
            .into_iter()
            .map(|mut x| {
                x.params = x
                    .params
                    .into_iter()
                    .map(|x| x.evaluate(params, variables))
                    .collect();
                x
            })
            .collect();

        State { inner }
    }

    fn has_prefix(&self, ctx: &Context<'_, A>) -> bool {
        if let Some(_pre) = self.previous {
            return Some(_pre) == ctx.previous.map(|x| x.token);
        }
        true
    }

    fn has_suffix(&self, ctx: &Context<'_, A>) -> bool {
        if let Some(_pre) = self.next {
            return Some(_pre) == ctx.next.map(|x| x.token);
        }
        true
    }

    pub(crate) fn params(&self, caller: &Module<A>) -> Variables {
        let mut params = HashMap::with_capacity(self.pattern.params.len());

        for (dex, p) in self.pattern.params.iter().enumerate() {
            match p {
                Value::Var(n) => {
                    if caller.params.len() > dex {
                        params.insert(*n, caller.params[dex].clone());
                    }
                }
                _ => unreachable!(),
            }
        }

        params
    }
}
