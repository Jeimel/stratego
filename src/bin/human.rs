use std::{io::stdin, process};
use stratego::{
    deployment::Deployment,
    mcts::{Search, ISMCTS},
    policy::Policy,
    select::Select,
    stratego::{GameState, MoveList, Position, StrategoState},
    value::Value,
};

fn main() {
    let color = select("color", &["red", "blue"]);

    let deployment_type = select("deployment type", &["own", "heuristic", "dataset"]);
    let human = match deployment_type.as_str() {
        "own" => custom_deployment(),
        "heuristic" => Deployment::Heuristic.get(),
        "dataset" => Deployment::Dataset.get(),
        _ => unreachable!(),
    };

    let mut ismcts = ISMCTS::<false>::new(
        100_000,
        Value::SimulationUniform,
        Policy::Uniform,
        Select::ISUCT(1.41),
        Deployment::Dataset,
    );

    let deployments = if color == "red" {
        deployment(&human, &ismcts.deployment())
    } else {
        deployment(&ismcts.deployment(), &human)
    };

    let pos = format!("{}/8/8/{} r", deployments.1, deployments.0);
    let mut pos = StrategoState::from(&pos);

    let human_stm = if color == "red" { false } else { true };

    while !pos.game_over() {
        let moves = pos.gen();

        if moves.len() == 0 {
            pos.set_game_state(GameState::Loss);
            break;
        }

        if pos.stm() != human_stm {
            let mov = ismcts.go(&mut pos);
            println!("MCTS move {}", mov);

            pos.make(mov);
            continue;
        }

        println!("{}", pos.anonymize((pos.stm() as usize) ^ 1));
        moves.iter().for_each(|m| println!("{m}"));

        let mov = read();
        make(&mut pos, &moves, &mov);
    }
}

fn custom_deployment() -> String {
    println!("Position: ");
    println!(
        "Flag {}\nSpy {}\nScout {}\nMiner {}\nGeneral {}\nMarshal {}\nBomb {}\n",
        Position::SYMBOLS[8],
        Position::SYMBOLS[9],
        Position::SYMBOLS[10],
        Position::SYMBOLS[11],
        Position::SYMBOLS[12],
        Position::SYMBOLS[13],
        Position::SYMBOLS[15],
    );

    read()
}

fn deployment(red: &str, blue: &str) -> (String, String) {
    (
        red.to_ascii_uppercase()
            .split('/')
            .rev()
            .collect::<Vec<_>>()
            .join("/"),
        blue.to_ascii_lowercase(),
    )
}

fn make(pos: &mut StrategoState, moves: &MoveList, mov_str: &str) {
    for i in 0..moves.len() {
        let mov = moves[i];

        if format!("{mov}") == mov_str {
            pos.make(mov);

            return;
        }
    }

    println!("error illegal move {}", mov_str);
}

fn select(text: &str, options: &[&str]) -> String {
    println!("Choose {}: {:?}", text, options);

    loop {
        let input = read();

        if let Some(result) = options.iter().find(|option| input == **option) {
            return result.to_string();
        }
    }
}

fn read() -> String {
    let mut input = String::new();
    let bytes_read = stdin().read_line(&mut input).unwrap();

    if bytes_read == 0 {
        process::exit(0);
    }

    input.trim().to_string()
}
