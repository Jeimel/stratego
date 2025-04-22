use bincode::{config, Decode, Encode};
use std::{
    fs::File,
    io::Write,
    sync::atomic::{AtomicBool, Ordering},
};
use stratego::stratego::Move;

#[derive(Encode, Decode)]
pub struct MoveData {
    pub mov: Move,
    pub score: f32,
    pub policy: Vec<(Move, usize)>,
}

#[derive(Encode, Decode)]
pub struct SearchData {
    pub pos: String,
    pub result: f32,
    pub moves: Vec<MoveData>,
}

impl SearchData {
    pub fn push(&mut self, mov: Move, score: f32, policy: Vec<(Move, usize)>) {
        self.moves.push(MoveData { mov, score, policy });
    }
}

pub struct ReplayBuffer {
    pub file: File,
    pub games: usize,
    pub limit: usize,
}

impl ReplayBuffer {
    pub fn push(&mut self, data: &SearchData, abort: &AtomicBool) {
        if abort.load(Ordering::Relaxed) {
            return;
        }

        self.games += 1;

        let config = config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(data, config).unwrap();

        Write::write_all(&mut self.file, &encoded).unwrap();

        if self.games >= self.limit {
            abort.store(true, Ordering::Relaxed);
            return;
        }

        if self.games % 32 == 0 {
            self.report();
        }
    }

    pub fn report(&self) {
        println!("info datagen games {}", self.games);
    }
}
