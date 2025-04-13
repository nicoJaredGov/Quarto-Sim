use super::quarto_agent::QuartoAgent;
use std::collections::HashSet;
use super::utils as qutils;

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

    pub fn make_first_move(&mut self, next_piece: u8) -> bool {
        if self.available_pieces.contains(&next_piece) {
            self.current_piece = next_piece;
            self.available_pieces.remove(&next_piece);
            true
        } else {
            println!("{} does not exist!\n", next_piece);
            false
        }
    }

    pub fn make_move(&mut self, position: u8, next_piece: u8){
        let (row, col) = qutils::get_2d_coords(position);
        self.board[row as usize][col as usize] = self.current_piece;
        self.available_positions.remove(&position);

        self.current_piece = next_piece;
        self.available_pieces.remove(&self.current_piece);
    }

    pub fn make_last_move(&mut self) {
        let last_position = self.available_positions.iter().next().unwrap().clone();
        let (row, col) = qutils::get_2d_coords(last_position);
        self.board[row as usize][col as usize] = self.current_piece;
    }

    pub fn is_valid_move(&self, position: u8, next_piece: u8) -> bool {
        let mut is_valid = true;

        if !self.available_pieces.contains(&next_piece) {
            println!("This piece has already been placed or will be placed now");
            is_valid = false;
        }
        if !self.available_positions.contains(&position) {
            println!("This cell is unavailable");
            is_valid = false;
        }

        is_valid
    }
}
