use crate::stratego::{Flag, GameState, Piece, StrategoState};
use rand::{
    distr::{weighted::WeightedIndex, Distribution},
    rng,
    seq::IteratorRandom,
};
use std::{cmp::Ordering, usize};

pub type Value = fn(&mut StrategoState) -> f32;

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

            scores[i] = match (mov.piece as usize).cmp(&other) {
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

pub fn piece_value(pos: &mut StrategoState) -> f32 {
    const VALUES: [isize; 7] = [200, 30, 25, 200, 400, 0, 20];

    let board = pos.board();
    let stm = pos.stm() as usize;

    let mut sum = 0;
    for side in [stm, stm ^ 1] {
        let pieces = board.get(side);

        for piece in Piece::SPY..=Piece::BOMB {
            let mask = board.get(piece) & pieces;
            sum += VALUES[piece - 3] * mask.count_ones() as isize;
        }

        sum = -sum;
    }

    sum as f32 / 950.0
}

pub fn network(pos: &mut StrategoState) -> f32 {
    let _red = pos.features::<false>(0);
    let _blue = pos.features::<true>(1);

    0.0
}
