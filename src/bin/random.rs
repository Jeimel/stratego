use rand::{rng, seq::IteratorRandom, Rng};
use std::collections::HashSet;
use stratego::{stratego::Position, Protocol};

fn main() {
    Random::default().run();
}

#[derive(Default)]
pub struct Random {}

impl Random {
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
}

impl Protocol for Random {
    fn option() -> String {
        String::new()
    }

    fn handle_deployment(&self, _: Vec<&str>) -> String {
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

    fn handle_go(
        &mut self,
        _: Vec<&str>,
        pos: &stratego::stratego::Position,
        stack: &stratego::stratego::MoveStack,
    ) -> stratego::stratego::Move {
        let mut rng = rand::rng();

        pos.gen(&stack).iter().choose(&mut rng).expect("valid move")
    }

    fn handle_options(&mut self, _: Vec<&str>) {
        unimplemented!()
    }
}
