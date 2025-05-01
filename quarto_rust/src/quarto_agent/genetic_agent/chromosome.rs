use rand::{seq::{IndexedMutRandom, IteratorRandom}, Rng};

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

    pub fn crossover(&self, other: &Chromosome) -> Self {
        let mut num_moves = self.movepath.len();
        if other.movepath.len() < num_moves {
            num_moves = other.movepath.len();
        }
        if num_moves < 2 {
            return self.clone();
        }

        let point = rand::rng().random_range(1..num_moves);
        let mut new_path = self.movepath[..point].to_vec();
        new_path.extend_from_slice(&other.movepath[point..]);

        println!("cross at {point}  {:?}", new_path);

        Chromosome {
            movepath: new_path,
        }
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
        true
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct ChromosomeId(pub u16);
