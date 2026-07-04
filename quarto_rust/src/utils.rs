use chrono::{Timelike, Utc};
use std::fs::File;

use crate::quarto::quarto_game_state::QuartoGameState;
use itertools::Itertools;

pub fn convert_move_to_str(game_move: u8) -> String {
    if game_move <= 9 {
        let mut str_move = String::from("0");
        str_move.push_str(&game_move.to_string());
        str_move
    } else {
        game_move.to_string()
    }
}

pub fn get_2d_coords(index: u8) -> (u8, u8) {
    (index / 4, index % 4)
}

pub fn get_linear_coords(row: u8, col: u8) -> u8 {
    4 * row + col
}

pub fn update_state(current_state: &mut QuartoGameState, position: u8, next_piece: u8) {
    let (row, col) = get_2d_coords(position);
    current_state.board[row as usize][col as usize] = current_state.current_piece;
    current_state.available_positions.remove(&position);
    current_state.current_piece = next_piece;
    current_state
        .available_pieces
        .remove(&current_state.current_piece);
}

pub fn undo_state_update(current_state: &mut QuartoGameState, position: u8, next_piece: u8) {
    let (row, col) = get_2d_coords(position);
    current_state.current_piece = current_state.board[row as usize][col as usize];
    current_state.board[row as usize][col as usize] = 16;
    current_state.available_positions.insert(position);
    current_state.available_pieces.insert(next_piece);
}

//Determines if there is a matching column of bits for a list of integers between 0 (inclusive) and 16 (exclusive)
pub fn matching_property_exists(line: &[u8]) -> bool {
    //bitwiseAnd - checks if there is a column of 1s by getting the conjunction
    //bitwiseNot - checks if there is a column of 0s after negating all integers, masking by 15 (1111) and then getting the conjuction
    let bitwise_and = line.iter().fold(15, |acc, item| acc & item);
    let bitwise_not = line.iter().fold(15, |acc, item| acc & (!item & 15));
    let result = bitwise_and | bitwise_not;
    result > 0
}

pub fn is_game_over(board: &[[u8; 4]; 4]) -> bool {
    let mut diag1 = [0u8; 4];
    let mut diag2 = [0u8; 4];

    for i in 0..4 {
        //check horizontal lines
        let line_has_empty_space = board[i].iter().any(|&x| x == 16);
        if !line_has_empty_space && matching_property_exists(&board[i]) {
            return true;
        }
        //check vertical lines
        let col: [u8; 4] = board.map(|row| row[i]);
        let line_has_empty_space = col.iter().any(|&x| x == 16);
        if !line_has_empty_space && matching_property_exists(&col) {
            return true;
        }
        //fill in diagonals
        diag1[i] = board[i][i];
        diag2[i] = board[i][3 - i];
    }

    //check obtuse diagonal line
    let line_has_empty_space = diag1.iter().any(|&x| x == 16);
    if !line_has_empty_space && matching_property_exists(&diag1) {
        return true;
    }
    //check acute diagonal line:
    let line_has_empty_space = diag2.iter().any(|&x| x == 16);
    if !line_has_empty_space && matching_property_exists(&diag2) {
        return true;
    }

    //no winning line found
    false
}

//counts how many lines of three pieces with an identical property
pub fn line_evaluation(board: [[u8; 4]; 4]) -> i32 {
    let mut num_lines: i32 = 0;
    let mut diag1 = Vec::new();
    let mut diag2 = Vec::new();

    for i in 0..4 {
        //check horizontal lines
        let row = board[i].iter().cloned().filter(|&x| x != 16).collect_vec();
        if row.len() == 3 && matching_property_exists(&row) {
            num_lines += 1
        }
        //check vertical lines
        let col = board.map(|row| row[i]);
        let col = col.iter().cloned().filter(|&x| x != 16).collect_vec();
        if col.len() == 3 && matching_property_exists(&col) {
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
    if diag1.len() == 3 && matching_property_exists(&diag1) {
        num_lines += 1
    }
    if diag2.len() == 3 && matching_property_exists(&diag2) {
        num_lines += 1
    }

    num_lines
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn max_possible_states(num_moves: u64, search_depth: u64) -> u64 {
    (num_moves * factorial(num_moves - 1).pow(2))
        / ((num_moves - search_depth) * factorial(num_moves - search_depth - 1).pow(2))
}

pub fn create_timestamped_log_file(
    dir_path: &str,
    player_one_name: &str,
    player_two_name: &str,
) -> File {
    let utc = Utc::now().with_nanosecond(0).unwrap();
    let file_datetime = utc.format("%Y-%m-%d_%H-%M-%S").to_string();

    let filename = format!(
        "{}{} {}_{}.csv",
        dir_path, file_datetime, player_one_name, player_two_name
    );

    File::create(filename).expect("Error creating log file")
}
