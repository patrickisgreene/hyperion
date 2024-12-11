use hyperion::{LSystemBuilder, Rule, State};
use pretty_assertions::assert_eq;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Contextual {
    A,
    B,
}

#[test]
fn generations() {
    use Contextual::*;
    let lsys = LSystemBuilder::new([B, A])
        .rule(Rule::new(A, [A, A, B]).with_previous(Some(B)))
        .build();
    assert_eq!(lsys.sample(0), State::new([B, A]));

    assert_eq!(lsys.sample(1), State::new([B, A, A, B]));
    assert_eq!(lsys.sample(2), State::new([B, A, A, B, A, B]));
    assert_eq!(lsys.sample(3), State::new([B, A, A, B, A, B, A, A, B, B]));
}
