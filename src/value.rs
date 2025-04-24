use crate::stratego::StrategoState;
use heuristic::heuristic;
use simulation::{simulation_cutoff, simulation_ordered};
use std::sync::Arc;

pub use network::Network;
pub use simulation::simulation_uniform;

mod heuristic;
mod network;
mod simulation;

pub enum Value {
    SimulationUniform,
    SimulationOrdered,
    SimulationCutoff(f32),
    Heuristic,
    Network(Arc<Network>),
}

impl Value {
    pub fn get(&self, pos: &mut StrategoState) -> f32 {
        match self {
            Value::SimulationUniform => simulation_uniform(pos),
            Value::SimulationOrdered => simulation_ordered(pos),
            Value::SimulationCutoff(c) => simulation_cutoff(pos, *c),
            Value::Heuristic => heuristic(pos),
            Value::Network(nn) => nn.get(pos),
        }
    }
}
