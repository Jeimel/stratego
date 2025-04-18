use super::{iteration, node::Node, Search};
use crate::{
    policy::Policy,
    select::Select,
    stratego::{Move, StrategoState},
    value::Value,
};
use ordered_float::OrderedFloat;
use rand::distr::weighted::WeightedIndex;
use std::sync::Arc;

pub struct ISMCTS<const MULTIPLE: bool> {
    iterations: usize,
    value: Value,
    policy: Policy,
    select: Select,
}

impl<const MULTIPLE: bool> Search for ISMCTS<MULTIPLE> {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Arc<Node>> {
        let children = node.children();

        let legal: Vec<_> = children
            .filter(|c| moves.iter().any(|m| c.mov().unwrap() == *m))
            .collect();

        let choice = legal
            .iter()
            .max_by_key(|c| OrderedFloat::from((self.select)(c)))
            .cloned();

        legal.iter().for_each(|c| c.stats_mut().availability += 1);

        choice
    }

    fn value(&self, pos: &mut StrategoState) -> f32 {
        (self.value)(pos)
    }

    fn policy(&self, pos: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32> {
        (self.policy)(pos, moves)
    }
}

impl<const MULTIPLE: bool> ISMCTS<MULTIPLE> {
    pub fn new(iterations: usize, value: Value, policy: Policy, select: Select) -> Self {
        Self {
            iterations,
            value,
            policy,
            select,
        }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        let root = Node::new();

        for _ in 0..self.iterations {
            let mut det = pos.determination();
            let node = Arc::clone(&root);

            iteration::execute_one::<ISMCTS<MULTIPLE>, MULTIPLE>(&mut det, node, self);
        }

        #[cfg(feature = "info")]
        {
            let mut children: Vec<_> = root.children().collect();
            children.sort_by_key(|c| c.stats().visits);

            for c in children {
                let stats = c.stats();

                println!(
                    "info move {} visits {} reward {} availability {}",
                    c.mov().unwrap(),
                    stats.visits,
                    stats.reward,
                    stats.availability,
                );
            }
        }

        root.max_visits().unwrap().mov().unwrap()
    }
}
