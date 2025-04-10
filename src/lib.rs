use mcts::{ISMCTS, MCTS, PIMC};
use rand::{rng, seq::IndexedRandom};
use random::UniformRandom;
use stratego::{Move, StrategoState};

pub mod mcts;
pub mod random;
pub mod stratego;
pub mod tournament;

// Deployments from pov of blue
pub const DEPLOYMENTS: [&str; 2] = ["5c2/2csdbd1/3gbfm1", "5cbf/5cdb/1gsd3m"];

pub enum Algorithm {
    MCTS(MCTS),
    PIMC(PIMC),
    ISMCTS(ISMCTS),
    Random(UniformRandom),
}

impl Algorithm {
    pub fn go(&mut self, pos: &StrategoState) -> Move {
        match self {
            Algorithm::MCTS(a) => a.go(pos),
            Algorithm::PIMC(a) => a.go(pos),
            Algorithm::ISMCTS(a) => a.go(pos),
            Algorithm::Random(a) => a.go(pos),
        }
    }

    pub fn deployment(&mut self) -> String {
        match self {
            Algorithm::MCTS(_) => random_deployment(),
            Algorithm::PIMC(_) => random_deployment(),
            Algorithm::ISMCTS(_) => random_deployment(),
            Algorithm::Random(a) => a.deployment(),
        }
    }
}

fn random_deployment() -> String {
    let mut rng = rng();

    DEPLOYMENTS.choose(&mut rng).unwrap().to_string()
}
