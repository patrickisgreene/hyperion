use crate::{Alphabet, Module};

/// The context given to each Grammar token when evaluating rules.
pub struct Context<'a, A: Alphabet> {

    /// The grammer token that came right before this one.
    pub previous: Option<&'a Module<A>>,
    
    /// The grammar token that comes right after this one.
    pub next: Option<&'a Module<A>>,
}
