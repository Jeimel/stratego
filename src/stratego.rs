mod attacks;
mod moves;
mod position;
mod util;

pub use moves::{Move, MoveList, MoveStack};
pub use position::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Ongoing,
    Win,
    Draw,
    Loss,
}
