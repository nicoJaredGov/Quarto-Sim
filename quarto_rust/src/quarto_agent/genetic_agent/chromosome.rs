use rand::{
    Rng,
    seq::{IndexedMutRandom, IteratorRandom},
};
use std::collections::HashSet;

use crate::quarto::{QuartoMove, quarto_game_state::QuartoGameState};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Chromosome {
    movepath: Vec<QuartoMove>,
}

impl Chromosome {
    pub fn new(movepath: Vec<QuartoMove>) -> Self {
        Self { movepath }
    }

    pub fn get_movepath(&self) -> &Vec<QuartoMove> {
        &self.movepath
    }

    pub fn crossover(&self, other: &Chromosome) -> Option<Self> {
        let mut num_moves = self.movepath.len();
        if other.movepath.len() < num_moves {
            num_moves = other.movepath.len();
        }
        if num_moves < 2 {
            return None;
        }

        let point = rand::rng().random_range(1..num_moves);
        let mut new_path = self.movepath[..point].to_vec();
        new_path.extend_from_slice(&other.movepath[point..]);

        println!("cross at {point}  {:?}", new_path);
        Some(Chromosome::new(new_path))
    }

    pub fn mutation(&self, state: &QuartoGameState) -> Self {
        let mut mutated_chromosome = self.clone();

        let mut rng = rand::rng();
        let quarto_move = mutated_chromosome.movepath.choose_mut(&mut rng).unwrap();
        //mutate position
        if rand::random::<f64>() < 0.8 {
            let random_position = state.available_positions.iter().choose(&mut rng);
            quarto_move.0 = random_position.unwrap().clone();
        }
        //mutate next piece
        else {
            let random_piece = state.available_pieces.iter().choose(&mut rng);
            quarto_move.1 = random_piece.unwrap().clone();
        }

        mutated_chromosome
    }

    pub fn is_valid(&self, state: &QuartoGameState) -> bool {
        //invalid if moves repeat
        let positions: HashSet<u8> = self.movepath.iter().map(|q| q.0).collect();
        if positions.len() < self.movepath.len() {
            return false;
        }
        let pieces: HashSet<u8> = self.movepath.iter().map(|q| q.1).collect();
        if pieces.len() < self.movepath.len() {
            return false;
        }

        //invalid if moves are not available in state
        let pos_intersect: HashSet<u8> = positions
            .intersection(&state.available_positions)
            .cloned()
            .collect();
        if pos_intersect.len() < positions.len() {
            return false;
        }
        let piece_intersect: HashSet<u8> = pieces
            .intersection(&state.available_pieces)
            .cloned()
            .collect();
        if piece_intersect.len() < pieces.len() {
            return false;
        }

        return true;
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct ChromosomeId(pub u16);
