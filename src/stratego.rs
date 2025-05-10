mod attacks;
mod information;
mod moves;
mod position;
mod util;

pub use attacks::{chebyshev, orthogonal, ranged};
pub use moves::{Move, MoveList, MoveStack};
pub use position::Position;
pub use util::{flip_bb, Flag, Piece};

use crate::bitboard_loop;
use information::InformationSet;
use rand::seq::SliceRandom;

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
        write!(f, "{}", self.board)
    }
}

impl StrategoState {
    const PIECES: usize = 7;
    const BOARD: usize = 64;
    pub const FEATURES: usize = StrategoState::BOARD * StrategoState::PIECES * 2;

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

    pub fn information(&self) -> InformationSet {
        self.info
    }

    pub fn gen(&self) -> MoveList {
        self.board.gen(&self.stack)
    }

    pub fn make(&mut self, mov: Move) {
        self.info.update(&mov, &self.board);
        self.board.make(&mov);
        self.stack.push(self.board.hash());
    }

    pub fn features<const STM: usize>(&self) -> [f32; StrategoState::FEATURES] {
        let mut features = [0f32; StrategoState::FEATURES];

        let us_bb = self.board.get(STM);
        let them_bb = self.board.get(STM ^ 1);
        for i in 0..StrategoState::PIECES {
            let piece = match i {
                0 => Piece::FLAG,
                1 => Piece::SPY,
                2 => Piece::SCOUT,
                3 => Piece::MINER,
                4 => Piece::GENERAL,
                5 => Piece::MARSHAL,
                6 => Piece::BOMB,
                _ => unreachable!(),
            };

            let pieces = self.board.get(piece);

            let mut us_bb = us_bb & pieces;
            let mut them_bb = them_bb & pieces;

            if STM == 1 {
                us_bb = flip_bb(us_bb);
                them_bb = flip_bb(them_bb);
            }

            bitboard_loop!(us_bb, sq, {
                let pc_idx = i * 2 + 0;
                let halfkp = sq as usize + pc_idx * StrategoState::BOARD;

                features[halfkp] = 1.0;
            });

            bitboard_loop!(them_bb, sq, {
                let pc_idx = i * 2 + 1;
                let halfkp = sq as usize + pc_idx * StrategoState::BOARD;

                features[halfkp] = 1.0;
            });
        }

        features
    }

    pub fn determination(&self) -> Self {
        let mut pos = self.clone();
        let mut rng = rand::rng();

        let mut red = pos.info.available_immovable(0);
        let mut blue = pos.info.available_immovable(1);

        red.shuffle(&mut rng);
        blue.shuffle(&mut rng);

        let unknown = pos.board.get(Piece::UNKNOWN) & self.info.initial(0) & pos.board.get(0);
        if unknown != 0 {
            StrategoState::determinize_bb(&mut pos, 0, unknown, &mut red);
        }

        let unknown = pos.board.get(Piece::UNKNOWN) & self.info.initial(1) & pos.board.get(1);
        if unknown != 0 {
            StrategoState::determinize_bb(&mut pos, 1, unknown, &mut blue);
        }

        let mut red = pos.info.available(0);
        let mut blue = pos.info.available(1);

        red.shuffle(&mut rng);
        blue.shuffle(&mut rng);

        let unknown = pos.board.get(Piece::UNKNOWN) & pos.board.get(0);
        if unknown != 0 {
            StrategoState::determinize_bb(&mut pos, 0, unknown, &mut red);
        }

        let unknown = pos.board.get(Piece::UNKNOWN) & pos.board.get(1);
        if unknown != 0 {
            StrategoState::determinize_bb(&mut pos, 1, unknown, &mut blue);
        }

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

    fn determinize_bb(pos: &mut StrategoState, stm: usize, bb: u64, pieces: &mut Vec<usize>) {
        let mut bb = bb;

        bitboard_loop!(bb, sq, {
            if let Some(new) = pieces.pop() {
                pos.board.toggle(stm, Piece::UNKNOWN, sq);
                pos.board.toggle(stm, new, sq);
            }
        });

        assert!(pieces.len() == 0);
    }
}
