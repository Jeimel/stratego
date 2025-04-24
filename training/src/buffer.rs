use bincode::{
    config::{self},
    Decode, Encode,
};
use std::{
    collections::VecDeque,
    fs::{File, OpenOptions},
    io::BufReader,
    sync::atomic::{AtomicBool, Ordering},
};
use stratego::stratego::{Move, StrategoState};

#[derive(Clone, Encode, Decode)]
pub struct SearchData {
    pub input: [f32; StrategoState::FEATURES],
    pub target: f32,
    pub policy: Vec<(Move, usize)>,
}

impl SearchData {
    pub fn new(input: [f32; StrategoState::FEATURES], policy: Vec<(Move, usize)>) -> Self {
        Self {
            input,
            target: 0.0,
            policy,
        }
    }
}

pub struct ReplayBuffer {
    pub file: File,
    pub dataset: VecDeque<SearchData>,
    pub size: usize,
    pub games: usize,
    pub limit: usize,
}

impl ReplayBuffer {
    pub fn new(path: &str, size: usize, limit: usize) -> Self {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        let mut buffer = ReplayBuffer {
            file,
            dataset: VecDeque::with_capacity(size),
            size,
            games: 0,
            limit,
        };

        let config = config::standard();

        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        loop {
            match bincode::decode_from_reader(&mut reader, config) {
                Ok(search_data) => buffer.push_pop(search_data),
                Err(_) => break,
            }
        }

        buffer
    }

    pub fn push(&mut self, data: Vec<SearchData>, abort: &AtomicBool) {
        if abort.load(Ordering::Relaxed) {
            return;
        }

        self.games += 1;

        let config = config::standard();
        for search_data in data {
            bincode::encode_into_std_write(&search_data, &mut self.file, config).unwrap();

            self.push_pop(search_data);
        }

        if self.games >= self.limit {
            abort.store(true, Ordering::Relaxed);
            return;
        }

        if self.games % 128 == 0 {
            self.report();
        }
    }

    pub fn reset(&mut self) {
        self.games = 0;
    }

    pub fn report(&self) {
        println!(
            "info datagen games {} dataset {}",
            self.games,
            self.dataset.len()
        );
    }

    fn push_pop(&mut self, data: SearchData) {
        if self.dataset.len() == self.size {
            self.dataset.pop_front();
        }
        self.dataset.push_back(data);
    }
}
