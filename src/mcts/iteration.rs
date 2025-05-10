use super::{node::Node, Search};
use crate::{
    policy::{ordered, policy, DEFAULT_WEIGHTS},
    stratego::{GameState, StrategoState},
    value::{self},
};
use rand::distr::Distribution;
use std::sync::Arc;

pub fn execute_one<S: Search, const MULTIPLE: bool>(
    pos: &mut StrategoState,
    mut node: Arc<Node>,
    search: &S,
) {
    let mut rng = rand::rng();

    let mut moves: Vec<_>;
    let mut untried;
    loop {
        moves = (if MULTIPLE {
            search
                .information(&pos.anonymize(pos.stm() as usize ^ 1))
                .gen()
        } else {
            pos.gen()
        })
        .iter()
        .collect();

        let sum = ordered(&pos, &moves, &DEFAULT_WEIGHTS).1;
        for c in node.children() {
            *c.policy_mut() = policy(&pos, &c.mov().unwrap(), &DEFAULT_WEIGHTS).exp() / sum;
        }

        untried = node.untried(&moves);
        if moves.is_empty() || !untried.is_empty() {
            break;
        }

        node = search.select(&node, &moves).unwrap();
        pos.make(node.mov().unwrap());
    }

    if untried.len() != 0 {
        let i = search.policy(&pos, &untried).sample(&mut rng);
        pos.make(untried[i]);

        node = node.add(untried[i], pos.game_state(), heuristic(pos));
    }

    let mut reward = -utility(pos, search);

    let mut previous = node;
    loop {
        previous.update(reward);
        reward = -reward;

        let parent = previous.parent();
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

fn heuristic(pos: &mut StrategoState) -> f32 {
    match pos.game_state() {
        GameState::Ongoing => value::evaluate(pos),
        GameState::Win => 1.0,
        GameState::Draw => 0.0,
        GameState::Loss => -1.0,
    }
}
