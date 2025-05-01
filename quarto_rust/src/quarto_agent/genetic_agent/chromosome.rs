use crate::quarto::{quarto_game_state::QuartoGameState, QuartoMove};

#[derive(Eq, Hash, PartialEq, Clone)]
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
        Chromosome { movepath: Vec::new() }
    }

    pub fn mutation(&mut self, state: &QuartoGameState) {}

    pub fn is_valid(&self, state: &QuartoGameState) -> bool {
        true
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct ChromosomeId(pub u16);
