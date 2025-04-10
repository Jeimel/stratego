use super::{node::NodeStats, MCTS, UCT};
use crate::stratego::{Move, StrategoState};
use std::collections::HashMap;

pub struct PIMC {
    determinizations: usize,
    iterations: usize,
}

impl UCT for PIMC {}

impl PIMC {
    pub fn new(determinizations: usize, iterations: usize) -> Self {
        Self {
            determinizations,
            iterations,
        }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        let mut root: HashMap<Move, NodeStats> = HashMap::new();

        for _ in 0..self.determinizations {
            let mut search = MCTS::new(self.iterations);
            let mut det = pos.determination();

            search.run(&mut det);

            search.root().children().for_each(|c| {
                let stats = c.stats();
                let entry = root.entry(c.mov().unwrap()).or_insert(NodeStats::default());

                entry.visits += stats.visits;
                entry.reward += stats.reward;
            })
        }

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
}
