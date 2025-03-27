use super::{
    attacks,
    moves::{Move, MoveList, MoveStack, SquareMask},
    util::Flag,
    GameState,
};
use crate::{
    bitboard_loop,
    stratego::util::{Piece, Zobrist},
};
use std::cmp::Ordering;

/// Represents board from pov of one player
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    bb: [u64; 10],
    stm: bool,
    state: GameState,
    hash: u64,
    half: u16,
    attacker: u8,
    last: [SquareMask; 2],
    attacks: u64,
    evading: [bool; 2],
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DELIMITER: &str = concat!("+---+---+---+---+---+---+---+---+", '\n');

        let mut pos = self.chars();

        let mut lakes = Position::LAKES;
        bitboard_loop!(lakes, sq, pos[sq as usize] = '~');

        let mut pos_str = String::from(DELIMITER);
        let mut notation = String::new();

        for rank in (0..8).rev() {
            let start = rank * 8;

            let mut rank_notation = String::new();
            let mut rank_board = String::new();
            let mut spaces = 0;

            for &c in &pos[start..(start + 8)] {
                rank_board.push_str(&format!("| {c} "));

                if c == ' ' || c == '~' {
                    spaces += 1;

                    continue;
                }

                if spaces > 0 {
                    rank_notation.push_str(&spaces.to_string());
                    spaces = 0;
                }

                rank_notation.push(c);
            }

            if spaces > 0 {
                rank_notation.push_str(&spaces.to_string());
            }

            notation.push_str(&format!("{}/", rank_notation));
            pos_str.push_str(&format!("{}| {}\n{}", rank_board, rank + 1, DELIMITER));
        }

        // Remove last backslash
        notation.pop();

        notation.push(' ');
        notation.push(if self.stm { 'b' } else { 'r' });

        pos_str.push_str("  a   b   c   d   e   f   g   h");
        write!(f, "{pos_str}\n\nNotation: {}", notation)
    }
}

impl Position {
    pub const SYMBOLS: [char; 16] = [
        'F', 'S', 'C', 'D', 'G', 'M', 'X', 'B', 'f', 's', 'c', 'd', 'g', 'm', 'x', 'b',
    ];
    const LAKES: u64 = 0x2424000000;

