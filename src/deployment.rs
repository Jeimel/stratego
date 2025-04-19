use crate::stratego::Position;
use rand::{rng, seq::IndexedRandom, Rng};
use std::collections::HashSet;

// Deployments from pov of blue
pub type Deployment = fn() -> String;

pub fn uniform() -> String {
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

    let mut deployment = [' '; 24];

    let mut rng = rng();
    let mut indices = HashSet::new();
    while indices.len() < 10 {
        indices.insert(rng.random_range(..deployment.len()));
    }

    indices
        .iter()
        .zip(PIECES.iter())
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

pub fn dataset() -> String {
    const DEPLOYMENTS: [&str; 2] = ["5c2/2csdbd1/3gbfm1", "5cbf/5cdb/1gsd3m"];

    let mut rng = rng();
    DEPLOYMENTS.choose(&mut rng).unwrap().to_string()
}
