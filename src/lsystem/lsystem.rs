use crate::{Alphabet, Axiom, Context, Rules, State, Variables};

#[derive(Debug, Clone)]
pub struct LSystem<A: Alphabet> {
    axiom: Axiom<A>,
    rules: Rules<A>,
    pub variables: Variables,
}

impl<A: Alphabet> LSystem<A> {
    pub fn new(axiom: Axiom<A>, rules: Rules<A>, variables: Variables) -> Self {
        Self {
            axiom,
            rules,
            variables,
        }
    }

    pub fn sample(&self, generation: usize) -> State<A> {
        let mut state = State::<A>::from(&self.axiom);
        let mut variables = self.variables.clone();

        if generation == 0 {
            return state;
        }

        for generation in 1..=generation {
            variables.insert('T', crate::Value::Num(generation as f32));
            state.inner = state
                .inner
                .clone()
                .into_iter()
                .enumerate()
                .map(|(index, pattern)| -> State<A> {
                    let context = if index == 0 {
                        Context {
                            previous: None,
                            next: state.inner.get(index + 1),
                        }
                    } else if index == state.inner.len() - 1 {
                        Context {
                            previous: state.inner.get(index - 1),
                            next: None,
                        }
                    } else {
                        Context {
                            previous: state.inner.get(index - 1),
                            next: state.inner.get(index + 1),
                        }
                    };
                    self.rules
                        .process(&pattern, context, &variables)
                        .unwrap_or_else(|| State {
                            inner: vec![pattern],
                        })
                })
                .flat_map(|x|x.inner)
                .collect();
        }

        state
    }
}
