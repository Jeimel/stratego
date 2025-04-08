use super::{iteration, node::Node, UCT};
use crate::stratego::{Move, StrategoState};
use std::rc::Rc;

pub struct ISMCTS {
    iterations: usize,
}

impl UCT for ISMCTS {}

impl ISMCTS {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        let root = Node::new();

        for _ in 0..self.iterations {
            let mut det = pos.determination();
            let node = Rc::clone(&root);

            iteration::execute_one(&mut det, node, self);
        }

        let mut children: Vec<_> = root.children().collect();

        children.sort_by_key(|c| c.visits());
        for c in children {
            println!(
                "info move {} visits {} reward {} availability {}",
                c.mov.unwrap(),
                c.visits(),
                c.reward(),
                c.availability(),
            );
        }

        root.max_visits()
            .map(|c| c.mov.unwrap_or_default())
            .unwrap_or_default()
    }
}
