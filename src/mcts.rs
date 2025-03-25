mod ismcts;
mod iteration;
mod mcts;
mod node;

pub use ismcts::ISMCTS;
pub use mcts::MCTS;
use node::Node;

use crate::stratego::{Move, StrategoState};
use std::rc::Rc;

pub trait Search {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Rc<Node>>;

    fn value(&self, pos: &mut StrategoState) -> f32;
}
