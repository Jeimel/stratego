use bincode::{
    config::{self},
    Decode, Encode,
};
use std::{
    collections::VecDeque,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
};
use stratego::stratego::{Move, StrategoState};
use tch::{kind, Tensor};

pub struct BatchSampler<'a> {
    index: i64,
    perm: Tensor,
    inputs: &'a Tensor,
    targets: &'a Tensor,
    size: i64,
    batch: i64,
}

impl<'a> BatchSampler<'a> {
    pub fn new(inputs: &'a Tensor, targets: &'a Tensor, size: i64, batch: i64) -> Self {
        let (a, b) = (inputs.size()[0], targets.size()[0]);
        assert!(a == b);

        Self {
            index: 0,
            perm: Tensor::randperm(a, kind::INT64_CPU),
            inputs,
            targets,
            size,
            batch,
        }
    }
}

impl<'a> Iterator for BatchSampler<'a> {
    type Item = (Tensor, Tensor);

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.index + self.batch).min(self.size);
        if self.index >= self.size || (next - self.index) < self.batch {
            return None;
        }

        let batch = self.perm.narrow(0, self.index, next - self.index);
        self.index = next;

        Some((
            self.inputs.index_select(0, &batch),
            self.targets.index_select(0, &batch),
        ))
    }
}

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

    pub fn push(&mut self, data: Vec<SearchData>) {
        self.games += 1;

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

        if self.games % 128 == 0 {
            self.report();
        }
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
