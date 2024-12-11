use hyperion::{LSystemBuilder, Rule, State};
use pretty_assertions::assert_eq;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Algea {
    A,
    B,
}

#[test]
fn generations() {
    use Algea::*;
    let lsys = LSystemBuilder::new([A])
        .rule(Rule::new(A, [A, B]))
        .rule(Rule::new(B, [A]))
        .build();
    assert_eq!(lsys.sample(0), State::new([A]));
    assert_eq!(lsys.sample(1), State::new([A, B]));
    assert_eq!(lsys.sample(2), State::new(vec![A, B, A]));
    assert_eq!(lsys.sample(3), State::new(vec![A, B, A, A, B]));
    assert_eq!(lsys.sample(4), State::new(vec![A, B, A, A, B, A, B, A]));
    assert_eq!(
        lsys.sample(5),
        State::new([A, B, A, A, B, A, B, A, A, B, A, A, B])
    );
    assert_eq!(
        lsys.sample(6),
        State::new([A, B, A, A, B, A, B, A, A, B, A, A, B, A, B, A, A, B, A, B, A])
    );
    assert_eq!(
        lsys.sample(7),
        State::new([
            A, B, A, A, B, A, B, A, A, B, A, A, B, A, B, A, A, B, A, B, A, A, B, A, A, B, A, B, A,
            A, B, A, A, B
        ])
    );
}
