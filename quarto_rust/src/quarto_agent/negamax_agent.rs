use itertools::Itertools;
use rand::Rng;
use std::{cmp, i32};

use super::Agent;
use crate::utils as qutils;
use crate::{quarto::QuartoMove, quarto_agent::QuartoGameState};

const MIN_EVAL: i32 = -1000;

pub struct NegamaxAgent {
    search_depth: u8,
    search_window: u8,
}

impl NegamaxAgent {
    pub fn new(search_depth: u8, search_window: u8) -> NegamaxAgent {
        NegamaxAgent {
            search_depth,
            search_window,
        }
    }
}

impl Agent for NegamaxAgent {
    fn make_first_move(&self) -> u8 {
        rand::rng().random_range(0..16)
    }
    fn make_move(&self, state: QuartoGameState) -> QuartoMove {
        alpha_beta(state, self.search_depth, self.search_window, -500, 500).1
    }
    fn get_name(&self) -> String {
        format!("NegamaxAgent-{}-{}", self.search_depth, self.search_window)
    }
}

fn alpha_beta(
    mut state: QuartoGameState,
    depth: u8,
    search_window: u8,
    mut alpha: i32,
    beta: i32,
) -> (i32, QuartoMove) {
    if qutils::is_game_over(&state.board) {
        return (MIN_EVAL, QuartoMove(16, 16));
    }
    if depth == 0 || state.available_positions.len() == 0 {
        return (evaluation(state.board), QuartoMove(16, 16));
    }

    if state.available_pieces.len() == 0 {
        state.available_pieces.insert(16);
    }
    let mut max_score = MIN_EVAL;
    let mut best_move = QuartoMove(16, 16);

    let mut search_window_counter: u8 = 0;
    let possible_moves = state
        .available_pieces
        .iter()
        .cartesian_product(state.available_positions.iter());
    for p in possible_moves {
        search_window_counter += 1;
        if search_window_counter > search_window {
            break;
        }

        //simulate move
        let (next_piece, position) = (*p.0, *p.1);
        let mut next_state = QuartoGameState {
            available_pieces: state.available_pieces.clone(),
            available_positions: state.available_positions.clone(),
            ..state
        };
        qutils::update_state(&mut next_state, position, next_piece);

        //call for next turn
        let curr_eval = -alpha_beta(next_state, depth - 1, search_window, -beta, -alpha).0;
        if curr_eval >= max_score {
            max_score = curr_eval;
            best_move.0 = position;
            best_move.1 = next_piece;
        }

        alpha = cmp::max(alpha, max_score);
        if alpha > beta {
            state.available_pieces.remove(&16);
            return (alpha, best_move);
        }
    }

    state.available_pieces.remove(&16);
    return (max_score, best_move);
}

//counts how many lines of three pieces with an identical property
fn evaluation(board: [[u8; 4]; 4]) -> i32 {
    let mut num_lines: i32 = 0;
    let mut diag1 = Vec::new();
    let mut diag2 = Vec::new();

    for i in 0..4 {
        //check horizontal lines
        let row = board[i].iter().cloned().filter(|&x| x != 16).collect_vec();
        if row.len() == 3 && qutils::matching_property_exists(&row) {
            num_lines += 1
        }
        //check vertical lines
        let col = board.map(|row| row[i]);
        let col = col.iter().cloned().filter(|&x| x != 16).collect_vec();
        if col.len() == 3 && qutils::matching_property_exists(&col) {
            num_lines += 1
        }
        //fill in diagonals
        if board[i][i] != 16 {
            diag1.push(board[i][i]);
        }
        if board[i][3 - i] != 16 {
            diag2.push(board[i][3 - i]);
        }
    }

    //check obtuse diagonal line
    if diag1.len() == 3 && qutils::matching_property_exists(&diag1) {
        num_lines += 1
    }
    if diag2.len() == 3 && qutils::matching_property_exists(&diag2) {
        num_lines += 1
    }

    num_lines
}
