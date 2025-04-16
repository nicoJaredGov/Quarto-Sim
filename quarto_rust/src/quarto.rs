use super::quarto_agent::QuartoAgent;
use super::utils as qutils;
use std::collections::HashSet;

const NUM_PIECES: u8 = 16;

pub struct QuartoGameState {
    pub board: [[u8; 4]; 4],
    pub current_piece: u8,
    pub available_pieces: HashSet<u8>,
    pub available_positions: HashSet<u8>,
}

pub struct Quarto {
    player_one: QuartoAgent,
    player_two: QuartoAgent,
    state: QuartoGameState,
    is_player_one_turn: bool,
    show_console_logs: bool,
}

pub struct QuartoMove(pub u8, pub u8);

impl QuartoGameState {
    pub fn new() -> QuartoGameState {
        QuartoGameState {
            board: [[16u8; 4]; 4],
            current_piece: 16,
            available_pieces: (0..NUM_PIECES).collect(),
            available_positions: (0..NUM_PIECES).collect(),
        }
    }
}

impl Quarto {
    pub fn new(
        player_one: QuartoAgent,
        player_two: QuartoAgent,
        show_console_logs: bool,
    ) -> Quarto {
        Quarto {
            player_one,
            player_two,
            state: QuartoGameState::new(),
            is_player_one_turn: true,
            show_console_logs,
        }
    }

    pub fn make_first_move(&mut self, next_piece: u8) -> bool {
        if self.state.available_pieces.contains(&next_piece) {
            self.state.current_piece = next_piece;
            self.state.available_pieces.remove(&next_piece);
            true
        } else {
            println!("{} does not exist!\n", next_piece);
            false
        }
    }

    pub fn make_move(&mut self, player_move: QuartoMove) -> bool {
        let QuartoMove(position, next_piece) = player_move;

        if self.is_valid_move(position, next_piece) {
            qutils::update_state(&mut self.state, position, next_piece);
            true
        } else {
            false
        }
    }

    pub fn make_last_move(&mut self) {
        let last_position = self
            .state
            .available_positions
            .iter()
            .next()
            .unwrap()
            .clone();
        let (row, col) = qutils::get_2d_coords(last_position);
        self.state.board[row as usize][col as usize] = self.state.current_piece;
        self.state.available_positions.remove(&last_position);
        self.state.current_piece = 16;
    }

    pub fn is_valid_move(&self, position: u8, next_piece: u8) -> bool {
        let mut is_valid = true;

        if !self.state.available_pieces.contains(&next_piece) {
            println!("This piece has already been placed or will be placed now");
            is_valid = false;
        }
        if !self.state.available_positions.contains(&position) {
            println!("This cell is unavailable");
            is_valid = false;
        }

        is_valid
    }

    pub fn get_random_piece(&self) -> u8 {
        self.state.available_pieces.iter().next().unwrap().clone()
    }

    pub fn get_current_state(&self) -> QuartoGameState {
        QuartoGameState {
            board: self.state.board,
            current_piece: self.state.current_piece,
            available_pieces: self.state.available_pieces.clone(),
            available_positions: self.state.available_positions.clone(),
        }
    }

    pub fn is_game_over(&self) -> bool {
        if qutils::is_game_over(&self.state.board) {
            if self.is_player_one_turn {
                println!("\nPlayer 1 ({}) won!", self.player_one.name());
            } else {
                println!("\nPlayer 2 ({}) won!", self.player_two.name());
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
                player_move = self
                    .player_one
                    .make_move(self.get_current_state(), self.show_console_logs);
            } else {
                player_move = self
                    .player_two
                    .make_move(self.get_current_state(), self.show_console_logs);
            }

            self.make_move(player_move);
            self.display_state();
            if self.is_game_over() {
                return;
            }
            self.is_player_one_turn = !self.is_player_one_turn;
        }

        self.make_last_move();
        self.display_state();
        if !self.is_game_over() {
            println!("\nDraw!")
        }
    }
}

//display methods
impl Quarto {
    pub fn display_board(&self) {
        for row in self.state.board {
            println!("{row:?}");
        }
    }

    pub fn display_info(&self) {
        println!(
            "current piece to place: {}\navailable pieces: {:?}\navailable positions: {:?}\n",
            self.state.current_piece, self.state.available_pieces, self.state.available_positions,
        )
    }

    pub fn display_state(&self) {
        if self.show_console_logs {
            self.display_board();
            self.display_info();
        }
    }
}
