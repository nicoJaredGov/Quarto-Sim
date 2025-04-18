use std::collections::HashSet;

const NUM_PIECES: u8 = 16;

pub struct QuartoGameState {
    pub board: [[u8; 4]; 4],
    pub current_piece: u8,
    pub available_pieces: HashSet<u8>,
    pub available_positions: HashSet<u8>,
}

impl QuartoGameState {
    pub fn new() -> Self {
        Self {
            board: [[16u8; 4]; 4],
            current_piece: 16,
            available_pieces: (0..NUM_PIECES).collect(),
            available_positions: (0..NUM_PIECES).collect(),
        }
    }
}