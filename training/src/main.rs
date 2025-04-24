#![allow(dead_code)]

mod buffer;
mod deployment;
mod thread;
mod value;

#[derive(Debug)]
struct Args {
    threads: usize,
    epochs: usize,
    size: usize,
    limit: usize,
    iterations: usize,
    network: String,
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
            threads: 1,
            epochs: 16,
            size: 250_000,
            limit: 1,
            iterations: 800,
            network: String::from("value.net"),
            output: String::from("datagen.bin"),
        };

        value::run(args);
    }
}
