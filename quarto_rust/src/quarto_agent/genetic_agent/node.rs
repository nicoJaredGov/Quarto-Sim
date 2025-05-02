use super::chromosome::ChromosomeId;
use crate::quarto::quarto_move::QuartoMove;

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

pub struct Node {
    pub value: RefCell<i32>,
    pub depth: u8,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<HashMap<QuartoMove, Rc<Node>>>,
    pub chromosome_id: RefCell<Option<ChromosomeId>>,
}

impl Node {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            value: RefCell::new(0),
            depth: 0,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
            chromosome_id: RefCell::new(None),
        })
    }

    pub fn associate_chromosome(self: &Rc<Self>, chromosome_id: ChromosomeId) {
        *self.chromosome_id.borrow_mut() = Some(chromosome_id);
    }

    pub fn add_child(self: &Rc<Self>, quarto_move: QuartoMove, evaluation: i32, depth: u8) {
        let child = Rc::new(Node {
            value: RefCell::new(evaluation),
            depth,
            parent: RefCell::new(Rc::downgrade(self)),
            children: RefCell::new(HashMap::new()),
            chromosome_id: RefCell::new(None),
        });

        self.children.borrow_mut().insert(quarto_move, child);
    }

    pub fn calculate_value(self: &Rc<Self>) {
        let is_max = self.depth % 2 == 0;
        let children_ref = self.children.borrow_mut();
        if children_ref.is_empty() {
            return;
        }

        let mut val_ref = self.value.borrow_mut();
        match is_max {
            true => {
                *val_ref = *children_ref
                    .values()
                    .max_by_key(|x| x.value.clone())
                    .unwrap()
                    .value
                    .borrow()
            }
            false => {
                *val_ref = *children_ref
                    .values()
                    .min_by_key(|x| x.value.clone())
                    .unwrap()
                    .value
                    .borrow()
            }
        }
    }
}
