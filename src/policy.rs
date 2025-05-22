use crate::stratego::{Flag, Move, Piece, StrategoState};
use rand::distr::weighted::WeightedIndex;
use std::cmp::Ordering;

pub const DEFAULT_WEIGHTS: [f32; 5] = [3.0, 1.0, 10.0, 18.0, 1000.0];

pub enum Policy {
    Uniform,
    Ordered([f32; 5]),
}

impl Policy {
    pub fn get(&self, pos: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32> {
        match self {
            Policy::Uniform => uniform(pos, moves),
            Policy::Ordered(weights) => {
                WeightedIndex::new(&ordered(pos, moves, weights).0).unwrap()
            }
        }
    }
}

pub fn uniform(_: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32> {
    let weights = vec![1f32 / moves.len() as f32; moves.len()];

    WeightedIndex::new(&weights).unwrap()
}

pub fn ordered(pos: &StrategoState, moves: &Vec<Move>, weights: &[f32; 5]) -> (Vec<f32>, f32) {
    let mut scores = vec![weights[0]; moves.len()];
    for (i, mov) in moves.iter().enumerate() {
        scores[i] = policy(&pos, mov, &weights)
    }

    let sum: f32 = scores.iter().map(|x| x).sum();
    (scores.iter().map(|x| x / sum).collect(), sum)
}

pub fn policy(pos: &StrategoState, mov: &Move, weights: &[f32; 5]) -> f32 {
    if (mov.flag & Flag::CAPTURE) == 0 {
        return weights[0];
    }

    let other = pos.board().piece(mov.to);
    if other == Piece::FLAG {
        return weights[4];
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

    return match ordering {
        Ordering::Less => weights[1],
        Ordering::Equal => weights[2],
        Ordering::Greater => weights[3],
    };
}
