mod ismcts;
mod iteration;
mod mcts;
mod node;

pub use ismcts::ISMCTS;
pub use mcts::MCTS;

use crate::stratego::{GameState, Move, StrategoState};
use node::Node;
use ordered_float::OrderedFloat;
use rand::{rng, seq::IteratorRandom};
use std::rc::Rc;

pub trait Search {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Rc<Node>>;

    fn value(&self, pos: &mut StrategoState) -> f32;
}

trait UCB {}

impl<T: UCB> Search for T {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Rc<Node>> {
        let children = node.children();

        let legal: Vec<_> = children
            .filter(|c| moves.iter().any(|m| c.mov.as_ref().unwrap() == m))
            .collect();

        let choice = legal
            .iter()
            .max_by_key(|c| {
                let u = c.reward() / c.visits() as f32;
                let v = 0.7 * ((c.availability() as f32).ln() / c.visits() as f32).sqrt();

                OrderedFloat::from(u + v)
            })
            .cloned();

        legal.iter().for_each(|c| *c.availability.borrow_mut() += 1);

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
