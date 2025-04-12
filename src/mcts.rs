mod ismcts;
mod iteration;
mod mcts;
mod node;
mod pimc;

pub use ismcts::ISMCTS;
pub use mcts::MCTS;
pub use pimc::PIMC;

use crate::stratego::{GameState, Move, StrategoState};
use node::Node;
use ordered_float::OrderedFloat;
use rand::{rng, seq::IteratorRandom};
use std::rc::Rc;

pub trait Search {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Rc<Node>>;

    fn value(&self, pos: &mut StrategoState) -> f32;
}

trait UCT {}

impl<T: UCT> Search for T {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Rc<Node>> {
        const C: f32 = 0.7;

        let children = node.children();

        let legal: Vec<_> = children
            .filter(|c| moves.iter().any(|m| c.mov().unwrap() == *m))
            .collect();

        let choice = legal
            .iter()
            .max_by_key(|c| {
                let stats = c.stats();

                let u = stats.reward / stats.visits as f32;
                let v = C * ((c.parent_visits() as f32).ln() / stats.visits as f32).sqrt();

                OrderedFloat::from(u + v)
            })
            .cloned();

        choice
    }

    fn value(&self, pos: &mut StrategoState) -> f32 {
        let mut rng = rng();

        let stm = pos.stm();
        while !pos.game_over() {
            let mov = pos.gen().iter().choose(&mut rng);

            if let Some(mov) = mov {
                pos.make(mov);
            } else {
                // Handle two-squares and more-squares rule
                pos.set_game_state(GameState::Loss);
            }
        }

        let current = f32::from(stm == pos.stm());
        match pos.game_state() {
            GameState::Draw => 0.0,
            GameState::Win => -1.0 + (2.0 * current),
            GameState::Loss => 1.0 + (-2.0 * current),
            GameState::Ongoing => unreachable!(),
        }
    }
}
