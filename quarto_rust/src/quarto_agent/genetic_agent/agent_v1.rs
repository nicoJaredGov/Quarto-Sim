use rand::seq::IteratorRandom;
use rand::{Rng, seq::SliceRandom};
use std::collections::HashMap;

use super::chromosome::Chromosome;
use super::{
    agent_config::GeneticMinmaxConfig, chromosome::ChromosomeId, reservation_tree::ReservationTree,
};
use crate::utils as qutils;
use crate::{
    quarto::quarto_move::QuartoMove,
    quarto_agent::{Agent, QuartoGameState},
};

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
    let mut fitness: HashMap<ChromosomeId, i32> = HashMap::new();

    //randomize initial population
    for id in 0..config.initial_population_size {
        let chromosome = gen_random_chromosome(&state, config.search_depth);
        let chromosome_id = ChromosomeId(id);
        add_chromosome(
            chromosome,
            &mut chromosomes,
            &mut fitness,
            &mut reservation_tree,
            &state,
            chromosome_id,
        );
    }

    //genetic evolution
    let mut best_chromosome_id = ChromosomeId(0);
    for _ in 0..config.max_generations {
        //fitness contains reference to chromosomes in the current gen
        let parents_len = fitness.len() as u16;
        if parents_len < 2 {
            break;
        }
        let num_chromosomes = chromosomes.len() as u16;
        let limit = config.max_population_size - parents_len.max(config.initial_population_size);
        let mut rng = rand::rng();

        //perform mutations and crossovers
        for id in num_chromosomes..(limit + num_chromosomes) {
            //random mutation
            let parent_one = fitness.keys().choose(&mut rng).unwrap();
            let parent_one = chromosomes.get(parent_one).unwrap();
            if rand::random::<f64>() < config.mutation_rate {
                let chromosome_id = ChromosomeId(parents_len + id);
                add_chromosome(
                    parent_one.mutation(&state),
                    &mut chromosomes,
                    &mut fitness,
                    &mut reservation_tree,
                    &state,
                    chromosome_id,
                );
                continue;
            }

            //crossover
            let parent_two = fitness.keys().choose(&mut rng).unwrap();
            let parent_two = chromosomes.get(parent_two).unwrap();
            if parent_one == parent_two {
                continue;
            }
            if rand::random::<f64>() < config.crossover_rate {
                if let Some(crossover_child) = parent_one.crossover(parent_two) {
                    let chromosome_id = ChromosomeId(parents_len + id);
                    add_chromosome(
                        crossover_child,
                        &mut chromosomes,
                        &mut fitness,
                        &mut reservation_tree,
                        &state,
                        chromosome_id,
                    );
                }
            }
        }

        //update fitness for all chromosomes in this generation
        reservation_tree.update_fitness(&mut fitness, config.max_population_size.into());

        //set next generation's initial population to top N chromosomes of current generation
        best_chromosome_id = fitness
            .iter()
            .max_by_key(|entry| entry.1)
            .unwrap()
            .0
            .clone();
    }

    let best_move = chromosomes
        .get(&best_chromosome_id)
        .unwrap()
        .get_movepath()
        .first()
        .unwrap()
        .clone();
    best_move
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

    let movepath: Vec<QuartoMove> = (0..chromosome_length)
        .map(|i| QuartoMove(next_positions[i], next_pieces[i]))
        .collect();
    Chromosome::new(movepath)
}

fn evaluate_chromosome(chromosome: &Chromosome, state: &QuartoGameState) -> i32 {
    let mut temp_state = state.clone();
    let mut my_turn = false;

    for QuartoMove(position, next_piece) in chromosome.get_movepath() {
        my_turn = !my_turn;
        qutils::update_state(&mut temp_state, *position, *next_piece);
        if qutils::is_game_over(&temp_state.board) {
            return if my_turn { 10 } else { -10 };
        }
    }

    let line_eval = qutils::line_evaluation(temp_state.board);
    return if my_turn { -line_eval } else { line_eval };
}

fn add_chromosome(
    chromosome: Chromosome,
    chromosomes: &mut HashMap<ChromosomeId, Chromosome>,
    fitness: &mut HashMap<ChromosomeId, i32>,
    reservation_tree: &mut ReservationTree,
    state: &QuartoGameState,
    chromosome_id: ChromosomeId,
) {
    let evaluation = evaluate_chromosome(&chromosome, &state);
    chromosomes.insert(chromosome_id.clone(), chromosome);
    fitness.entry(chromosome_id.clone()).or_insert(0);
    reservation_tree.add_path(chromosome_id, evaluation, &chromosomes);
}
