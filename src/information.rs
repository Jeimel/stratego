use crate::{stratego::StrategoState, value::evaluate};
use ordered_float::OrderedFloat;

pub enum Information {
    Random,
    Heuristic(usize),
}

impl Information {
    pub fn get(&self, pos: &StrategoState) -> StrategoState {
        match self {
            Information::Random => pos.determination(),
            Information::Heuristic(attempts) => {
                let (det, _) = (0..*attempts)
                    .map(|_| {
                        let mut det = pos.determination();
                        let score = evaluate(&mut det);

                        (det, score)
                    })
                    .max_by_key(|(_, score)| OrderedFloat::from(*score))
                    .unwrap();

                det
            }
        }
    }
}
