use crate::stratego::{Move, Position, StrategoState};
use rand::{rng, seq::IteratorRandom, Rng};
use std::collections::HashSet;

#[derive(Default)]
pub struct UniformRandom {}

impl UniformRandom {
    const PIECES: [char; 10] = [
        Position::SYMBOLS[0],
        Position::SYMBOLS[1],
        Position::SYMBOLS[2],
        Position::SYMBOLS[2],
        Position::SYMBOLS[3],
        Position::SYMBOLS[3],
        Position::SYMBOLS[4],
        Position::SYMBOLS[5],
        Position::SYMBOLS[7],
        Position::SYMBOLS[7],
    ];

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        let mut rng = rng();

        pos.clone()
            .gen()
            .iter()
            .choose(&mut rng)
            .expect("valid move")
    }

    pub fn deployment(&self) -> String {
        let mut deployment = [' '; 24];

        let mut rng = rng();
        let mut indices = HashSet::new();
        while indices.len() < 10 {
            indices.insert(rng.random_range(..deployment.len()));
        }

        indices
            .iter()
            .zip(Self::PIECES.iter())
            .for_each(|(i, piece)| deployment[*i] = *piece);

        let mut board = String::new();
        for rank in deployment.chunks(8) {
            let mut last_piece = 0;

            for cell in rank {
                if *cell == ' ' {
                    last_piece += 1;
                    continue;
                }

                if last_piece != 0 {
                    board.push(char::from(last_piece + b'0'));
                }

                board.push(*cell);
                last_piece = 0;
            }

            if last_piece != 0 {
                board.push(char::from(last_piece + b'0'));
            }

            board.push('/');
        }

        let mut chars = board.chars();
        chars.next_back();

        chars.as_str().to_string()
    }
}
