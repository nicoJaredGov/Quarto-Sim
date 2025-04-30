use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::{chromosome::Chromosome, node::Node};

pub struct ReservationTree<'a> {
    root: Rc<Node>,
    leaf_nodes: HashMap<&'a Chromosome, Rc<Node>>,
    unique_evals: HashSet<i32>,
}

impl<'a> ReservationTree<'a> {
    pub fn new() -> Self {
        Self {
            root: Node::new(),
            leaf_nodes: HashMap::new(),
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
                },
                None => break,
            }
        }
    }

    pub fn add_path(&mut self, chromosome: &'a Chromosome, evaluation: i32) {
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
        self.unique_evals.insert(evaluation);
        self.minmax(Rc::clone(&current_node));
        self.leaf_nodes.insert(chromosome, Rc::clone(&current_node));
    }

    pub fn compute_fitness(&mut self, chromosome: &'a Chromosome) -> u8 {
        let leaf_node = self.leaf_nodes.get(chromosome);
        let mut fitness: u8 = 0;

        if let Some(leaf) = leaf_node {
            let leaf_value = *leaf.value.borrow();
            let mut current_node = Rc::clone(leaf);

            loop {
                let parent = current_node.parent.borrow().upgrade();
                match parent {
                    Some(next_node) => {
                        if next_node.value.borrow().clone() != leaf_value {
                            fitness = 16 - current_node.depth;
                            break;
                        }
                        current_node = Rc::clone(&next_node);
                        
                    },
                    None => break,
                }
            }
        } 

        fitness
    }   
}
