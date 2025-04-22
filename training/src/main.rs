#![allow(dead_code)]

use bincode::config;
use buffer::{ReplayBuffer, SearchData};
use std::{
    fs::{File, OpenOptions},
    io::BufReader,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use thread::DatagenThread;

mod buffer;
mod deployment;
mod thread;

#[derive(Debug)]
struct Args {
    threads: usize,
    limit: usize,
    iterations: usize,
    output: String,
}

fn main() {
    #[cfg(feature = "deployment")]
    {
        let mut vs = tch::nn::VarStore::new(tch::Device::cuda_if_available());
        let net = stratego::deployment::Network::new(&vs.root());

        vs.load("deployment.net").unwrap();
        deployment::run(&vs, net, 30, 2048);

        vs.save("deployment.net").unwrap();
    }

    #[cfg(feature = "datagen")]
    {
        let args = Args {
            threads: 4,
            limit: 100,
            iterations: 800,
            output: String::from("datagen.bin"),
        };

        run_datagen(args);
        read_datagen(&args.output);
    }
}

fn run_datagen(args: Args) {
    println!("{:?}", args);

    let abort = AtomicBool::new(false);
    let abort = &abort;

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(args.output)
        .unwrap();
    let buffer = ReplayBuffer {
        file,
        games: 0,
        limit: args.limit,
    };

    let buffer_mutex = Arc::new(Mutex::new(buffer));

    std::thread::scope(|s| {
        let abort = &abort;

        for _ in 0..args.threads {
            let buffer = buffer_mutex.clone();

            s.spawn(move || {
                let mut thread = DatagenThread::new(args.iterations, buffer, abort);
                thread.run();
            });
        }
    });
}

fn read_datagen(output: &str) -> Vec<SearchData> {
    let config = config::standard();

    let file = File::open(output).unwrap();
    let mut reader = BufReader::new(file);

    let mut data = Vec::new();
    loop {
        match bincode::decode_from_reader(&mut reader, config) {
            Ok(search_data) => data.push(search_data),
            Err(_) => break,
        }
    }

    data
}
