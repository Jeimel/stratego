use super::{util::Flag, Move, Piece, Position};
use crate::bitboard_loop;

/// Keeps track of all legal information,
/// which can be retrieved from the `StrategoState`
///
/// What moves reveal legal information?
/// - moves of absolute distance > 1 (Piece must be Scout)
/// - captures (Piece ranks must be revealed)
#[derive(Clone)]
pub struct InformationSet {
    unknown: [[usize; 10]; 2],
    bb: [u64; 2],
}

impl InformationSet {
    // All pieces are `UNKNOWN`
    pub fn from(board: &Position) -> Self {
        let mut unknown = [[0usize; 10]; 2];
        let mut bb = [0u64; 2];

        for piece in Piece::FLAG..=Piece::BOMB {
            let occ = board.get(piece);

            for stm in 0..2 {
                let mut side = board.get(stm) & occ;

                unknown[stm][piece] += side.count_ones() as usize;
                bitboard_loop!(side, sq, bb[stm] |= 1u64 << sq);
            }
        }

        InformationSet { unknown, bb }
    }

    pub fn available(&self, stm: usize) -> Vec<usize> {
        self.unknown[stm]
            .iter()
            .enumerate()
            .skip(2)
            .filter(|(_, &pc)| pc > 0usize)
            .flat_map(|(i, &pc)| std::iter::repeat(i).take(pc))
            .collect()
    }

    pub fn get(&self, stm: usize) -> u64 {
        self.bb[stm]
    }

    // TODO: create bitboard for start pos
    //  - if only 3 bits are left from difference pieces must be bomb or flag
    pub fn update(&mut self, mov: &Move, board: &Position) {
        let stm = board.stm() as usize;
        let piece = mov.piece as usize;

        if (mov.flag & Flag::CAPTURE) != 0 && ((1u64 << mov.to) & self.bb[stm ^ 1]) != 0 {
            self.remove(stm ^ 1, board.piece(mov.to), mov.to);
        }

        if ((1u64 << mov.from) & self.bb[stm]) == 0 {
            return;
        }

        self.bb[stm] ^= 1u64 << mov.from;
        self.bb[stm] ^= 1u64 << mov.to;

        if (mov.flag & Flag::CAPTURE) != 0 {
            self.remove(stm, piece, mov.to);
        } else if distance(mov.from as i32, mov.to as i32) > 1 {
            self.remove(stm, Piece::SCOUT, mov.to);
        }
    }

    pub fn remove(&mut self, stm: usize, piece: usize, sq: u8) {
        self.bb[stm] ^= 1u64 << sq;
        self.unknown[stm][piece] -= 1;
    }
}

fn distance(from: i32, to: i32) -> usize {
    let rank = ((from >> 3) - (to >> 3)).abs() as usize;
    let file = ((from & 7) - (to & 7)).abs() as usize;

    rank + file
}
