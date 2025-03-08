use crate::moves::{Move, MoveList, MoveStack, SquareMask};
use crate::util::Zobrist;
use crate::{
    attacks, bitboard_loop,
    util::{Flag, Piece},
};
use std::cmp::Ordering;

/// Represents board from pov of one player
#[derive(Clone, Copy)]
pub struct Position {
    bb: [u64; 10],
    stm: bool,
    hash: u64,
    half: u16,
    last: [SquareMask; 2],
    attacks: u64,
    evading: [bool; 2],
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DELIMITER: &str = concat!("+---+---+---+---+---+---+---+---+", '\n');

        let mut pos = [' '; 64];
        for piece in Piece::FLAG..=Piece::BOMB {
            let mut piece_mask = self.bb[piece];

            bitboard_loop!(piece_mask, sq, {
                let bb = 1 << sq as u64;
                let offset = if (bb & self.bb[0]) != 0 { 0 } else { 8 };

                pos[sq as usize] = Position::SYMBOLS[piece + offset - 2];
            });
        }

        let mut lakes = Position::LAKES;
        bitboard_loop!(lakes, sq, pos[sq as usize] = '~');

        let mut pos_str = String::from(DELIMITER);

        for rank in (0..8).rev() {
            let start = rank * 8;
            let rank_str = &pos[start..(start + 8)]
                .iter()
                .fold(String::new(), |mut acc, &c| {
                    acc.push_str(&format!("| {} ", c));
                    acc
                });

            pos_str.push_str(&format!("{}| {}\n{}", rank_str, rank + 1, DELIMITER));
        }

        pos_str.push_str("  a   b   c   d   e   f   g   h");
        write!(f, "{}", pos_str)
    }
}

impl Position {
    const SYMBOLS: [char; 16] = [
        'F', 'S', 'C', 'D', 'G', 'M', 'X', 'B', 'f', 's', 'c', 'd', 'g', 'm', 'x', 'b',
    ];
    const LAKES: u64 = 0x2424000000;

    pub fn from(notation: &str) -> Self {
        let fields = notation.split(' ').collect::<Vec<&str>>();

        let mut pos = Position {
            bb: [0u64; 10],
            stm: false,
            hash: 0,
            half: 0,
            last: [SquareMask::default(); 2],
            attacks: 0,
            evading: [false; 2],
        };

        let (mut file, mut rank) = (0, 7);
        for c in fields[0].chars().collect::<Vec<_>>() {
            match c {
                c if c.is_numeric() => file += c as u32 - '0' as u32,
                '/' => (file, rank) = (0, rank - 1),
                _ => {
                    let side = c.is_ascii_lowercase() as usize;
                    let c = c.to_ascii_uppercase();
                    let piece = Position::SYMBOLS
                        .iter()
                        .position(|&symbol| symbol == c)
                        .unwrap()
                        .wrapping_add(2);

                    pos.toggle(side, piece, (rank * 8 + file) as u8);

                    file += 1;
                }
            };
        }

        pos.stm = fields[1] != "r";

        pos
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn half(&self) -> usize {
        self.half as usize
    }

    pub fn game_over(&self, stm: usize) -> bool {
        (self.bb[Piece::FLAG] & self.bb[stm]) == 0
    }

    pub fn make(&mut self, mov: &Move) {
        let stm = usize::from(self.stm);
        let piece = mov.piece as usize;

        // Increase two-squares counter if moving back to previous square or traversing along path
        self.last[stm].moves = if (self.last[stm].from == mov.to && self.last[stm].to == mov.from)
            || (self.last[stm].path & (1u64 << mov.to)) != 0
        {
            self.last[stm].moves + 1
        } else {
            // Store traversed path if moved piece is `SCOUT`
            self.last[stm].path = if usize::from(mov.piece) == Piece::SCOUT {
                attacks::between_squares(mov.from, mov.to)
            } else {
                0
            };

            0
        };

        self.stm ^= true;
        // Store last move for current side to enforce two-squares rule
        self.last[stm].from = mov.from;
        self.last[stm].to = mov.to;
        // Store possible attacks in next turn to check if opponent is evading
        self.attacks = attacks::orthogonal(mov.to as usize);
        // Check if next move can't be repetitive
        self.evading[stm] = mov.flag == Flag::EVADING;

        if mov.flag == Flag::CHANCE {
            // Remove piece from 'UNKNOWN' bitboard
            self.toggle(stm, Piece::UNKNOWN, mov.from);
        } else {
            // Remove piece from old square
            self.toggle(stm, piece, mov.from);
        }

        // Add piece to new square when not capturing
        if mov.flag == Flag::CAPTURE {
            self.toggle(stm, piece, mov.to);
            self.half += 1;

            return;
        }

        // Reset if board changes because of capture
        self.half = 0;

        let other = self.piece(mov.to);

        // Spy can capture general or only miner can defuse bomb
        if (piece == Piece::SPY && other == Piece::GENERAL)
            || (piece == Piece::MINER && other == Piece::BOMB)
        {
            self.toggle(stm ^ 1, other, mov.to);
            self.toggle(stm, piece, mov.to);

            return;
        }

        match piece.cmp(&other) {
            // Do nothing, because piece is already removed
            Ordering::Less => {}
            // Delete only other, because piece is already removed
            Ordering::Equal => self.toggle(stm ^ 1, other, mov.to),
            // Delete other and add piece
            Ordering::Greater => {
                self.toggle(stm ^ 1, other, mov.to);
                self.toggle(stm, piece, mov.to);
            }
        }
    }

    fn piece(&self, sq: u8) -> usize {
        let bb = 1u64 << sq;

        self.bb
            .iter()
            .skip(2)
            .position(|piece_bb| (piece_bb & bb) != 0)
            .unwrap_or(usize::MAX - 2)
            .wrapping_add(2)
    }

    fn toggle(&mut self, stm: usize, piece: usize, sq: u8) {
        let bb = 1u64 << sq;

        self.hash ^= Zobrist::get(stm, sq as usize);

        self.bb[stm] ^= bb;
        self.bb[piece] ^= bb;
    }
}
