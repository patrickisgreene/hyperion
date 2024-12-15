/// Conditional used when elvaluating LSystem rules.
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Conditional {
    Or,
    And,
    EqualTo,
    GreaterThan,
    LessThan,
}
