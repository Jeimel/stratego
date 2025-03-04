use std::cmp::Ordering;

use crate::{
    attacks, bitboard_loop,
    util::{Flag, Piece},
};

pub struct Move {
    pub from: u8,
    pub to: u8,
    pub flag: u8,
    pub piece: u8,
}

impl Move {
    pub const NULL: Move = Move {
        from: 0,
        to: 0,
        flag: 0,
        piece: 0,
    };
}

pub struct MoveList {
    pub length: usize,
    moves: [Move; 218],
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList {
            moves: [Move::NULL; 218],
            length: 0,
        }
    }
}

impl std::ops::Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        &self.moves[index]
    }
}

impl MoveList {
    pub fn push(&mut self, from: u8, to: u8, flag: u8, piece: u8) {
        self.moves[self.length] = Move {
            from,
            to,
            flag,
            piece,
        };
        self.length += 1;
    }
}

/// Represents board from pov of one player
#[derive(Clone, Copy)]
pub struct Position {
    bb: [u64; 10],
    stm: usize,
    moves: u16,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DELIMITER: &'static str = concat!("+---+---+---+---+---+---+---+---+", '\n');

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
            stm: 0,
            moves: 0,
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

                    let sq = 1 << (rank * 8 + file);

                    pos.bb[side] |= sq;
                    pos.bb[piece] |= sq;

                    file += 1;
                }
            };
        }

        pos.stm = if fields[1] == "r" { 0 } else { 1 };
        pos.moves = fields[2].parse().unwrap();

        pos
    }
}
