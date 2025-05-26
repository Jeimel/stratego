use stratego::{
    deployment::Deployment,
    information::Information,
    mcts::{ISMCTS, PIMC},
    policy::{Policy, DEFAULT_WEIGHTS},
    random::UniformRandom,
    select::Select,
    stratego::StrategoState,
    tournament::Tournament,
    value::{evaluate, Value},
    Algorithm,
};

fn main() {
    // Value::SimulationCutoff(0.01, |pos: &mut StrategoState| { (evaluate(pos) / 750.0).tanh() })
    // Value::SimulationOrdered([3, 1, 5, 15, 1000]),

    let mut tournament = Tournament::new(150);

    let one = ISMCTS::new(
        10_000,
        Value::SimulationOrderedCutoff(DEFAULT_WEIGHTS, 0.025, |pos: &mut StrategoState| {
            (evaluate(pos) / 750.0).tanh()
        }),
        Policy::Uniform,
        Select::ISUCT(1.41),
        Deployment::Dataset,
        Information::Random,
    );
    let two = ISMCTS::new(
        10_000,
        Value::SimulationOrderedCutoff(DEFAULT_WEIGHTS, 0.025, |pos: &mut StrategoState| {
            (evaluate(pos) / 750.0).tanh()
        }),
        Policy::Uniform,
        Select::ISUCT(1.41),
        Deployment::Dataset,
        Information::Random,
    );
    let three = PIMC::new(
        10,
        1_000,
        Value::SimulationOrderedCutoff(DEFAULT_WEIGHTS, 0.025, |pos: &mut StrategoState| {
            (evaluate(pos) / 750.0).tanh()
        }),
        Policy::Uniform,
        Select::ISUCT(1.41),
        Deployment::Dataset,
        Information::Random,
    );

    tournament.add("soismcts", Algorithm::SOISMCTS(one), false);
    tournament.add("moismcts", Algorithm::MOISMCTS(two), false);
    tournament.add("pimc", Algorithm::PIMC(three), false);
    tournament.add(
        "random",
        Algorithm::Random(UniformRandom::new(Deployment::Random)),
        true,
    );

    tournament.run(50);
}
