use stratego::{
    deployment::{heuristic::evaluate, Network},
    stratego::StrategoState,
    value::simulation_uniform,
};
use tch::{
    nn::{Adam, OptimizerConfig, VarStore},
    Device, Kind, Reduction, Tensor,
};

pub fn run(path: &str, epochs: usize, batch_size: usize) {
    let mut vs = VarStore::new(Device::cuda_if_available());
    let net = Network::new(&vs.root());

    let _ = vs.load(path);
    vs.save(path).unwrap();

    let mut opt = Adam::default().build(&vs, 1e-3).unwrap();
    opt.set_weight_decay(1e-4);

    for i in 0..epochs {
        let (inputs, targets) = batch_supervised(&net, batch_size);

        let inputs = Tensor::stack(&inputs, 0);
        let targets = Tensor::stack(&targets, 0);

        let predictions = net.forward(&inputs);

        let loss = predictions.mse_loss(&targets, Reduction::Mean);
        opt.backward_step(&loss);

        println!("info epoch {} loss {:?}", i + 1, loss);
    }

    vs.save(path).unwrap();
}

fn batch_supervised(net: &Network, size: usize) -> (Vec<Tensor>, Vec<Tensor>) {
    let mut inputs = Vec::with_capacity(size);
    let mut targets = Vec::with_capacity(size);

    for _ in 0..size {
        let deployment = net.get(5);
        let score = evaluate(&deployment);

        inputs.push(Network::tensor(&deployment));
        targets.push(Tensor::from(score as f32).unsqueeze(0));
    }

    (inputs, targets)
}

fn batch_reinforcement(net: &Network, size: usize) -> (Vec<Tensor>, Vec<Tensor>) {
    let mut input = Vec::with_capacity(size);
    let mut targets = Vec::with_capacity(size);

    for _ in 0..size {
        let (red_deployment, blue_deployment) = (net.get(25), net.get(25));
        let mut pos = position(&red_deployment, &blue_deployment);

        let result = simulation_uniform(&mut pos);
        let (red_result, blue_result) = match result {
            1.0 => (1.0, 0.0),
            0.0 => (0.5, 0.5),
            -1.0 => (0.0, 1.0),
            _ => unreachable!(),
        };

        let (red_result, blue_result) = (
            Tensor::from(red_result).to_kind(Kind::Float),
            Tensor::from(blue_result).to_kind(Kind::Float),
        );
        let (red_tensor, blue_tensor) = (
            Network::tensor(&red_deployment),
            Network::tensor(&blue_deployment),
        );

        input.push(red_tensor);
        input.push(blue_tensor);

        targets.push(red_result);
        targets.push(blue_result);
    }

    (input, targets)
}

fn position(red: &str, blue: &str) -> StrategoState {
    let deployments = (
        red.to_ascii_uppercase()
            .split('/')
            .rev()
            .collect::<Vec<_>>()
            .join("/"),
        blue.to_ascii_lowercase(),
    );

    let pos_str = format!("{}/8/8/{} r", deployments.1, deployments.0);
    StrategoState::from(&pos_str)
}
