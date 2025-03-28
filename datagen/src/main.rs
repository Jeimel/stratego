mod thread;

use clap::Parser;
use std::sync::atomic::AtomicBool;
use thread::DatagenThread;

#[derive(Parser, Debug)]
#[command(name = "datagen")]
struct Args {
    #[arg(short, long, help = "Number of threads to use")]
    threads: usize,
    #[arg(short, long, help = "Number of games to run")]
    games: usize,
    #[arg(short, long, help = "Number of iterations per game")]
    iterations: usize,
    #[arg(short, long, help = "Output file path")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let abort = AtomicBool::new(false);

    std::thread::scope(|s| {
        let abort = &abort;

        for _ in 0..args.threads {
            s.spawn(move || {
                let mut thread = DatagenThread::new();
                thread.run(args.iterations, abort);
            });
        }
    });
}
