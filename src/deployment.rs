use crate::stratego::Position;
use heuristic::heuristic;
use rand::{rng, seq::IndexedRandom, Rng};
use std::collections::HashSet;

pub mod heuristic;
mod network;

pub use heuristic::evaluate;
pub use network::Network;

// Deployments from pov of blue
pub enum Deployment {
    Random,
    Heuristic(usize, bool),
    Dataset,
    Network(Network, usize),
}

impl Deployment {
    pub fn get(&self) -> String {
        match self {
            Deployment::Random => random(),
            Deployment::Heuristic(attempts, min) => heuristic(*attempts, *min),
            Deployment::Dataset => dataset(),
            Deployment::Network(net, attempts) => net.get(*attempts),
        }
    }
}

pub fn random() -> String {
    const PIECES: [char; 10] = [
        Position::SYMBOLS[8],
        Position::SYMBOLS[9],
        Position::SYMBOLS[10],
        Position::SYMBOLS[10],
        Position::SYMBOLS[11],
        Position::SYMBOLS[11],
        Position::SYMBOLS[12],
        Position::SYMBOLS[13],
        Position::SYMBOLS[15],
        Position::SYMBOLS[15],
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

fn dataset() -> String {
    const DEPLOYMENTS: [&str; 12] = [
        "1c6/2d3mc/d1sgbfb1",
        "3bfbc1/1cd1m3/2sgd3",
        "fb1c4/bg4c1/1dsgd3",
        "1c6/s2cm3/g1d1dfbb",
        "7c/d1fbg3/cbmd1s2",
        "4mfbc/c4bds/1d2g3",
        "7c/fb2d1c1/mgsbd3",
        "3c3c/1g2bfm1/d1s1db2",
        "5c1/2csdbd1/3gbfm1",
        "7c/c3bfm1/1dsg1bd1",
        "5cbf/5cdb/1gsd3m",
        "1mfbc3/1cbd4/2sg1d2",
    ];

    let mut rng = rng();
    let mut deployment = DEPLOYMENTS.choose(&mut rng).unwrap().to_string();

    if rng.random() {
        deployment = deployment
            .split('/')
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join("/");
    }

    deployment
}
