use crate::stratego::StrategoState;
use heuristic::heuristic;
use simulation::simulation_ordered;

pub use heuristic::MAX;
pub use simulation::simulation_uniform;

mod heuristic;
mod simulation;

pub enum Value {
    SimulationUniform,
    SimulationOrdered,
    Heuristic,
}

impl Value {
    pub fn get(&self, pos: &mut StrategoState) -> f32 {
        match self {
            Value::SimulationUniform => simulation_uniform(pos),
            Value::SimulationOrdered => simulation_ordered(pos),
            Value::Heuristic => heuristic(pos),
        }
    }
}
