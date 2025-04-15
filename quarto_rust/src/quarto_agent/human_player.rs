use super::Agent;
use crate::{quarto::QuartoMove, quarto_agent::QuartoGameState};
use std::io;

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new(name: &str) -> HumanPlayer {
        HumanPlayer {
            name: String::from(name),
        }
    }
}

impl Agent for HumanPlayer {
    fn make_first_move(&self) -> u8 {
        get_user_move("Pick first piece for your opponent: ")
    }
    fn make_move(&self, state: QuartoGameState) -> QuartoMove {
        let _ = state;
        let position = get_user_move("Cell: ");
        let next_piece = get_user_move("Your opponent's next piece: ");
        QuartoMove(position, next_piece)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn get_user_move(display_message: &str) -> u8 {
    println!("{display_message}");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let user_input: u8 = user_input.trim().parse().unwrap();
    user_input
}
