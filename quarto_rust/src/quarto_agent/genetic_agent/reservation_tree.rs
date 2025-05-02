use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
use itertools::Itertools;

use super::{
    chromosome::{Chromosome, ChromosomeId},
    node::Node,
};

pub struct ReservationTree {
    root: Rc<Node>,
    unique_evals: HashSet<i32>,
}

impl ReservationTree {
    pub fn new() -> Self {
        Self {
            root: Node::new(),
            unique_evals: HashSet::new(),
        }
    }

    fn minmax(&mut self, leaf: Rc<Node>) {
        let mut current_node = leaf;
        loop {
            let parent = current_node.parent.borrow().upgrade();
            match parent {
                Some(next_node) => {
                    current_node = Rc::clone(&next_node);
                    current_node.calculate_value();
                }
                None => break,
            }
        }
    }

    pub fn add_path(
        &mut self,
        chromosome_id: ChromosomeId,
        evaluation: i32,
        chromosomes: &HashMap<ChromosomeId, Chromosome>,
    ) {
        let chromosome = chromosomes.get(&chromosome_id).unwrap();
        let movepath = chromosome.get_movepath();
        let mut current_node = Rc::clone(&self.root);
        let mut end_found = false;

        for quarto_move in movepath {
            //traverse existing path
            if !end_found {
                let node = current_node.children.borrow().get(quarto_move).cloned();
                match node {
                    Some(next_node) => current_node = Rc::clone(&next_node),
                    None => end_found = true,
                }
            }
            //extend path
            if end_found {
                let depth = current_node.depth + 1;
                current_node.add_child(quarto_move.clone(), evaluation, depth);
                let node = current_node.children.borrow().get(quarto_move).cloned();
                current_node = node.unwrap();
            }
        }

        current_node.associate_chromosome(chromosome_id.clone());
        self.unique_evals.insert(evaluation);
        self.minmax(Rc::clone(&current_node));
    }

    fn recursive_fitness(
        &self,
        fitness: &mut HashMap<ChromosomeId, i32>,
        count_limit: usize,
        node: Rc<Node>,
        evaluation: i32,
    ) {
        if fitness.len() >= count_limit {
            return;
        }
        let chromosome_id = (*node.chromosome_id.borrow()).clone();
        if let Some(chromosome_id) = chromosome_id {
            fitness.insert(chromosome_id, evaluation);
        } else {
            for child in node
                .children
                .borrow()
                .values()
                .filter(|node| *node.value.borrow() == evaluation)
            {
                self.recursive_fitness(fitness, count_limit, Rc::clone(child), evaluation);
            }
        }
    }

    pub fn update_fitness(&self, fitness: &mut HashMap<ChromosomeId, i32>, count_limit: usize) {
        fitness.clear();
        for eval in self.unique_evals.iter().sorted().rev() {
            self.recursive_fitness(fitness, count_limit, Rc::clone(&self.root), *eval);
        }
    }
}
