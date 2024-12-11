#![doc = include_str!("../README.md")]

mod alphabet;
mod context;
mod lsystem;
mod module;
mod operator;
mod rules;
mod value;

pub use self::alphabet::Alphabet;
pub use self::context::Context;
pub use self::lsystem::{Axiom, LSystem, LSystemBuilder, State};
pub use self::module::Module;
pub use self::operator::Operator;
pub use self::rules::{Condition, Conditional, Rule, Rules};
pub use self::value::Value;

pub type Parameters = Vec<Value>;
pub type Variables = std::collections::HashMap<char, Value>;
