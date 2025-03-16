use std::time::Instant;

use stratego::{Move, MoveStack, Position};

pub mod mcts;
pub mod stratego;
pub mod tournament;

const EMPTY_POS: &str = "8/8/8/8/8/8/8/8 r";

pub trait Protocol: Default {
    fn run(&mut self) {
        Self::identify();

        let mut pos = Position::from(EMPTY_POS);
        let mut stack = MoveStack::default();

        loop {
            let mut input = String::new();
            let bytes = std::io::stdin().read_line(&mut input).unwrap();

            if bytes == 0 {
                break;
            }

            let commands = input.split_whitespace().collect::<Vec<_>>();
            let command = *commands.first().unwrap();

            match command {
                "quit" => std::process::exit(0),
                "setoption" => self.handle_options(commands),
                "position" => self.set_position(commands, &mut pos, &mut stack),
                "newgame" => {
                    pos = Position::from(EMPTY_POS);
                    stack = MoveStack::default();
                }
                "isready" => println!("readyok"),
                "deployment" => println!("deployment {}", self.handle_deployment(commands)),
                "go" => println!("bestmove {}", self.handle_go(commands, &pos, &stack)),
                "perft" => run_perft(&pos, &mut stack.clone(), 5),
                "d" => println!("{pos}"),
                _ => println!("Unknown command: {command}"),
            }
        }
    }

    fn identify() {
        println!("{}", preamble());
        println!("{}\nok", Self::option());
    }

    fn set_position(&mut self, commands: Vec<&str>, pos: &mut Position, stack: &mut MoveStack) {
        let mut notation = String::from(commands[1]);
        notation.push(' ');
        notation.push_str(commands[2]);

        *pos = Position::from(&notation);
        stack.push(pos.hash());

        for mov in commands.iter().skip(4) {
            if let Some(mov) = pos.gen(stack).iter().find(|m| *mov == format!("{m}")) {
                pos.make(&mov);
                stack.push(pos.hash());
            } else {
                unreachable!()
            }
        }
    }

    /// Must return list of valid options in the format:
    /// setoption name <name> = <value>
    fn option() -> String;

    /// Must return a valid deployment in notation
    fn handle_deployment(&self, commands: Vec<&str>) -> String;

    /// Must return a valid move in the given position
    fn handle_go(&mut self, commands: Vec<&str>, pos: &Position, stack: &MoveStack) -> Move;

    /// Set options for the given algorithm
    fn handle_options(&mut self, commands: Vec<&str>);
}

fn preamble() -> String {
    String::from(concat!(
        "id name stratego",
        '\n',
        "id author Felix Jablinski"
    ))
}

fn run_perft(pos: &Position, stack: &mut MoveStack, depth: usize) {
    let now = Instant::now();
    let nodes = perft(pos, stack, depth);

    let time = now.elapsed().as_micros();
    println!(
        "perft {depth} time {} nodes {nodes} ({:.2} Mnps)",
        time / 1000,
        nodes as f32 / time as f32
    );
}

fn perft(pos: &Position, stack: &mut MoveStack, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    let moves = pos.gen(stack);
    for i in 0..moves.length() {
        let mut new_pos = *pos;

        new_pos.make(&moves[i]);
        stack.push(new_pos.hash());
        let child_nodes = perft(&new_pos, stack, depth - 1);
        stack.pop();

        nodes += child_nodes;
    }

    nodes
}
