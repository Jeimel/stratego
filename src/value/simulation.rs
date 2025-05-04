use crate::{
    stratego::{Flag, GameState, Piece, StrategoState},
    value::heuristic::heuristic,
};
use rand::{
    distr::{weighted::WeightedIndex, Distribution},
    rng,
    seq::IteratorRandom,
    Rng,
};
use std::{cmp::Ordering, usize};

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

pub fn simulation_ordered(pos: &mut StrategoState) -> f32 {
    let mut rng = rng();

    let stm = pos.stm();
    while !pos.game_over() {
        let moves = pos.gen();

        if moves.len() == 0 {
            // Handle two-squares and more-squares rule
            pos.set_game_state(GameState::Loss);
            break;
        }

        let mut scores = vec![5usize; moves.len()];
        for (i, mov) in moves.iter().enumerate() {
            if (mov.flag & Flag::CAPTURE) == 0 {
                continue;
            }

            let other = pos.board().piece(mov.to);
            if other == Piece::FLAG {
                scores[i] = 50;
                continue;
            }

            let piece = mov.piece as usize;
            let ordering = if (piece == Piece::SPY && other == Piece::MARSHAL)
                || (piece == Piece::MINER && other == Piece::BOMB)
            {
                // Spy can capture general or miner can defuse bomb
                Ordering::Greater
            } else {
                piece.cmp(&other)
            };

            scores[i] = match ordering {
                Ordering::Less => 1,
                Ordering::Equal => 5,
                Ordering::Greater => 15,
            }
        }

        let dist = WeightedIndex::new(&scores).unwrap();
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

pub fn simulation_cutoff(pos: &mut StrategoState, c: f32) -> f32 {
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
        GameState::Ongoing => heuristic(pos, 0.01),
    }
}
