pub fn convert_move_to_str(game_move: u8) -> String {
    if game_move <= 9 {
        let mut str_move = String::from("0");
        str_move.push_str(&game_move.to_string());
        str_move
    } else {
        game_move.to_string()
    }
}

// fn encodeBoard(boardArray, currentPiece):
//     encoding = ""
//     for i in range(4):
//         for j in range(4):
//             encoding += convertIntMoveToStr(boardArray[i][j])

//     encoding += convertIntMoveToStr(currentPiece)
//     return encoding

// fn decodeBoard(encoding):
//     board_array = [int(encoding[i] + encoding[i + 1]) for i in range(0, len(encoding) - 2, 2)]
//     board_array = np.reshape(board_array, (4, 4))

//     current_piece = int(encoding[-2:])

//     return board_array, current_piece

// fn getEncodingAfterMove(currentEncoding: str, nextPosition: int, nextPiece: int):
//     return (
//         currentEncoding[: 2 * nextPosition]
//         + currentEncoding[-2:]
//         + currentEncoding[2 * nextPosition + 2 : -2]
//         + convertIntMoveToStr(nextPiece)
//     )

pub fn get_2d_coords(index: u8) -> (u8, u8) {
    (index / 4, index % 4)
}

pub fn get_linear_coords(row: u8, col: u8) -> u8 {
    4 * row + col
}

// # Determines if there is a matching column of bits for a list of integers between 0 (inclusive) and 16 (exclusive)
// fn matchingPropertyExists(line):
//     # bitwiseAnd - checks if there is a column of 1s by getting the conjunction
//     # bitwiseNot - checks if there is a column of 0s after negating all integers, masking by 15 (1111) and then getting the conjuction
//     bitwiseAnd = line[0]
//     bitwiseNot = ~line[0] & 15
//     for i in range(1, len(line)):
//         bitwiseAnd &= line[i]
//         bitwiseNot &= ~line[i] & 15

//     result = bitwiseAnd | bitwiseNot
//     return result > 0

// fn isGameOver(board):
//     for i in range(4):
//         # check horizontal lines
//         if np.count_nonzero(board[i] == 16) == 0:
//             if matchingPropertyExists(board[i]):
//                 return True

//         # check vertical lines
//         if np.count_nonzero(board[:, i] == 16) == 0:
//             if matchingPropertyExists(board[:, i]):
//                 return True

//     # check obtuse diagonal line
//     if np.count_nonzero(np.diag(board) == 16) == 0:
//         if matchingPropertyExists(np.diag(board)):
//             return True

//     # check acute diagonal line:
//     if np.count_nonzero(np.diag(board[::-1]) == 16) == 0:
//         if matchingPropertyExists(np.diag(board[::-1])):
//             return True

//     # no winning line found
//     return False

// fn isGameOverEncoding(encoding):
//     board = [int(encoding[i] + encoding[i + 1]) for i in range(0, len(encoding) - 2, 2)]

//     for i in range(4):
//         # check horizontal lines
//         horizontal = board[4 * i : 4 * (i + 1)]
//         if 16 not in horizontal:
//             if matchingPropertyExists(horizontal):
//                 return True

//         # check vertical lines
//         vertical = board[i : len(board) : 4]
//         if 16 not in vertical:
//             if matchingPropertyExists(vertical):
//                 return True

//     # check obtuse diagonal line
//     diagonal1 = board[0 : len(board) : 5]
//     if 16 not in diagonal1:
//         if matchingPropertyExists(diagonal1):
//             return True

//     # check acute diagonal line:
//     diagonal2 = board[3:-1:3]
//     if 16 not in diagonal2:
//         if matchingPropertyExists(diagonal2):
//             return True

//     return False