use hyperion::{LSystemBuilder, Module, Operator, Rule, State, Value};
use pretty_assertions::assert_eq;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Parametric {
    F,
    B,
    Up,
    Roll,
    Push,
    Pop,
}

#[test]
fn generations() {
    use Operator::*;
    use Parametric::*;
    use Value::*;
    let axiom_params = vec![Value::Num(1.0), Value::Num(10.0)];
    let axiom = [Module::new(B).params(axiom_params)];
    let lsys = LSystemBuilder::new(axiom)
        .rule(Rule::new(
            Module::new(B).params(vec![Var('l'), Var('w')]),
            [
                Module::new(F).params(vec![Var('l'), Var('w')]),
                Module::new(Push),
                Module::new(Up).params(vec![Var('c')]),
                Module::new(B).params(vec![
                    Expr(Box::new(Var('l')), Mul, Box::new(Var('b'))),
                    Expr(Box::new(Var('w')), Mul, Box::new(Var('h'))),
                ]),
                Module::new(Roll),
                Module::new(Roll).params(vec![Value::Num(180.0)]),
                Module::new(Push),
                Module::new(Up).params(vec![Value::Var('d')]),
                Module::new(B).params(vec![
                    Expr(Box::new(Var('l')), Mul, Box::new(Var('e'))),
                    Expr(Box::new(Var('w')), Mul, Box::new(Var('h'))),
                ]),
            ],
        ))
        .variable('e', Value::Num(1.0))
        .variable('b', Value::Num(5.0))
        .variable('h', Value::Num(10.0))
        .variable('c', Value::Num(100.0))
        .variable('d', Value::Num(1000.0))
        .build();
    assert_eq!(
        lsys.sample(1),
        State::new([
            Module::new(F).params(vec![Value::Num(1.0), Value::Num(10.0)]),
            Module::new(Push),
            Module::new(Up).params(vec![Value::Num(100.0)]),
            Module::new(B).params(vec![Value::Num(5.0), Value::Num(100.0)]),
            Module::new(Roll),
            Module::new(Roll).params(vec![Value::Num(180.0)]),
            Module::new(Push),
            Module::new(Up).params(vec![Value::Num(1000.0)]),
            Module::new(B).params(vec![Value::Num(1.0), Value::Num(100.0)]),
        ])
    );
}
