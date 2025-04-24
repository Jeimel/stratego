use crate::{
    deployment::Deployment,
    stratego::{Move, StrategoState},
};
use rand::{rng, seq::IteratorRandom};

pub struct UniformRandom {
    deployment: Deployment,
}

impl UniformRandom {
    pub fn new(deployment: Deployment) -> Self {
        Self { deployment }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        let mut rng = rng();

        pos.clone()
            .gen()
            .iter()
            .choose(&mut rng)
            .expect("valid move")
    }

    pub fn deployment(&mut self) -> String {
        self.deployment.get()
    }
}
