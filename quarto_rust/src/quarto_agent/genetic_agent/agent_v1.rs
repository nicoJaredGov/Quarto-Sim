use std::collections::HashMap;
use rand::{seq::SliceRandom, Rng};

use crate::{quarto::QuartoMove, quarto_agent::{Agent, QuartoGameState}};
use super::{agent_config::GeneticMinmaxConfig, chromosome::ChromosomeId, reservation_tree::{self, ReservationTree}};
use super::chromosome::Chromosome;
use crate::utils as qutils;

pub struct GeneticMinmaxAgent {
    config: GeneticMinmaxConfig,
}

impl GeneticMinmaxAgent {
    pub fn new(config: GeneticMinmaxConfig) -> Self {
        Self { config }
    }
}

impl Agent for GeneticMinmaxAgent {
    fn make_first_move(&self) -> u8 {
        rand::rng().random_range(0..16)
    }
    fn make_move(&self, state: QuartoGameState) -> QuartoMove {
        generate_sol(&self.config, state)
    }
    fn get_name(&self) -> String {
        format!(
            "Genetic-{}-{}",
            self.config.search_depth, self.config.max_generations
        )
    }
}

fn generate_sol(config: &GeneticMinmaxConfig, state: QuartoGameState) -> QuartoMove {
    //initialization
    let mut reservation_tree = ReservationTree::new();
    let mut chromosomes: HashMap<ChromosomeId, Chromosome> = HashMap::new();
    let mut fitness: HashMap<ChromosomeId, u8> = HashMap::new();

    //randomize initial population
    for id in 0..config.initial_population_size {
        let chromosome = gen_random_chromosome(&state, config.search_depth);
        let evaluation = evaluate_chromosome(&chromosome, &state);
        let chromosome_id = ChromosomeId(id);

        chromosomes.insert(chromosome_id.clone(), chromosome);
        fitness.entry(chromosome_id.clone()).or_insert(0);
        reservation_tree.add_path(chromosome_id.clone(), evaluation, &chromosomes);
    }

    for _ in 0..config.max_generations {
        let parents = chromosomes.keys();
        let parents_len = parents.len() as u16;
        if parents_len < 2 {
            break
        }
        let limit = config.max_population_size - parents_len.max(config.initial_population_size);

        for id in parents_len..limit {
            //random mutation

            //crossover
        }

        //update fitness for all chromosomes in this generation

        //set next generation's initial population to top N chromosomes of current generation
        
    }


    QuartoMove(16, 16)
}

//todo v2: consider making this fast by doing completely random, even invalid moves - then check if valid afterwards
//also consider maintaining two sep vectors in the chromosome
fn gen_random_chromosome(state: &QuartoGameState, search_depth: u8) -> Chromosome {
    let mut next_positions: Vec<u8> = state.available_positions.iter().cloned().collect();
    let mut next_pieces: Vec<u8> = state.available_pieces.iter().cloned().collect();
    let mut chromosome_length: usize = search_depth.into();

    let mut rng = rand::rng();
    next_positions.shuffle(&mut rng);
    next_pieces.shuffle(&mut rng);
    
    if next_pieces.len() < search_depth.into() {
        next_pieces.push(16);
        chromosome_length = state.available_positions.len();
    }

    let mut movepath: Vec<QuartoMove> = Vec::new();
    for i in 0..chromosome_length {
        movepath.push(QuartoMove(next_positions[i], next_pieces[i]));
    }

    Chromosome::new(movepath)
}

fn evaluate_chromosome(chromosome: &Chromosome, state: &QuartoGameState) -> i32 {
    let mut temp_state = state.clone();
    let mut my_turn = true;

    for QuartoMove(position, next_piece) in chromosome.get_movepath() {
        qutils::update_state(&mut temp_state, *position, *next_piece);

        if qutils::is_game_over(&temp_state.board) {
            return if my_turn {10} else {-10};
        }

        my_turn = !my_turn;
    }

    let line_eval = qutils::line_evaluation(temp_state.board);
    return if my_turn {line_eval} else {-line_eval};
}
