use crate::stratego::StrategoState;
use simulation::{simulation_cutoff, simulation_ordered};

pub use heuristic::{evaluate, heuristic};
pub use network::Network;
pub use simulation::simulation_uniform;

mod heuristic;
mod network;
mod simulation;

pub enum Value {
    SimulationUniform,
    SimulationOrdered,
    SimulationCutoff(f32),
    Heuristic(f32),
    Network(Network),
}

impl Value {
    pub fn get(&self, pos: &mut StrategoState) -> f32 {
        match self {
            Value::SimulationUniform => simulation_uniform(pos),
            Value::SimulationOrdered => simulation_ordered(pos),
            Value::SimulationCutoff(c) => simulation_cutoff(pos, *c),
            Value::Heuristic(scaling) => heuristic(pos, *scaling),
            Value::Network(nn) => nn.get(pos),
        }
    }
}
