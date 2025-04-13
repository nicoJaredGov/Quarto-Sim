use super::Move;

pub struct HumanPlayer {
    name: String
}

impl HumanPlayer {
    pub fn new(name: &str) -> HumanPlayer {
        HumanPlayer { name: String::from(name) }
    }
}

impl Move for HumanPlayer {
    fn make_first_move(&self) -> u8 {
        println!("Human made first move");
        16
    }
    fn make_move(&self) -> u8 {
        println!("Human made a move");
        16
    }
}