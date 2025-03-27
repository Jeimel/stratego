use super::{iteration, node::Node, UCB};
use crate::stratego::{Move, StrategoState};
use std::rc::Rc;

pub struct ISMCTS {
    root: Rc<Node>,
    iterations: usize,
}

impl UCB for ISMCTS {}

impl ISMCTS {
    pub fn new(iterations: usize) -> Self {
        Self {
            root: Node::new(),
            iterations,
        }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        self.root = Node::new();

        for _ in 0..self.iterations {
            let mut det = pos.determination();
            let node = Rc::clone(&self.root);

            iteration::execute_one(&mut det, node, self);
        }

        let mut children: Vec<_> = self.root.children().collect();

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

        self.root
            .max_visits()
            .map(|c| c.mov.unwrap_or_default())
            .unwrap_or_default()
    }
}
