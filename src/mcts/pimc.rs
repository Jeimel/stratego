use super::{iteration, node::NodeStats, Node, Search};
use crate::{
    deployment::Deployment,
    policy::Policy,
    select::Select,
    stratego::{Move, StrategoState},
    value::Value,
};
use ordered_float::OrderedFloat;
use rand::distr::weighted::WeightedIndex;
use std::{collections::HashMap, sync::Arc};

pub struct PIMC {
    determinizations: usize,
    iterations: usize,
    value: Value,
    policy: Policy,
    select: Select,
    deployment: Deployment,
}

impl Search for PIMC {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Arc<Node>> {
        let children = node.children();

        let legal: Vec<_> = children
            .filter(|c| moves.iter().any(|m| c.mov().unwrap() == *m))
            .collect();

        let choice = legal
            .iter()
            .max_by_key(|c| OrderedFloat::from(self.select.get(c)))
            .cloned();

        choice
    }

    fn value(&self, pos: &mut StrategoState) -> f32 {
        self.value.get(pos)
    }

    fn policy(&self, pos: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32> {
        self.policy.get(pos, moves)
    }

    fn deployment(&self) -> String {
        self.deployment.get()
    }
}

impl PIMC {
    pub fn new(
        determinizations: usize,
        iterations: usize,
        value: Value,
        policy: Policy,
        select: Select,
        deployment: Deployment,
    ) -> Self {
        Self {
            determinizations,
            iterations,
            value,
            policy,
            select,
            deployment,
        }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        let mut root: HashMap<Move, NodeStats> = HashMap::new();

        for _ in 0..self.determinizations {
            let node = Node::new();
            let det = pos.determination();

            for _ in 0..self.iterations {
                let mut pos = det.clone();
                let node = Arc::clone(&node);

                iteration::execute_one::<PIMC, false>(&mut pos, node, self);
            }

            node.children().for_each(|c| {
                let stats = c.stats();
                let entry = root
                    .entry(c.mov().unwrap())
                    .or_insert(NodeStats::new(0, 0.0));

                entry.visits += stats.visits;
                entry.reward += stats.reward;
            })
        }

        #[allow(unused_mut)]
        let mut children: Vec<_> = root.iter().collect();

        #[cfg(feature = "info")]
        {
            children.sort_by_key(|c| c.1.visits);
            for c in &children {
                let stats = c.1;

                println!(
                    "info move {} visits {} reward {}",
                    c.0, stats.visits, stats.reward,
                );
            }
        }

        *children.iter().max_by_key(|c| c.1.visits).unwrap().0
    }

    pub fn deployment(&self) -> String {
        self.deployment.get()
    }
}
