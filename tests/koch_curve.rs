use hyperion::{LSystemBuilder, Rule, State};
use pretty_assertions::assert_eq;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum KochCurve {
    F,
    L,
    R,
}

#[test]
fn generations() {
    use KochCurve::*;
    let lsys = LSystemBuilder::new([F])
        .rule(Rule::new(F, [F, L, F, R, F, R, F, L, F]))
        .build();
    assert_eq!(lsys.sample(0), State::new([F]));
    assert_eq!(lsys.sample(1), State::new(vec![F, L, F, R, F, R, F, L, F]));
}
