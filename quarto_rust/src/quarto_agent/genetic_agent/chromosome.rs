use rand::seq::{IndexedMutRandom, IteratorRandom};

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

    pub fn crossover(&self, other: &Chromosome) -> Chromosome {
        Chromosome {
            movepath: Vec::new(),
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
