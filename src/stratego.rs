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

        let red_bb = self.board.get(0);
        let blue_bb = self.board.get(1);
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

            let mut red_bb = red_bb & pieces;
            let mut blue_bb = blue_bb & pieces;

            if STM == 1 {
                red_bb = flip_bb(red_bb);
                blue_bb = flip_bb(blue_bb);
            }

            bitboard_loop!(red_bb, sq, {
                let pc_idx = i * 2 + 0;
                let halfkp = sq as usize + pc_idx * StrategoState::BOARD;

                features[halfkp] = 1.0;
            });

            bitboard_loop!(blue_bb, sq, {
                let pc_idx = i * 2 + 1;
                let halfkp = sq as usize + pc_idx * StrategoState::BOARD;

                features[halfkp] = 1.0;
            });
        }

        features
    }

    pub fn determination(&self) -> Self {
        const BASE: [u64; 2] = [0xffffff, 0xffffff0000000000];

        let mut pos = self.clone();
        let mut rng = rand::rng();

        let mut red = pos.info.available_immovable(0);
        let mut blue = pos.info.available_immovable(1);

        red.shuffle(&mut rng);
        blue.shuffle(&mut rng);

        let unknown = pos.board.get(Piece::UNKNOWN) & BASE[0] & pos.board.get(0);
        StrategoState::determinize_bb(&mut pos, 0, unknown, &mut red);

        let unknown = pos.board.get(Piece::UNKNOWN) & BASE[1] & pos.board.get(1);
        StrategoState::determinize_bb(&mut pos, 1, unknown, &mut blue);

        let mut red = pos.info.available(0);
        let mut blue = pos.info.available(1);

        red.shuffle(&mut rng);
        blue.shuffle(&mut rng);

        let unknown = pos.board.get(Piece::UNKNOWN) & pos.board.get(0);
        StrategoState::determinize_bb(&mut pos, 0, unknown, &mut red);

        let unknown = pos.board.get(Piece::UNKNOWN) & pos.board.get(1);
        StrategoState::determinize_bb(&mut pos, 1, unknown, &mut blue);

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
            let new = pieces.pop();
            if let Some(new) = new {
                pos.board.toggle(stm, Piece::UNKNOWN, sq);
                pos.board.toggle(stm, new, sq);
            }
        });
    }
}
