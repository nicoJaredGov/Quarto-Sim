use std::collections::HashMap;

use super::chromosome::Chromosome;
use crate::quarto::QuartoMove;

pub struct RootNode<'a> {
    pub value: i32,
    pub children: Vec<&'a Node<'a>>,
}
pub struct Node<'a> {
    pub quarto_move: &'a QuartoMove,
    pub value: i32,
    pub parent: &'a Node<'a>,
    pub children: Vec<&'a Node<'a>>,
}

pub struct LeafNode<'a> {
    pub quarto_move: &'a QuartoMove,
    pub value: i32,
    pub parent: &'a Node<'a>,
}

pub struct ReservationTree<'a> {
    root: RootNode<'a>,
    leaf_nodes: HashMap<&'a Chromosome, &'a LeafNode<'a>>,
}

impl<'a> ReservationTree<'a> {
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
