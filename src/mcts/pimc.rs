use super::{node::NodeStats, MCTS};
use crate::{
    deployment::Deployment,
    policy::Policy,
    select::Select,
    stratego::{Move, StrategoState},
    value::Value,
};
use std::collections::HashMap;

pub struct PIMC {
    determinizations: usize,
    iterations: usize,
    value: Value,
    policy: Policy,
    select: Select,
    deployment: Deployment,
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
            let mut search = MCTS::new(
                self.iterations,
                self.value,
                self.policy,
                self.select,
                self.deployment,
            );
            let mut det = pos.determination();

            search.run(&mut det);

            search.root().children().for_each(|c| {
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
        (self.deployment)()
    }
}
