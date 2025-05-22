use bincode::{
    config::{self},
    Decode, Encode,
};
use std::{
    collections::VecDeque,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
    usize,
};
use stratego::stratego::{Move, StrategoState};

#[derive(Clone, Encode, Decode)]
pub struct SearchData {
    pub input: [[f32; StrategoState::FEATURES]; 2],
    pub target: f32,
    pub heuristic: f32,
    pub result: f32,
    pub policy: Vec<(Move, usize)>,
    pub stm: bool,
}

impl SearchData {
    pub fn new(
        input: [[f32; StrategoState::FEATURES]; 2],
        target: f32,
        heuristic: f32,
        policy: Vec<(Move, usize)>,
        stm: bool,
    ) -> Self {
        Self {
            input,
            target,
            heuristic,
            result: 0.0,
            policy,
            stm,
        }
    }
}

pub struct ReplayBuffer {
    pub file: File,
    pub dataset: VecDeque<SearchData>,
    pub size: usize,
    pub limit: usize,
    pub len: usize,
    pub results: [usize; 3],
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
            limit,
            len: 0,
            results: [0usize; 3],
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

    pub fn push(&mut self, data: Vec<SearchData>) {
        {
            let mut writer = BufWriter::new(&mut self.file);
            let config = config::standard();

            for search_data in &data {
                bincode::encode_into_std_write(search_data, &mut writer, config).unwrap();
            }

            writer.flush().unwrap();
        }

        for search_data in data {
            self.push_pop(search_data);
        }
    }

    pub fn report(&self) {
        let sum = (self.results[0] + self.results[1] + self.results[2]) as f32;

        println!(
            "info datagen len {} wins {} draws {} losses {}",
            self.len,
            self.results[0] as f32 / sum,
            self.results[1] as f32 / sum,
            self.results[2] as f32 / sum
        );
    }

    fn push_pop(&mut self, data: SearchData) {
        match data.result {
            -1.0 => self.results[2] += 1,
            0.0 => self.results[1] += 1,
            1.0 => self.results[0] += 1,
            _ => unreachable!(),
        }

        self.len += 1;

        if self.dataset.len() == self.size {
            self.dataset.pop_front();
        }
        self.dataset.push_back(data);
    }
}
