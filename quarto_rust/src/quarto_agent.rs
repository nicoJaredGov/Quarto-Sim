pub mod human_player;

use human_player::HumanPlayer;

pub enum QuartoAgent {
    HumanPlayer(HumanPlayer),
    RandomAgent,
    NegamaxAgent
}

pub trait Move {
    fn make_first_move(&self) -> u8;
    fn make_move(&self) -> u8;
}