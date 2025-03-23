use super::mcts::MCTS;
use crate::stratego::{Move, StrategoState};

pub struct ISMCTS {
    mcts: MCTS,
    determinizations: usize,
}

impl ISMCTS {
    pub fn new(iterations: usize, determinizations: usize) -> Self {
        Self {
            mcts: MCTS::new(iterations / determinizations),
            determinizations,
        }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        // Reset current position, so no subtree is used
        self.mcts.set_pos(None);
        self.mcts.set_root(&pos);

        for _ in 0..self.determinizations {
            let det = pos.determination();
            self.mcts.run(&det);
        }

        self.mcts
            .max_visits()
            .map(|c| c.mov.unwrap_or_default())
            .unwrap_or_default()
    }
}
