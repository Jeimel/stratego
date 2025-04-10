use super::{iteration, node::Node, Search};
use crate::stratego::{GameState, Move, StrategoState};
use ordered_float::OrderedFloat;
use rand::{rng, seq::IteratorRandom};
use std::rc::Rc;

pub struct ISMCTS {
    iterations: usize,
}

impl Search for ISMCTS {
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
                let v = C * ((stats.availability as f32).ln() / stats.visits as f32).sqrt();

                OrderedFloat::from(u + v)
            })
            .cloned();

        legal.iter().for_each(|c| c.stats_mut().availability += 1);

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
