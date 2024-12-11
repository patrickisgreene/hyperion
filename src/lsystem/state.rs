use std::fmt;

use crate::{Alphabet, Axiom, Module};

#[derive(Debug, Clone, PartialEq)]
pub struct State<A: Alphabet> {
    pub(crate) inner: Vec<Module<A>>,
}

impl<A: Alphabet> State<A> {
    pub fn new<I: Into<Module<A>>, Iter: IntoIterator<Item = I>>(value: Iter) -> Self {
        State {
            inner: value.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl<A: Alphabet> State<A> {
    pub fn contains(&self, m: &Module<A>) -> bool {
        self.inner.contains(m)
    }
}

impl<A: Alphabet> Default for State<A> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<'a, A: Alphabet> From<&'a Axiom<A>> for State<A> {
    fn from(value: &'a Axiom<A>) -> Self {
        value.inner.clone()
    }
}

impl<A: Alphabet> From<Module<A>> for State<A> {
    fn from(value: Module<A>) -> Self {
        State { inner: vec![value] }
    }
}

impl<A: Alphabet> IntoIterator for State<A> {
    type Item = Module<A>;

    type IntoIter = std::vec::IntoIter<Module<A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<A: Alphabet + fmt::Display> fmt::Display for State<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for m in &self.inner {
            out.push_str(&format!("{m}"));
        }
        write!(f, "{out}")
    }
}
