use crate::{
    buffer::{BatchSampler, ReplayBuffer},
    thread::DatagenThread,
};
use stratego::value::Network;
use tch::{
    nn::{Adam, OptimizerConfig, VarStore},
    Device, Reduction, Tensor,
};

#[derive(Debug)]
pub struct ValueArgs {
    pub threads: usize,
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
        {
            let _ = tch::no_grad_guard();

            datagen(&args, &mut buffer);
        }

        let mut inputs = Vec::with_capacity(args.buffer_size);
        let mut targets = Vec::with_capacity(args.buffer_size);
        for data in &buffer.dataset {
            inputs.push(Tensor::from_slice(&data.input));
            targets.push(Tensor::from_slice(&[data.target]));
        }
        let inputs = Tensor::stack(&inputs, 0);
        let targets = Tensor::stack(&targets, 0);

        assert!(inputs.size()[0] == targets.size()[0]);

        let size = inputs.size()[0];
        for epoch in 0..args.epochs {
            let mut epoch_loss = 0.0;

            let sampler = BatchSampler::new(&inputs, &targets, size, args.batch_size as i64);
            for (inputs, targets) in sampler {
                let predictions = net.forward(&inputs);

                let loss = predictions.mse_loss(&targets, Reduction::Mean);
                opt.backward_step(&loss);

                epoch_loss += loss.double_value(&[]) as f32;
            }

            epoch_loss *= args.batch_size as f32 / size as f32;
            println!("info epoch {} loss {}", epoch + 1, epoch_loss);
        }

        println!("info step {}", step + 1);

        vs.save(&args.network).unwrap();
    }
}

fn datagen(args: &ValueArgs, buffer: &mut ReplayBuffer) {
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