    pub fn from(notation: &str) -> Self {
        let fields = notation.split(' ').collect::<Vec<&str>>();

        let mut pos = Self {
            bb: [0u64; 10],
            stm: false,
            state: GameState::default(),
            hash: 0,
            half: 0,
            attacker: 0,
            last: [SquareMask::default(); 2],
            attacks: 0,
            evading: [false; 2],
        };

        let (mut file, mut rank) = (0, 7);
        for c in fields[0].chars() {
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

    pub fn stm(&self) -> bool {
        self.stm
    }

    pub fn game_state(&self) -> GameState {
        self.state
    }

    pub fn set_game_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn game_over(&self) -> bool {
        self.state != GameState::Ongoing
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn half(&self) -> usize {
        self.half as usize
    }

    pub fn get(&self, index: usize) -> u64 {
        self.bb[index]
    }

    pub fn toggle(&mut self, stm: usize, piece: usize, sq: u8) {
        let bb = 1u64 << sq;

        self.hash ^= Zobrist::get(stm, sq as usize);

        self.bb[stm] ^= bb;
        self.bb[piece] ^= bb;
    }

    pub fn piece(&self, sq: u8) -> usize {
        let bb = 1u64 << sq;

        self.bb
            .iter()
            .skip(2)
            .position(|piece_bb| (piece_bb & bb) != 0)
            .unwrap_or(usize::MAX - 2)
            .wrapping_add(2)
    }

    pub fn make(&mut self, mov: &Move) {
        let stm = usize::from(self.stm);
        let piece = mov.piece as usize;

        // Increase two-squares counter if moving back to previous square or
        // traversing along path (piece must be on path already)
        self.last[stm].moves = if (self.last[stm].from == mov.to && self.last[stm].to == mov.from)
            || ((self.last[stm].path & (1u64 << mov.to)) != 0
                && (self.last[stm].path & (1u64 << mov.from)) != 0)
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
        self.evading[stm] = (mov.flag & Flag::EVADING) != 0;

        // Remove piece from old square
        if ((1u64 << mov.from) & self.bb[Piece::UNKNOWN]) != 0 {
            self.toggle(stm, Piece::UNKNOWN, mov.from);
        } else {
            self.toggle(stm, piece, mov.from);
        }

        // Add piece to new square when not capturing
        if (mov.flag & Flag::CAPTURE) == 0 {
            self.toggle(stm, piece, mov.to);
            self.half += 1;
            self.attacker = 0;

            return;
        }

        // Captures can only be done by piece with known rank
        assert!(piece != Piece::UNKNOWN);

        // Reset if board changes because of capture
        self.half = 0;
        self.attacker = mov.piece;

        let other = self.piece(mov.to);

        let ordering = if (piece == Piece::SPY && piece == Piece::MARSHAL)
            || (piece == Piece::MINER && other == Piece::BOMB)
        {
            // Spy can capture general or miner can defuse bomb
            Ordering::Greater
        } else {
            piece.cmp(&other)
        };

        match ordering {
            // Do nothing, because `attacker` is already removed
            Ordering::Less => {}
            // Delete only other, because `attacker` is already removed
            Ordering::Equal => self.toggle(stm ^ 1, other, mov.to),
            // Add `attacker` again
            Ordering::Greater => {
                self.toggle(stm, piece, mov.to);
                self.toggle(stm ^ 1, other, mov.to);
            }
        }

        let immovable = self.bb[Piece::FLAG] | self.bb[Piece::BOMB];

        // Current player has captured the flag
        if other == Piece::FLAG {
            self.state = GameState::Win;
        }
        // If all bitboards except the immovable pieces are empty the game is drawn
        else if ((self.bb[0] | self.bb[1]) ^ immovable) == 0 {
            self.state = GameState::Draw;
        }
        // If the current side has no pieces the other wins
        else if (self.bb[stm] & !immovable) == 0 {
            self.state = GameState::Loss;
        }
        // If other side has no pieces the current side wins
        else if (self.bb[stm ^ 1] & !immovable) == 0 {
            self.state = GameState::Win;
        }
    }

    pub fn gen(&self, stack: &MoveStack) -> MoveList {
        let mut moves = MoveList::default();
        // If opponent has won the game in the last turn, no moves are generated
        if self.state != GameState::Ongoing {
            return moves;
        }

        let stm = usize::from(self.stm);
        let attacks = self.attacks(stm ^ 1);
        let occ = self.bb[0] | self.bb[1] | Position::LAKES;

        let from_mask = if self.last[stm].from != u8::MAX {
            1u64 << self.last[stm].from
        } else {
            0
        };

        // Remove path from attacks on third repetition
        let square_mask = if self.last[stm].moves == 2 {
            self.last[stm].path | from_mask
        } else {
            0
        };

        for piece in Piece::SPY..=Piece::MARSHAL {
            let mut piece_mask = self.bb[piece] & self.bb[stm];

            bitboard_loop!(piece_mask, from, {
                let from_bb = 1u64 << from;

                let mut attack_mask = match piece {
                    Piece::SCOUT => attacks::ranged(from as usize, occ),
                    _ => attacks::orthogonal(from as usize),
                };

                // Moving back to previous square/path is forbidden on third time
                if self.last[stm].to == from {
                    attack_mask ^= square_mask;
                }

                // `occ` already includes `LAKES`
                let mut quiets = attack_mask & !occ;

                // If opponent's piece is chasing then all quiet moves are evading
                let move_flag = if (self.attacks & from_bb) != 0 {
                    Flag::EVADING
                } else {
                    Flag::QUIET
                };

                // Piece can't move back to old position when chasing except the previous position
                let repetitions = quiets & attacks & !from_mask;
                if self.evading[stm ^ 1] && repetitions != 0 {
                    quiets ^= self.repetition(stack, stm, from, repetitions);
                }

                bitboard_loop!(quiets, to, moves.push(from, to, move_flag, piece as u8));

                // `ranged` and `orthogonal` don't subtract `LAKES` implicitly
                let mut captures = attack_mask & self.bb[stm ^ 1] & !Position::LAKES;

                bitboard_loop!(
                    captures,
                    to,
                    moves.push(from, to, Flag::CAPTURE, piece as u8)
                );
            });
        }

        moves
    }

    fn attacks(&self, stm: usize) -> u64 {
        let mut bb = self.bb[stm];
        let mut attacks = 0;

        bitboard_loop!(bb, sq, attacks |= attacks::orthogonal(sq as usize));

        attacks
    }

    fn repetition(&self, stack: &MoveStack, stm: usize, from: u8, bb: u64) -> u64 {
        let hash = self.hash ^ Zobrist::get(stm, from as usize);

        let mut repetitions = 0;
        let mut bb = bb;
        bitboard_loop!(bb, sq, {
            if stack.repetition(self.half(), hash ^ Zobrist::get(stm, sq as usize)) {
                repetitions |= 1u64 << sq;
            }
        });

        repetitions
    }

    fn chars(&self) -> [char; 64] {
        let mut pos = [' '; 64];

        for piece in Piece::FLAG..=Piece::BOMB {
            let mut piece_mask = self.bb[piece];

            bitboard_loop!(piece_mask, sq, {
                let bb = 1 << sq as u64;
                let offset = if (bb & self.bb[0]) != 0 { 0 } else { 8 };

                if pos[sq as usize] != ' ' {
                    unreachable!()
                }

                pos[sq as usize] = Position::SYMBOLS[piece + offset - 2];
            });
        }

        pos
    }
}
