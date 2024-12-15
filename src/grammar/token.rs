use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Token {
    F,
    Pop,
    Push,
    Left,
    Up,
    Down,
    Right,
    Roll,
    Rotate,
    CounterRoll,
    EndPolygon,
    StartPolygon,
    PolygonVertex,
    External(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::F => write!(f, "F"),
            Token::Pop => write!(f, "]"),
            Token::Push => write!(f, "["),
            Token::Left => write!(f, "+"),
            Token::Right => write!(f, "-"),
            Token::Rotate => write!(f, "$"),
            Token::Up => write!(f, "&"),
            Token::Down => write!(f, "^"),
            Token::Roll => write!(f, "/"),
            Token::EndPolygon => write!(f, "}}"),
            Token::StartPolygon => write!(f, "{{"),
            Token::PolygonVertex => write!(f, "."),
            Token::CounterRoll => write!(f, "\\"),
            Token::External(x) => write!(f, "{x}"),
        }
    }
}