use super::Agent;
use crate::{quarto::QuartoMove, quarto_agent::QuartoGameState};
use rand::Rng;

pub struct RandomAgent;

impl Agent for RandomAgent {
    fn make_first_move(&self) -> u8 {
        rand::rng().random_range(0..16)
    }
    fn make_move(&self, state: QuartoGameState) -> QuartoMove {
        let position = state.available_positions.iter().next().unwrap().clone();
        let next_piece = state.available_pieces.iter().next().unwrap().clone();
        QuartoMove(position, next_piece)
    }
    fn get_name(&self) -> String {
        String::from("Random Agent")
    }
}
