use crate::{quarto::QuartoMove, quarto_agent::QuartoGameState};
use super::Agent;

pub struct HumanPlayer {
    name: String
}

impl HumanPlayer {
    pub fn new(name: &str) -> HumanPlayer {
        HumanPlayer { name: String::from(name) }
    }
}

impl Agent for HumanPlayer {
    fn make_first_move(&self) -> u8 {
        println!("Human made first move");
        16
    }
    fn make_move(&self, state: QuartoGameState) -> QuartoMove {
        println!("Human made a move");
        QuartoMove(16, 16)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
