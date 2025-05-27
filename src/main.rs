use std::{io::stdin, process, time::Instant};
use stratego::information::Information;
use stratego::mcts::ISMCTS;
use stratego::stratego::{MoveList, StrategoState};
use stratego::{deployment::Deployment, policy::Policy, select::Select, value::Value};

fn main() {
    let mut pos = StrategoState::from("d2f4/bbg4c/1m3dsc/8/8/BD3M1G/F5SD/1BCC4 r");

    let game_moves = [];
    for mov in game_moves {
        let pos_gen = pos.gen();
        make_move(&mut pos, &pos_gen, mov);

        println!("{}", mov);
        println!("{}", pos);
    }

    let mut ismcts = ISMCTS::<false>::new(
        10_000,
        Value::SimulationUniform,
        Policy::Uniform,
        Select::ISUCT(1.41),
        Deployment::Dataset,
        Information::Random,
    );

    loop {
        let moves = pos.gen();

        let mut input = String::new();
        let bytes_read = stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            process::exit(0);
        }

        let commands = input.split_ascii_whitespace().collect::<Vec<_>>();

        let first = *commands.first().unwrap();
        match first {
            "quit" => std::process::exit(0),
            "d" => println!("{pos}"),
            "moves" => moves.iter().for_each(|m| println!("{m}")),
            "move" => make_move(&mut pos, &moves, commands[1]),
            "perft" => run_perft(&pos, 6),
            "go" => {
                let annonym = pos.anonymize(pos.stm() as usize ^ 1);

                println!("info ismcts move {}", ismcts.go(&annonym));
            }
            "annonym" => println!("{}", pos.anonymize(pos.stm() as usize ^ 1)),
            "deter" => {
                let annonym = pos.anonymize(pos.stm() as usize ^ 1);
                println!("{}", annonym.determination());
            }
            _ => {}
        }
    }
}

fn make_move(pos: &mut StrategoState, moves: &MoveList, mov_str: &str) {
    for i in 0..moves.len() {
        let mov = moves[i];

        if format!("{mov}") == mov_str {
            pos.make(mov);

            return;
        }
    }

    println!("error illegal move {}", mov_str);
}

fn run_perft(pos: &StrategoState, depth: usize) {
    let pos = pos.clone();

    let now = Instant::now();
    let nodes = perft(pos, depth);

    let time = now.elapsed().as_micros();
    println!(
        "perft {depth} time {} nodes {nodes} ({:.2} Mnps)",
        time / 1000,
        nodes as f32 / time as f32
    );
}

fn perft(pos: StrategoState, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    for mov in pos.gen().iter() {
        let mut new_pos = pos.clone();

        new_pos.make(mov);
        let child_nodes = perft(new_pos, depth - 1);

        nodes += child_nodes;
    }

    nodes
}
