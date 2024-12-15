use hyperion::{grammar::Token, Condition, Conditional, ConditionalValue, LSystemBuilder, Module, Operator, Rule, State, Value};
use pretty_assertions::assert_eq;

#[test]
fn parse_sympodial() {
    let parsed = LSystemBuilder::new_str("A(1,0.25)")
        .unwrap()
        .rule_str("A(l,w)->F(l,w)[W&(c)B(l*b,w*h)]//(180)[&(d)B(l*e,w*h)")
        .unwrap()
        .build();

    let axiom_mod = Module::new(Token::External('A')).params(vec![Value::Num(1.0), Value::Num(0.25)]);
    let rule_mod = Module::new(Token::External('A')).params(vec![Value::Var('l'), Value::Var('w')]);
    let rule_state = State::new([
        Module::new(Token::F).params(vec![Value::Var('l'), Value::Var('w')]),
        Module::new(Token::Push),
        Module::new(Token::External('W')),
        Module::new(Token::Up).params(vec![Value::Var('c')]),
        Module::new(Token::External('B')).params(vec![
            Value::Expr(Box::new(Value::Var('l')), Operator::Mul, Box::new(Value::Var('b'))),
            Value::Expr(Box::new(Value::Var('w')), Operator::Mul, Box::new(Value::Var('h')))
        ]),
        Module::new(Token::Pop),
        Module::new(Token::Roll),
        Module::new(Token::Roll).params(vec![Value::Num(180.0)]),
        Module::new(Token::Push),
        Module::new(Token::Up).params(vec![Value::Var('d')]),
        Module::new(Token::External('B')).params(vec![
            Value::Expr(Box::new(Value::Var('l')), Operator::Mul, Box::new(Value::Var('e'))),
            Value::Expr(Box::new(Value::Var('w')), Operator::Mul, Box::new(Value::Var('h')))
        ]),
    ]);
    let coded = LSystemBuilder::new(State::new([axiom_mod]))
        .rule(Rule::new(rule_mod, rule_state))
        .build();
    
    assert_eq!(coded.axiom, parsed.axiom);
    assert_eq!(coded.rules, parsed.rules);
}

#[test]
fn parse_rose_leaf() {
    let parsed = LSystemBuilder::new_str("[{A(0).}][{C(0).}]")
        .unwrap()
        .rule_str("B(i) : i > 0 -> F(d,e)B(i-1)")
        .unwrap()
        .build();

    let axiom_mod = State::new([
        Module::new(Token::Push),
        Module::new(Token::StartPolygon),
        Module::new(Token::External('A')).params(vec![Value::Num(0.0)]),
        Module::new(Token::PolygonVertex),
        Module::new(Token::EndPolygon),
        Module::new(Token::Pop),
        Module::new(Token::Push),
        Module::new(Token::StartPolygon),
        Module::new(Token::External('C')).params(vec![Value::Num(0.0)]),
        Module::new(Token::PolygonVertex),
        Module::new(Token::EndPolygon),
        Module::new(Token::Pop),
    ]);
    let rule_mod = Module::new(Token::External('B')).params(vec![Value::Var('i')]);
    let rule_state = State::new([
        Module::new(Token::F).params(vec![Value::Var('d'), Value::Var('e')]),
        Module::new(Token::External('B')).params(vec![
            Value::Expr(Box::new(Value::Var('i')), Operator::Sub, Box::new(Value::Num(1.0))),
        ]),
    ]);
    let coded = LSystemBuilder::new(axiom_mod)
        .rule(Rule::new(rule_mod, rule_state).with_condition(Some(Condition {
            a: ConditionalValue::Value(Value::Var('i')),
            cond: Conditional::GreaterThan,
            b: ConditionalValue::Value(Value::Num(0.0))
        })))
        .build();
    
    assert_eq!(coded.axiom, parsed.axiom);
    assert_eq!(coded.rules, parsed.rules);
}

#[test]
fn parse_phylotaxis() {
    let parsed = LSystemBuilder::new_str("A(0)")
        .unwrap()
        .rule_str("A(n) -> +(a)[f(n^0.5)D]A(n+1)")
        .unwrap()
        .build();

    let axiom_mod = Module::new(Token::External('A')).params(vec![Value::Num(0.0)]);
    let rule_mod = Module::new(Token::External('A')).params(vec![Value::Var('n')]);
    let rule_state = State::new([
        Module::new(Token::Left).params(vec![Value::Var('a')]),
        Module::new(Token::Push),
        Module::new(Token::External('f')).params(vec![Value::Expr(Box::new(Value::Var('n')), Operator::Exponent, Box::new(Value::Num(0.5)))]),
        Module::new(Token::External('D')),
        Module::new(Token::Pop),
        Module::new(Token::External('A')).params(vec![Value::Expr(Box::new(Value::Var('n')), Operator::Add, Box::new(Value::Num(1.0)))]),
    ]);
    let coded = LSystemBuilder::new(State::new([axiom_mod]))
        .rule(Rule::new(rule_mod, rule_state))
        .build();
    
    assert_eq!(coded.axiom, parsed.axiom);
    assert_eq!(coded.rules, parsed.rules);
}