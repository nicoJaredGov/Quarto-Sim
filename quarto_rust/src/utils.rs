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

//Determines if there is a matching column of bits for a list of integers between 0 (inclusive) and 16 (exclusive)
pub fn matching_property_exists(line: &[u8; 4]) -> bool {
    //bitwiseAnd - checks if there is a column of 1s by getting the conjunction
    //bitwiseNot - checks if there is a column of 0s after negating all integers, masking by 15 (1111) and then getting the conjuction
    let bitwise_and = line.iter().fold(1, |acc, item| acc & item);
    let bitwise_not = line.iter().fold(1, |acc, item| acc & (!item & 15));
    let result = bitwise_and | bitwise_not;
    result > 0
}

pub fn is_game_over(board: &[[u8; 4]; 4]) -> bool {
    let mut diag1 = [0u8; 4];
    let mut diag2 = [0u8; 4];

    for i in 0..4 {
        //check horizontal lines
        if matching_property_exists(&board[i]) && board[i][0] != 16 {
            return true;
        }
        //check vertical lines
        let col: [u8; 4] = board.map(|row| row[i]);
        if matching_property_exists(&col) && col[0] != 16 {
            return true;
        }
        //fill in diagonals
        diag1[i] = board[i][i];
        diag2[i] = board[i][3 - i];
    }

    //check obtuse diagonal line
    if matching_property_exists(&diag1) && diag1[0] != 16 {
        return true;
    }
    //check acute diagonal line:
    if matching_property_exists(&diag2) && diag2[0] != 16 {
        return true;
    }

    //no winning line found
    false
}
