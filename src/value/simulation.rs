use super::Heuristic;
use crate::{
    policy::ordered,
    stratego::{GameState, StrategoState},
};
use rand::{
    distr::{weighted::WeightedIndex, Distribution},
    rng,
    seq::IteratorRandom,
    Rng,
};

pub fn simulation_uniform(pos: &mut StrategoState) -> f32 {
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

pub fn simulation_ordered(pos: &mut StrategoState, weights: &[f32; 5]) -> f32 {
    let mut rng = rng();

    let stm = pos.stm();
    while !pos.game_over() {
        let moves = pos.gen();

        if moves.len() == 0 {
            // Handle two-squares and more-squares rule
            pos.set_game_state(GameState::Loss);
            break;
        }

        let softmax = ordered(pos, &moves.iter().collect(), &weights).0;
        let dist = WeightedIndex::new(&softmax).unwrap();
        let mov = moves[dist.sample(&mut rng)];
        pos.make(mov);
    }

    let current = f32::from(stm == pos.stm());
    match pos.game_state() {
        GameState::Draw => 0.0,
        GameState::Win => -1.0 + (2.0 * current),
        GameState::Loss => 1.0 + (-2.0 * current),
        GameState::Ongoing => unreachable!(),
    }
}

pub fn simulation_cutoff(pos: &mut StrategoState, c: f32, heuristic: Heuristic) -> f32 {
    let mut rng = rng();

    let stm = pos.stm();
    while !pos.game_over() {
        if rng.random::<f32>() < c {
            break;
        }

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
        GameState::Ongoing => heuristic(pos) * (-1.0 + 2.0 * current),
    }
}

pub fn simulation_ordered_cutoff(
    pos: &mut StrategoState,
    weights: &[f32; 5],
    c: f32,
    heuristic: Heuristic,
) -> f32 {
    let mut rng = rng();

    let stm = pos.stm();
    while !pos.game_over() {
        if rng.random::<f32>() < c {
            break;
        }

        let moves = pos.gen();
        if moves.len() == 0 {
            // Handle two-squares and more-squares rule
            pos.set_game_state(GameState::Loss);
            break;
        }

        let softmax = ordered(pos, &moves.iter().collect(), &weights).0;
        let dist = WeightedIndex::new(&softmax).unwrap();
        let mov = moves[dist.sample(&mut rng)];
        pos.make(mov);
    }

    let current = f32::from(stm == pos.stm());
    match pos.game_state() {
        GameState::Draw => 0.0,
        GameState::Win => -1.0 + (2.0 * current),
        GameState::Loss => 1.0 + (-2.0 * current),
        GameState::Ongoing => heuristic(pos) * (-1.0 + 2.0 * current),
    }
}
