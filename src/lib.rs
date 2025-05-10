use mcts::{Search, ISMCTS, MCTS, PIMC};
use random::UniformRandom;
use stratego::{Move, StrategoState};

pub mod deployment;
pub mod information;
pub mod mcts;
pub mod policy;
pub mod random;
pub mod select;
pub mod stratego;
pub mod tournament;
pub mod value;

pub enum Algorithm {
    MCTS(MCTS),
    PIMC(PIMC),
    SOISMCTS(ISMCTS<false>),
    MOISMCTS(ISMCTS<true>),
    Random(UniformRandom),
}

impl Algorithm {
    pub fn go(&mut self, pos: &StrategoState) -> Move {
        match self {
            Algorithm::MCTS(a) => a.go(pos),
            Algorithm::PIMC(a) => a.go(pos),
            Algorithm::SOISMCTS(a) => a.go(pos),
            Algorithm::MOISMCTS(a) => a.go(pos),
            Algorithm::Random(a) => a.go(pos),
        }
    }

    pub fn deployment(&mut self) -> String {
        match self {
            Algorithm::MCTS(a) => a.deployment(),
            Algorithm::PIMC(a) => a.deployment(),
            Algorithm::SOISMCTS(a) => a.deployment(),
            Algorithm::MOISMCTS(a) => a.deployment(),
            Algorithm::Random(a) => a.deployment(),
        }
    }
}
