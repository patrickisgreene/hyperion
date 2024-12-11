/// Marker trait for a type that can be used as an LSystem grammer.
pub trait Alphabet: Copy + PartialEq {}

impl <T: Copy + PartialEq>Alphabet for T {}