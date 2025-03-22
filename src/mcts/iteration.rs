use super::node::Node;
use crate::stratego::{GameState, StrategoState};
use rand::seq::IteratorRandom;
use std::rc::{Rc, Weak};

pub fn execute_one(pos: &mut StrategoState, mut node: Rc<Node>) {
    let mut rng = rand::rng();

    let mut moves: Vec<_>;
    let mut untried;
    loop {
        moves = pos.gen().iter().collect();
        untried = node.untried(&moves);

        if moves.is_empty() || !untried.is_empty() {
            break;
        }

        node = node.select(&moves).unwrap();
        pos.make(node.mov.unwrap());
    }

    if let Some(mov) = untried.into_iter().choose(&mut rng) {
        pos.make(mov);

        node = node.add(mov, pos.game_state());
    }

    let mut reward = utility(pos);

    let (mut previous, mut state) = (node, GameState::default());
    loop {
        previous.update(reward);
        previous.propagate_state(state);

        reward = -reward;

        let parent = previous.parent.as_ref().and_then(Weak::upgrade);
        if let Some(node) = parent {
            state = previous.game_state();
            previous = node;
        } else {
            break;
        }
    }
}

fn utility(pos: &mut StrategoState) -> f32 {
    match pos.game_state() {
        GameState::Ongoing => pos.rollout(),
        GameState::Win => 1.0,
        GameState::Draw => 0.0,
        GameState::Loss => -1.0,
    }
}
