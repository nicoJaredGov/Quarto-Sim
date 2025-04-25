use std::collections::HashMap;

use super::chromosome::Chromosome;
use crate::quarto::QuartoMove;

pub struct RootNode<'a, T> {
    pub value: i32,
    pub children: Vec<&'a Node<'a, T>>,
}
pub struct Node<'a, T> {
    pub state: &'a T,
    pub value: i32,
    pub parent: &'a Node<'a, T>,
    pub children: Vec<&'a Node<'a, T>>,
}

pub struct LeafNode<'a, T> {
    pub state: &'a QuartoMove,
    pub value: i32,
    pub parent: &'a Node<'a, T>,
}

pub struct ReservationTree<'a, T> {
    root: RootNode<'a, T>,
    leaf_nodes: HashMap<&'a Chromosome, &'a LeafNode<'a, T>>,
}

impl<'a, T> ReservationTree<'a, T> {
    pub fn new() -> Self {
        Self {
            root: RootNode {
                value: -10,
                children: Vec::new(),
            },
            leaf_nodes: HashMap::new(),
        }
    }

    pub fn add_path(&mut self, chromosome: &Chromosome, evalutation: i32) {
        
    }
}
