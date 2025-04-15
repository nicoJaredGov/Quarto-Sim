use super::quarto_agent::QuartoAgent;
use super::utils as qutils;
use std::collections::HashSet;

const NUM_PIECES: u8 = 16;

pub struct Quarto {
    player_one: QuartoAgent,
    player_two: QuartoAgent,
    board: [[u8; 4]; 4],
    current_piece: u8,
    available_pieces: HashSet<u8>,
    available_positions: HashSet<u8>,
    is_player_one_turn: bool,
}

pub struct QuartoGameState {
    pub board: [[u8; 4]; 4],
    pub available_pieces: HashSet<u8>,
    pub available_positions: HashSet<u8>,
}

pub struct QuartoMove(pub u8, pub u8);

impl Quarto {
    pub fn new(player_one: QuartoAgent, player_two: QuartoAgent) -> Quarto {
        Quarto {
            player_one,
            player_two,
            board: [[16u8; 4]; 4],
            current_piece: 16,
            available_pieces: (0..NUM_PIECES).collect(),
            available_positions: (0..NUM_PIECES).collect(),
            is_player_one_turn: true,
        }
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

    pub fn make_move(&mut self, player_move: QuartoMove) -> bool {
        let QuartoMove(position, next_piece) = player_move;

        if self.is_valid_move(position, next_piece) {
            let (row, col) = qutils::get_2d_coords(position);
            self.board[row as usize][col as usize] = self.current_piece;
            self.available_positions.remove(&position);

            self.current_piece = next_piece;
            self.available_pieces.remove(&self.current_piece);
            true
        } else {
            false
        }
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

    pub fn get_random_piece(&self) -> u8 {
        self.available_pieces.iter().next().unwrap().clone()
    }

    pub fn get_current_state(&self) -> QuartoGameState {
        QuartoGameState {
            board: self.board,
            available_pieces: self.available_pieces.clone(),
            available_positions: self.available_positions.clone(),
        }
    }

    pub fn is_game_over(&self) -> bool {
        if qutils::is_game_over(&self.board) {
            if self.is_player_one_turn {
                println!("\nPlayer 1 {} won!", self.player_one.name());
            } else {
                println!("\nPlayer 2 {} won!", self.player_two.name());
            }
            return true;
        }
        return false;
    }

    pub fn run(&mut self) {
        self.make_first_move(self.get_random_piece());
        self.display_state();

        for _ in 1..16 {
            let player_move: QuartoMove;
            if self.is_player_one_turn {
                player_move = self.player_one.make_move(self.get_current_state());
            } else {
                player_move = self.player_two.make_move(self.get_current_state());
            }

            self.make_move(player_move);
            self.display_state();
            if self.is_game_over() {
                return;
            }
            self.is_player_one_turn = !self.is_player_one_turn;
            
        }
    }
}

//display methods
impl Quarto {
    pub fn display_board(&self) {
        for row in self.board {
            println!("{row:?}");
        }
    }

    pub fn display_info(&self) {
        println!(
            "current piece to place: {}\navailable pieces: {:?}\navailable positions: {:?}\n",
            self.current_piece, self.available_pieces, self.available_positions,
        )
    }

    pub fn display_state(&self) {
        self.display_board();
        self.display_info();
    }
}
