use hyperion::{LSystemBuilder, Rule, State};
use pretty_assertions::assert_eq;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BinaryTree {
    One,
    Zero,
    Push,
    Pop,
}

#[test]
fn generations() {
    use BinaryTree::*;
    let lsys = LSystemBuilder::new([Zero])
        .rule(Rule::new(One, [One, One]))
        .rule(Rule::new(Zero, [One, Push, Zero, Pop, Zero]))
        .build();
    assert_eq!(lsys.sample(0), State::new([Zero]));
    assert_eq!(lsys.sample(1), State::new(vec![One, Push, Zero, Pop, Zero]));
    assert_eq!(
        lsys.sample(2),
        State::new(vec![
            One, One, Push, One, Push, Zero, Pop, Zero, Pop, One, Push, Zero, Pop, Zero,
        ])
    );
    assert_eq!(
        lsys.sample(3),
        State::new(vec![
            One, One, One, One, Push, One, One, Push, One, Push, Zero, Pop, Zero, Pop, One, Push,
            Zero, Pop, Zero, Pop, One, One, Push, One, Push, Zero, Pop, Zero, Pop, One, Push, Zero,
            Pop, Zero,
        ])
    );
}
