use std::{io::stdin, process};
use stratego::{
    deployment::{heuristic::heuristic, Deployment},
    information::Information,
    mcts::{Search, ISMCTS},
    policy::{Policy, DEFAULT_WEIGHTS},
    select::Select,
    stratego::{Flag, GameState, MoveList, Piece, Position, StrategoState},
    value::{evaluate, Value},
};

fn main() {
    let color = select("color", &["red", "blue"]);

    let deployment_type = select("deployment type", &["own", "heuristic", "dataset"]);
    let human = match deployment_type.as_str() {
        "own" => custom_deployment(),
        "heuristic" => heuristic(80, true),
        "dataset" => Deployment::Dataset.get(),
        _ => unreachable!(),
    };

    let mut ismcts = ISMCTS::<false>::new(
        100_000,
        Value::SimulationOrderedCutoff(DEFAULT_WEIGHTS, 0.025, |pos: &mut StrategoState| {
            (evaluate(pos) / 750.0).tanh()
        }),
        Policy::Uniform,
        Select::ISUCT(1.41),
        Deployment::Dataset,
        Information::Random,
    );

    let deployments = if color == "red" {
        deployment(&human, &ismcts.deployment())
    } else {
        deployment(&ismcts.deployment(), &human)
    };

    let pos = format!("{}/8/8/{} r", deployments.1, deployments.0);
    let mut pos = StrategoState::from(&pos);

    let human_stm = if color == "red" { false } else { true };

    let mut capture = String::new();
    while !pos.game_over() {
        let moves = pos.gen();

        if moves.len() == 0 {
            pos.set_game_state(GameState::Loss);
            break;
        }

        if pos.stm() != human_stm {
            let mov = ismcts.go(&mut pos);
            println!("info move {}{}", capture, mov);

            capture = make(&mut pos, &moves, &format!("{mov}"));
            continue;
        }

        println!("{}", pos.anonymize((pos.stm() as usize) ^ 1));
        println!("Choose move: [");
        moves.iter().for_each(|m| println!("  {m},"));
        println!("]");

        let mov = read();
        capture = make(&mut pos, &moves, &mov);
    }

    println!("Game state: {:?}", pos.game_state());
}

fn custom_deployment() -> String {
    let (red, blue) = deployment(&heuristic(1000, false), &heuristic(1000, false));
    println!("Example:\nRed: {}\nBlue: {}", red, blue);

    println!("Position: ");
    println!(
        "Flag {}\nSpy {}\nScout {}\nMiner {}\nGeneral {}\nMarshal {}\nBomb {}",
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

fn make(pos: &mut StrategoState, moves: &MoveList, mov_str: &str) -> String {
    for i in 0..moves.len() {
        let mov = moves[i];

        if format!("{mov}") != mov_str {
            continue;
        }

        let capture = if (mov.flag & Flag::CAPTURE) != 0 {
            format!("{}x", Piece::rank(pos.board().piece(mov.to)))
        } else {
            String::new()
        };

        pos.make(mov);

        return capture;
    }

    println!("error illegal move {}", mov_str);
    String::new()
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
