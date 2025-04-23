use crate::stratego::{Flag, Move, Piece, StrategoState};
use rand::distr::weighted::WeightedIndex;
use std::cmp::Ordering;

pub enum Policy {
    Uniform,
    Ordered,
}

impl Policy {
    pub fn get(&self, pos: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32> {
        match self {
            Policy::Uniform => uniform(pos, moves),
            Policy::Ordered => ordered(pos, moves),
        }
    }
}

pub fn uniform(_: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32> {
    let weights = vec![1f32 / moves.len() as f32; moves.len()];

    WeightedIndex::new(&weights).unwrap()
}

pub fn ordered(pos: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32> {
    let mut logits = vec![5f32; moves.len()];
    for (i, mov) in moves.iter().enumerate() {
        if (mov.flag & Flag::CAPTURE) == 0 {
            continue;
        }

        let other = pos.board().piece(mov.to);
        if other == Piece::FLAG {
            logits[i] = 50.0;
            continue;
        }

        logits[i] = match (mov.piece as usize).cmp(&other) {
            Ordering::Less => 1.0,
            Ordering::Equal => 5.0,
            Ordering::Greater => 15.0,
        }
    }

    let sum: f32 = logits.iter().map(|x| x.exp()).sum();
    let softmax: Vec<f32> = logits.iter().map(|x| x.exp() / sum).collect();

    WeightedIndex::new(&softmax).unwrap()
}
