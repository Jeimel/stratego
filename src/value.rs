use crate::stratego::StrategoState;
use simulation::{simulation_cutoff, simulation_ordered};

pub use heuristic::{evaluate, heuristic};
pub use network::Network;
pub use simulation::simulation_uniform;

mod heuristic;
mod network;
mod simulation;

type Heuristic = fn(&mut StrategoState) -> f32;

pub enum Value {
    SimulationUniform,
    SimulationOrdered([usize; 5]),
    SimulationCutoff(f32, Heuristic),
    Heuristic(f32),
    Network(Network),
}

impl Value {
    pub fn get(&self, pos: &mut StrategoState) -> f32 {
        match self {
            Value::SimulationUniform => simulation_uniform(pos),
            Value::SimulationOrdered(weights) => simulation_ordered(pos, *weights),
            Value::SimulationCutoff(c, heuristic) => simulation_cutoff(pos, *c, *heuristic),
            Value::Heuristic(scaling) => heuristic(pos, *scaling),
            Value::Network(nn) => nn.get(pos),
        }
    }
}
