#![allow(dead_code)]

mod buffer;
mod deployment;
mod thread;
mod value;

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
        let args = value::ValueArgs {
            threads: 1,
            steps: 1,
            epochs: 1,
            batch_size: 32,
            buffer_size: 250,
            games: 8,
            iterations: 800,
            network: String::from("value.net"),
            output: String::from("datagen.bin"),
        };

        value::run(args);
    }
}
