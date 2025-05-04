use stratego::deployment::evaluate;

fn main() {
    const DEPLOYMENTS: usize = 200_000;

    let mut vs = tch::nn::VarStore::new(tch::Device::cuda_if_available());
    let net = stratego::deployment::Network::new(&vs.root());

    vs.load("deployment.net").unwrap();

    for attempts in [100] {
        let result: Vec<_> = (0..DEPLOYMENTS)
            .map(|_| {
                let deployment = net.get(attempts);

                (deployment.clone(), evaluate(&deployment))
            })
            .collect();

        let (deployment_min, score_min) = result.iter().min_by_key(|(_, score)| score).unwrap();
        let (deployment_max, score_max) = result.iter().max_by_key(|(_, score)| score).unwrap();
        let sum = result.iter().map(|(_, score)| score).sum::<isize>();

        println!(
            "info attemps {} min {} score {} max {} score {} mean {}",
            attempts,
            deployment_min,
            score_min,
            deployment_max,
            score_max,
            sum / result.len() as isize
        );
    }
}
