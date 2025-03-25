use super::{node::Node, Search};
use crate::stratego::{GameState, StrategoState};
use rand::seq::IteratorRandom;
use std::rc::{Rc, Weak};

pub fn execute_one<S: Search>(pos: &mut StrategoState, mut node: Rc<Node>, search: &S) {
    let mut rng = rand::rng();

    let mut moves: Vec<_>;
    let mut untried;
    loop {
        moves = pos.gen().iter().collect();
        untried = node.untried(&moves);

        if moves.is_empty() || !untried.is_empty() {
            break;
        }

        node = search.select(&node, &moves).unwrap();
        pos.make(node.mov.unwrap());
    }

    if let Some(mov) = untried.into_iter().choose(&mut rng) {
        pos.make(mov);

        node = node.add(mov, pos.game_state());
    }

    let mut reward = utility(pos, search);

    let mut previous = node;
    loop {
        previous.update(reward);
        reward = -reward;

        let parent = previous.parent.as_ref().and_then(Weak::upgrade);
        if let Some(node) = parent {
            previous = node;
        } else {
            break;
        }
    }
}

fn utility<S: Search>(pos: &mut StrategoState, search: &S) -> f32 {
    match pos.game_state() {
        GameState::Ongoing => search.value(pos),
        GameState::Win => 1.0,
        GameState::Draw => 0.0,
        GameState::Loss => -1.0,
    }
}
