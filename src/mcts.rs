mod ismcts;
mod iteration;
mod mcts;
mod node;
mod pimc;

pub use ismcts::ISMCTS;
pub use mcts::MCTS;
pub use node::{Node, NodeStats};
pub use pimc::PIMC;
use rand::distr::weighted::WeightedIndex;

use crate::stratego::{Move, StrategoState};
use std::sync::Arc;

pub trait Search {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Arc<Node>>;

    fn value(&self, pos: &mut StrategoState) -> f32;

    fn policy(&self, pos: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32>;
}
