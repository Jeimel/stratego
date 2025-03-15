use crate::stratego::{GameState, MoveStack, Position};
use rand::seq::IteratorRandom;

mod iteration;
mod node;
mod search;

pub use search::Search;

impl Position {
    pub fn rollout(&mut self, stack: &mut MoveStack) -> f32 {
        let mut rng = rand::rng();

        let stm = self.stm();

        let mut moves = Vec::new();
        while !self.game_over() {
            let mov = self.gen(&stack).iter().choose(&mut rng);

            if let Some(m) = mov {
                moves.push(m);
                self.make(&m);
                stack.push(self.hash());
            } else {
                // Handle two-squares and more-squares rule
                self.set_game_state(GameState::Loss);
            }
        }

        let current = f32::from(stm == self.stm());
        match self.game_state() {
            GameState::Draw => 0.0,
            GameState::Win => 1.0 + (-2.0 * current),
            GameState::Loss => -1.0 + (2.0 * current),
            GameState::Ongoing => unreachable!(),
        }
    }
}
