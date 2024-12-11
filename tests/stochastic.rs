use hyperion::{LSystemBuilder, Module, Rule};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Stochastic {
    A,
    B,
}

#[test]
fn simple() {
    use Stochastic::*;
    let lsys = LSystemBuilder::new([A, A, A, A, A])
        .rule(Rule::new(A, [B]).with_probability(0.33))
        .build();

    let output = lsys.sample(1);
    assert!(output.contains(&Module::new(B)));
    assert!(output.contains(&Module::new(A)));
}
