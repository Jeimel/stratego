#![allow(dead_code)]

mod buffer;
mod deployment;
mod thread;
mod value;

fn main() {
    #[cfg(feature = "deployment")]
    {
        deployment::run("deployment.net", 2000, 4096);
    }

    #[cfg(feature = "value")]
    {
        let args = value::ValueArgs {
            threads: 4,
            supervised: true,
            steps: 50,
            epochs: 10, // from 1-50: 10, 51-100: 100
            batch_size: 16384,
            buffer_size: 1_000_000,
            games: 512,
            iterations: 1600,
            network: String::from("value.net"),
            output: String::from("/Users/felixjablinski/Downloads/datagen.bin"),
        };

        value::run(args);
    }
}
