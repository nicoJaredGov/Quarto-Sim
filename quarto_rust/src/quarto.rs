use super::quarto_agent::QuartoAgent;
use std::collections::HashSet;

const NUM_PIECES: u8 = 16;

pub struct Quarto {
    player_one: QuartoAgent,
    player_two: QuartoAgent,
    num_retries: u8,
    board: [[u8; 4]; 4],
    current_piece: u8,
    available_pieces: HashSet<u8>,
    available_positions: HashSet<u8>,
}

impl Quarto {
    pub fn new(player_one: QuartoAgent, player_two: QuartoAgent) -> Quarto {
        Quarto {
            player_one,
            player_two,
            num_retries: 2,
            board: [[0u8; 4]; 4],
            current_piece: 16,
            available_pieces: (0..NUM_PIECES).collect(),
            available_positions: (0..NUM_PIECES).collect(),
        }
    }

    pub fn display_board(&self) {
        for row in self.board {
            println!("{row:?}");
        }
    }

    pub fn display_info(&self) {
        println!(
            "current piece to place: {}\navailable pieces: {:?}\navailable positions: {:?}",
            self.current_piece, self.available_pieces, self.available_positions,
        )
    }

    pub fn display_state(&self) {
        self.display_board();
        self.display_info();
    }

    pub fn make_first_move(&mut self, next_piece: u8) {
        if self.available_pieces.contains(&next_piece) {
            self.current_piece = next_piece;
            self.available_pieces.remove(&next_piece);
        } else {
            println!("{} does not exist!\n", next_piece);
        }
    }
}
