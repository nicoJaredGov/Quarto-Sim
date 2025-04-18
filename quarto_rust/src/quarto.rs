pub mod quarto_game_state;
pub mod game_result;

use super::quarto_agent::QuartoAgent;
use super::utils as qutils;
use quarto_game_state::QuartoGameState;
use game_result::GameResult;

pub struct Quarto {
    player_one: QuartoAgent,
    player_two: QuartoAgent,
    state: QuartoGameState,
    is_player_one_turn: bool,
    show_console_logs: bool,
    log_stats: bool,
    num_retries_allowed: u8,
}

pub struct QuartoMove(pub u8, pub u8);

impl Quarto {
    pub fn new(player_one: QuartoAgent, player_two: QuartoAgent) -> Self {
        Self {
            player_one,
            player_two,
            state: QuartoGameState::new(),
            is_player_one_turn: true,
            show_console_logs: false,
            log_stats: false,
            num_retries_allowed: 2,
        }
    }

    pub fn reset(&mut self) {
        self.state = QuartoGameState::new();
        self.is_player_one_turn = true;
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

    pub fn try_make_move(&mut self) -> bool {
        let mut retry = 0;
        while retry < self.num_retries_allowed {
            let player_move = match self.is_player_one_turn {
                true => self
                    .player_one
                    .make_move(self.get_current_state(), self.show_console_logs),
                false => self
                    .player_two
                    .make_move(self.get_current_state(), self.show_console_logs),
            };
            let QuartoMove(position, next_piece) = player_move;

            if self.is_valid_move(position, next_piece) {
                qutils::update_state(&mut self.state, position, next_piece);
                return true;
            } else {
                retry += 1;
            }
        }

        println!("Maximum retries exceeded - game over");
        return false;
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
            match self.is_player_one_turn {
                true => println!("\nPlayer 1 ({}) won!", self.player_one.name()),
                false => println!("\nPlayer 2 ({}) won!", self.player_two.name()),
            }
            return true;
        }
        return false;
    }

    pub fn run(&mut self) -> GameResult {
        self.make_first_move(self.get_random_piece());
        self.display_state();

        for _ in 1..16 {
            if !self.try_make_move() {
                return match self.is_player_one_turn {
                    true => GameResult::PlayerOneInvalid,
                    false => GameResult::PlayerTwoInvalid,
                };
            }
            self.display_state();
            if self.is_game_over() {
                return match self.is_player_one_turn {
                    true => GameResult::PlayerOneWon,
                    false => GameResult::PlayerTwoWon,
                };
            }
            self.is_player_one_turn = !self.is_player_one_turn;
        }

        self.make_last_move();
        self.display_state();
        if self.is_game_over() {
            return match self.is_player_one_turn {
                true => GameResult::PlayerOneWon,
                false => GameResult::PlayerTwoWon,
            };
        } else {
            println!("\nDraw!");
            return GameResult::Draw;
        }
    }
}

//setters 
impl Quarto {
    pub fn with_console_logs(&mut self) -> &mut Self {
        self.show_console_logs = true;
        self
    }

    pub fn with_file_logs(&mut self) -> &mut Self {
        self.log_stats = true;
        self
    }

    pub fn set_num_retries(&mut self, num_retries: u8) -> &mut Self {
        self.num_retries_allowed = num_retries;
        self
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
