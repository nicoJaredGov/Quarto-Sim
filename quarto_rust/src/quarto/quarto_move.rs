use std::fmt;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct QuartoMove(pub u8, pub u8);

impl fmt::Display for QuartoMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}) ", self.0, self.1)
    }
}