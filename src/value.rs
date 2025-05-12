use crate::stratego::StrategoState;
use simulation::{simulation_cutoff, simulation_ordered, simulation_ordered_cutoff};

pub use heuristic::{evaluate, heuristic};
pub use network::Network;
pub use simulation::simulation_uniform;

mod heuristic;
mod network;
mod simulation;

type Heuristic = fn(&mut StrategoState) -> f32;

pub enum Value {
    SimulationUniform,
    SimulationOrdered([f32; 5]),
    SimulationCutoff(f32, Heuristic),
    SimulationOrderedCutoff([f32; 5], f32, Heuristic),
    Heuristic(f32),
    Network(Network, [f32; 5], f32),
}

impl Value {
    pub fn get(&self, pos: &mut StrategoState) -> f32 {
        match self {
            Value::SimulationUniform => simulation_uniform(pos),
            Value::SimulationOrdered(weights) => simulation_ordered(pos, weights),
            Value::SimulationCutoff(c, heuristic) => simulation_cutoff(pos, *c, *heuristic),
            Value::SimulationOrderedCutoff(weights, c, heuristic) => {
                simulation_ordered_cutoff(pos, weights, *c, *heuristic)
            }
            Value::Heuristic(scaling) => heuristic(pos, *scaling),
            Value::Network(nn, weights, c) => {
                let stm = pos.stm();

                let result =
                    simulation_ordered_cutoff(pos, weights, *c, |_: &mut StrategoState| 5.0);

                if result == 1.0 || result == -1.0 || result == 0.0 {
                    return result;
                }

                let current = f32::from(stm == pos.stm());
                nn.get(pos) * (-1.0 + 2.0 * current)
            }
        }
    }
}
