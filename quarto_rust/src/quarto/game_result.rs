use std::fmt;

pub enum GameResult {
    PlayerOneWon,
    PlayerTwoWon,
    Draw,
    PlayerOneInvalid,
    PlayerTwoInvalid,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           GameResult::PlayerOneWon => write!(f, "1"),
           GameResult::PlayerTwoWon => write!(f, "2"),
           GameResult::Draw => write!(f, "0"),
           GameResult::PlayerOneInvalid => write!(f, "-1"),
           GameResult::PlayerTwoInvalid => write!(f, "-2"),
       }
    }
}

