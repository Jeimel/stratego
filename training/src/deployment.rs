use stratego::{
    deployment::{heuristic::evaluate, Network},
    stratego::StrategoState,
    value::simulation_uniform,
};
use tch::{
    nn::{Adam, ModuleT, OptimizerConfig, VarStore},
    Kind, Reduction, Tensor,
};

pub fn run(vs: &VarStore, net: Network, epochs: usize, batch_size: usize) {
    let mut opt = Adam::default().build(&vs, 1e-3).unwrap();

    for i in 0..epochs {
        let (input, targets) = batch_supervised(&net, batch_size);

        let input = Tensor::stack(&input, 0);
        let targets = Tensor::stack(&targets, 0);

        let loss = net
            .forward_t(&input, true)
            .l1_loss(&targets, Reduction::Mean);
        opt.backward_step(&loss);

        println!("info epoch {i} loss {:?}", loss);
    }
}

fn batch_supervised(net: &Network, size: usize) -> (Vec<Tensor>, Vec<Tensor>) {
    let mut input = Vec::with_capacity(size);
    let mut targets = Vec::with_capacity(size);

    for _ in 0..size {
        let deployment = net.get();
        let score = evaluate(&deployment);

        input.push(Network::tensor(&deployment));
        targets.push(Tensor::from(score as f32 / 94.0));
    }

    (input, targets)
}

fn batch_reinforcement(net: &Network, size: usize) -> (Vec<Tensor>, Vec<Tensor>) {
    let mut input = Vec::with_capacity(size);
    let mut targets = Vec::with_capacity(size);

    for _ in 0..size {
        let (red_deployment, blue_deployment) = (net.get(), net.get());
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
