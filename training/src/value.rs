use crate::{buffer::ReplayBuffer, thread::DatagenThread, Args};
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use stratego::value::Network;
use tch::{
    nn::{Adam, OptimizerConfig, VarStore},
    Device, Reduction, Tensor,
};

pub fn run(args: Args) {
    println!("{:?}", args);

    let mut vs = VarStore::new(Device::cuda_if_available());
    let net = vs.load(&args.network);

    if net.is_ok() {
        println!("info network {}", args.network);
    }

    let buffer = Arc::new(Mutex::new(ReplayBuffer::new(
        &args.output,
        args.size,
        args.limit,
    )));
    let net = Arc::new(Network::new(&vs.root()));

    {
        let _ = tch::no_grad_guard();

        let abort = AtomicBool::new(false);
        datagen(&args, buffer.clone(), net.clone(), abort);
    }

    let buffer = buffer.lock().unwrap();

    let mut inputs = Vec::with_capacity(args.size);
    let mut targets = Vec::with_capacity(args.size);

    for data in &buffer.dataset {
        inputs.push(Tensor::from_slice(&data.input));
        targets.push(Tensor::from_slice(&[data.target]));
    }
    let inputs = Tensor::stack(&inputs, 0);
    let targets = Tensor::stack(&targets, 0);

    let predictions = net.forward(&inputs);

    let mut opt = Adam::default().build(&vs, 0.001).unwrap();

    let loss = predictions.mse_loss(&targets, Reduction::Mean);
    opt.backward_step(&loss);

    println!("Loss: {:?}", loss);

    vs.save(&args.network).unwrap();
}

fn datagen<'a>(
    args: &Args,
    buffer: Arc<Mutex<ReplayBuffer>>,
    net: Arc<Network>,
    abort: AtomicBool,
) {
    std::thread::scope(|s| {
        let abort = &abort;

        for _ in 0..args.threads {
            let buffer = buffer.clone();
            let net = net.clone();

            s.spawn(move || {
                let mut thread = DatagenThread::new(args.iterations, buffer, net, abort);
                thread.run();
            });
        }
    });
}
