use nom::{
    branch::alt, bytes::complete::tag, character::complete::{alpha1, space0}, combinator::map, error::Error, number::complete::double, IResult
};

use crate::{Condition, Conditional, ConditionalValue, Module, Operator, Parameters, Rule, State, Value};

use super::Token;

pub fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("+"), |_| Operator::Add),
        map(tag("-"), |_| Operator::Sub),
        map(tag("*"), |_| Operator::Mul),
        map(tag("/"), |_| Operator::Div),
        map(tag("^"), |_| Operator::Exponent)
    ))(input)
}

pub fn parse_conditional(input: &str) -> IResult<&str, Conditional> {
    alt((
        map(tag("="), |_| Conditional::EqualTo),
        map(tag(">"), |_| Conditional::GreaterThan),
        map(tag("<"), |_| Conditional::LessThan),
        map(tag("&"), |_| Conditional::And),
        map(tag("|"), |_| Conditional::Or),
    ))(input)
}

pub fn parse_lit_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(double, |s| Value::Num(s as f32)),
        map(alpha1, |s: &str| Value::Var(s.chars().next().unwrap())),
    ))(input)
}

pub fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((parse_expr, parse_lit_value))(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Value> {
    let (input, _) = space0(input)?;
    let (input, a) = parse_lit_value(input)?;
    let (input, _) = space0(input)?;
    let (input, cond) = parse_operator(input)?;
    let (input, _) = space0(input)?;
    let (input, b) = parse_lit_value(input)?;
    Ok((input, Value::Expr(Box::new(a), cond, Box::new(b))))
}

pub fn parse_condition_value(input: &str) -> IResult<&str, ConditionalValue> {
    let (input, _) = space0(input)?;
    alt((
        map(parse_value, ConditionalValue::Value),
        map(parse_condition, |expr| ConditionalValue::Condition(Box::new(expr))),
    ))(input)
}

pub fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, _) = space0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, a) = parse_value(input)?;
    let (input, _) = space0(input)?;
    let (input, cond) = parse_conditional(input)?;
    let (input, _) = space0(input)?;
    let (input, b) = parse_condition_value(input)?;
    Ok((input, Condition { a: ConditionalValue::Value(a), cond, b }))
}

pub fn parse_token(input: &str) -> IResult<&str, Token> {
    let (input, _) = space0(input)?;
    alt((
        map(tag("F"), |_| Token::F),
        map(tag("+"), |_| Token::Left),
        map(tag("-"), |_| Token::Right),
        map(tag("&"), |_| Token::Up),
        map(tag("^"), |_| Token::Down),
        map(tag("["), |_| Token::Push),
        map(tag("]"), |_| Token::Pop),
        map(tag("$"), |_| Token::Rotate),
        map(tag("{"), |_| Token::StartPolygon),
        map(tag("}"), |_| Token::EndPolygon),
        map(tag("."), |_| Token::PolygonVertex),
        map(tag("\\"), |_| Token::CounterRoll),
        map(tag("/"), |_| Token::Roll),
        map(alpha1, |s: &str| Token::External(s.chars().next().unwrap())),
    ))(input)
}

pub fn open_paran(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ()))
}

pub fn close_paran(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ()))
}

pub fn comma(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ()))
}

pub fn parse_parameters(input: &str) -> IResult<&str, Parameters> {
    let (input, _) = space0(input)?;
    nom::sequence::delimited(
        open_paran,
        nom::multi::separated_list0(comma, parse_value),
        close_paran
    )(input)
}

pub fn parse_module(input: &str) -> IResult<&str, Module<Token>> {
    let (input, _) = space0(input)?;
    let (input, token) = parse_token(input)?;
    let (input, _) = space0(input)?;
    let (input, params) = parse_parameters(input).unwrap_or((input, vec![]));
    Ok((input, Module::new(token).params(params)))
}

pub fn parse_state(input: &str) -> IResult<&str, State<Token>> {
    map(nom::multi::many1(parse_module), State::new)(input)
}

pub fn parse_prefix(input: &str) -> IResult<&str, Option<Token>> {
    let res = parse_token(input);
    let (input, _) = tag::<&str, &str, Error<&str>>("<")(input)?;
    match res {
        Ok((input, token)) => Ok((input, Some(token))),
        Err(_) => Ok((input, None))
    }
}

pub fn parse_suffix(input: &str) -> IResult<&str, Option<Token>> {
    let (input, _) = space0(input)?;
    let res = parse_token(input);
    let (input, _) = tag::<&str, &str, Error<&str>>(">")(input)?;
    let (input, _) = space0(input)?;
    match res {
        Ok((_, token)) => Ok((input, Some(token))),
        Err(err) => Err(err)
    }
}

pub fn parse_probability(input: &str) -> IResult<&str, f32> {
    let (input, _) = space0(input)?;
    match tag(":")(input) {
        Err(err) => Err(err),
        Ok((input, _)) => {
            let (input, _) = space0(input)?;
            map(double, |s| s as f32)(input)
        }
    }
}

pub fn parse_rule(input: &str) -> IResult<&str, Rule<Token>> {
    let (input, prefix) = parse_prefix(input).unwrap_or((input, None));
    let (input, module) = parse_module(input)?;
    let (input, suffix) = parse_suffix(input).unwrap_or((input, None));
    let (input, condition) = parse_condition(input).map(|(x, y)|(x, Some(y))).unwrap_or((input, None));
    let (input, probability) = parse_probability(input).unwrap_or((input, 1.0));
    let (input, _) = space0(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = space0(input)?;
    let (input, state) = parse_state(input)?;
    Ok((
        input,
        Rule::new(module, state)
            .with_condition(condition)
            .with_next(suffix)
            .with_previous(prefix)
            .with_probability(probability)
    ))
}