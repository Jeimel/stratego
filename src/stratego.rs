mod attacks;
mod information;
mod moves;
mod position;
mod util;

use crate::bitboard_loop;
use information::InformationSet;
pub use moves::{Move, MoveList, MoveStack};
pub use position::Position;
use rand::seq::SliceRandom;
pub use util::Piece;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Ongoing,
    Win,
    Draw,
    Loss,
}

#[derive(Clone)]
pub struct StrategoState {
    board: Position,
    stack: MoveStack,
    info: InformationSet,
}

impl std::fmt::Display for StrategoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}", self.info);

        write!(f, "{}", self.board)
    }
}

impl StrategoState {
    pub fn from(notation: &str) -> Self {
        let board = Position::from(notation);

        Self {
            board,
            stack: MoveStack::default(),
            info: InformationSet::from(&board),
        }
    }

    pub fn board(&self) -> Position {
        self.board
    }

    pub fn gen(&mut self) -> MoveList {
        self.board.gen(&self.stack)
    }

    pub fn make(&mut self, mov: Move) {
        self.info.update(&mov, &self.board);
        self.board.make(&mov);
        self.stack.push(self.board.hash());
    }

    pub fn determination(&self) -> Self {
        let mut pos = self.clone();
        let mut rng = rand::rng();

        let mut red = pos.info.available(0);
        let mut blue = pos.info.available(1);

        red.shuffle(&mut rng);
        blue.shuffle(&mut rng);

        let mut unknown = pos.board.get(Piece::UNKNOWN);
        bitboard_loop!(unknown, sq, {
            let stm = usize::from((pos.board.get(0) & (1 << sq)) == 0);

            let new = if stm == 0 {
                red.pop().unwrap()
            } else {
                blue.pop().unwrap()
            };

            pos.board.toggle(stm, Piece::UNKNOWN, sq);
            pos.board.toggle(stm, new, sq);
        });

        pos
    }

    pub fn anonymize(&self, stm: usize) -> Self {
        let mut pos = self.clone();

        let mut bb = self.info.get(stm);
        bitboard_loop!(bb, sq, {
            let piece = pos.board.piece(sq);

            pos.board.toggle(stm, piece, sq);
            pos.board.toggle(stm, Piece::UNKNOWN, sq);
        });

        pos
    }

    pub fn game_state(&self) -> GameState {
        self.board.game_state()
    }

    pub fn set_game_state(&mut self, state: GameState) {
        self.board.set_game_state(state);
    }

    pub fn hash(&self) -> u64 {
        self.board.hash()
    }

    pub fn stm(&self) -> bool {
        self.board.stm()
    }

    pub fn game_over(&self) -> bool {
        self.board.game_over()
    }
}
