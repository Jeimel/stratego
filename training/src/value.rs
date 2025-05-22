use crate::{
    buffer::{ReplayBuffer, SearchData},
    thread::DatagenThread,
};
use std::{collections::VecDeque, time::Instant};
use stratego::{
    deployment::Deployment,
    information::Information,
    mcts::ISMCTS,
    policy::Policy,
    random::UniformRandom,
    select::Select,
    tournament::Tournament,
    value::{Network, Value},
    Algorithm,
};
use tch::{
    kind,
    nn::{Adam, OptimizerConfig, VarStore},
    Device, Reduction, Tensor,
};

#[derive(Debug)]
pub struct ValueArgs {
    pub threads: usize,
    pub supervised: bool,
    pub steps: usize,
    pub epochs: usize,
    pub batch_size: usize,
    pub buffer_size: usize,
    pub games: usize,
    pub iterations: usize,
    pub network: String,
    pub output: String,
}

pub fn run(args: ValueArgs) {
    println!("{:?}", args);

    let mut vs = VarStore::new(Device::cuda_if_available());
    let net = Network::new(&vs.root());

    let _ = vs.load(&args.network);
    vs.save(&args.network).unwrap();

    let mut opt = Adam::default().build(&vs, 0.001).unwrap();

    let mut buffer = ReplayBuffer::new(&args.output, args.buffer_size, args.games);

    for step in 0..args.steps {
        let start = Instant::now();

        {
            let _guard = tch::no_grad_guard();

            datagen(&args, &mut buffer);
            buffer.report();
        }

        let (features_us, features_them, targets) = if args.supervised {
            supervised(&buffer.dataset)
        } else {
            reinforcement(&buffer.dataset, 0.3)
        };

        let size = targets.size()[0];

        let mut step_loss = 0.0;
        for _ in 0..args.epochs {
            let perm = Tensor::randperm(size, kind::INT64_CPU);
            let batch = perm.narrow(0, 0, args.batch_size as i64);

            let features_us = features_us.index_select(0, &batch);
            let features_them = features_them.index_select(0, &batch);
            let targets = targets.index_select(0, &batch);

            let loss = net
                .forward_batch(&features_us, &features_them)
                .mse_loss(&targets, Reduction::Mean);
            opt.backward_step(&loss);

            step_loss += loss.double_value(&[]) as f32;
        }

        vs.save(&args.network).unwrap();

        step_loss /= args.epochs as f32;

        println!(
            "info step {} loss {} time {:?}",
            step + 1,
            step_loss,
            start.elapsed()
        );

        if (step + 1) % 10 != 0 {
            continue;
        }

        let net = ISMCTS::new(
            10_000,
            Value::NetworkCutoff(Network::new(&vs.root()), 0.025),
            Policy::Uniform,
            Select::ISUCT(1.41),
            Deployment::Heuristic(100, false),
            Information::Random,
        );
        let uct = ISMCTS::new(
            10_000,
            Value::SimulationUniform,
            Policy::Uniform,
            Select::ISUCT(1.41),
            Deployment::Heuristic(100, false),
            Information::Random,
        );
        let random = UniformRandom::new(Deployment::Heuristic(100, false));

        let mut tournament = Tournament::new(150);
        tournament.add("net", Algorithm::SOISMCTS(net), false);
        tournament.add("uct", Algorithm::SOISMCTS(uct), false);
        tournament.add("random", Algorithm::Random(random), false);
        tournament.run(5);
    }
}

fn supervised(dataset: &VecDeque<SearchData>) -> (Tensor, Tensor, Tensor) {
    let mut features_us = Vec::with_capacity(dataset.len());
    let mut features_them = Vec::with_capacity(dataset.len());
    let mut targets = Vec::with_capacity(dataset.len());

    for data in dataset {
        let red = Tensor::from_slice(&data.input[0]);
        let blue = Tensor::from_slice(&data.input[1]);

        let (us, them) = if data.stm { (blue, red) } else { (red, blue) };

        features_us.push(us);
        features_them.push(them);
        targets.push(Tensor::from_slice(&[data.heuristic]));
    }

    (
        Tensor::stack(&features_us, 0),
        Tensor::stack(&features_them, 0),
        Tensor::stack(&targets, 0),
    )
}

fn reinforcement(dataset: &VecDeque<SearchData>, lambda: f32) -> (Tensor, Tensor, Tensor) {
    let mut features_us = Vec::with_capacity(dataset.len());
    let mut features_them = Vec::with_capacity(dataset.len());
    let mut targets = Vec::with_capacity(dataset.len());

    for data in dataset {
        let red = Tensor::from_slice(&data.input[0]);
        let blue = Tensor::from_slice(&data.input[1]);

        let (us, them) = if data.stm { (blue, red) } else { (red, blue) };

        features_us.push(us);
        features_them.push(them);
        targets.push(Tensor::from_slice(&[
            data.target * lambda + data.result * (1.0 - lambda)
        ]));
    }

    (
        Tensor::stack(&features_us, 0),
        Tensor::stack(&features_them, 0),
        Tensor::stack(&targets, 0),
    )
}

fn datagen(args: &ValueArgs, buffer: &mut ReplayBuffer) {
    if args.games == 0 {
        return;
    }

    assert!(args.games % args.threads == 0);

    let mut handles = Vec::with_capacity(args.threads);

    let games = args.games / args.threads;
    for _ in 0..args.threads {
        let mut vs = VarStore::new(Device::cuda_if_available());
        vs.load(&args.network).unwrap();

        let net = Network::new(&vs.root());

        let thread = DatagenThread::new(args.iterations, games, net);
        handles.push(std::thread::spawn(move || thread.run()));
    }

    for handle in handles.drain(..) {
        let worker = handle.join().unwrap();
        buffer.push(worker);
    }
}
