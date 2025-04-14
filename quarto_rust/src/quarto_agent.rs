pub mod human_player;
pub mod random_agent;

use crate::quarto::{QuartoGameState, QuartoMove};

pub struct QuartoAgent {
    agent: Box<dyn Agent>,
}

impl QuartoAgent {
    pub fn new(agent: Box<dyn Agent>) -> QuartoAgent {
        QuartoAgent { agent }
    }
    pub fn make_move(&self, state: QuartoGameState) -> QuartoMove {
        let player_move = self.agent.make_move(state);
        println!(
            "{} made move ({},{})\n",
            self.agent.get_name(),
            player_move.0,
            player_move.1
        );
        player_move
    }
    pub fn name(&self) -> String {
        self.agent.get_name()
    }
}

pub trait Agent {
    fn make_first_move(&self) -> u8;
    fn make_move(&self, state: QuartoGameState) -> QuartoMove;
    fn get_name(&self) -> String;
}
