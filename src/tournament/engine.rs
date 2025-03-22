use crate::{
    stratego::{Move, StrategoState},
    Algorithm,
};

pub struct Engine {
    name: String,
    algorithm: Algorithm,
    cheating: bool,
}

impl Engine {
    pub fn new(name: &str, algorithm: Algorithm, cheating: bool) -> Self {
        Self {
            name: name.to_string(),
            algorithm,
            cheating,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cheating(&self) -> bool {
        self.cheating
    }

    pub fn go(&mut self, pos: StrategoState) -> Move {
        Algorithm::go(&mut self.algorithm, &pos)
    }

    pub fn deployment(&mut self) -> String {
        Algorithm::deployment(&mut self.algorithm)
    }
}
