use crate::stratego::util::{Flag, Piece};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub flag: u8,
    pub piece: u8,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn to_notation(sq: u8) -> String {
            let file = sq & 7;
            let rank = sq >> 3;

            format!("{}{}", (b'a' + file) as char, rank + 1)
        }

        let prefix = if (self.flag & Flag::ATTACKED) != 0 {
            format!("{}x", Piece::rank(usize::from(self.flag >> 5)))
        } else {
            String::new()
        };

        let suffix = if (self.flag & Flag::CAPTURE) != 0 {
            format!("x{}", Piece::rank(self.piece as usize))
        } else {
            String::new()
        };

        write!(
            f,
            "{}{}{}{}",
            prefix,
            to_notation(self.from),
            to_notation(self.to),
            suffix
        )
    }
}

pub struct MoveList {
    length: usize,
    moves: [Move; 218],
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList {
            moves: [Move::default(); 218],
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
    pub fn iter(&self) -> impl Iterator<Item = Move> + '_ {
        self.moves[..self.length].iter().cloned()
    }

    pub fn length(&self) -> usize {
        self.length
    }

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

#[derive(Clone, Default)]
pub struct MoveStack {
    stack: Vec<u64>,
    ply: u16,
}

impl MoveStack {
    pub fn repetition(&self, half: usize, current: u64) -> bool {
        if self.stack.len() < 7 {
            return false;
        }

        for &hash in self.stack.iter().rev().take(half + 1).skip(1).step_by(2) {
            if hash == current {
                return true;
            }
        }

        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &u64> {
        self.stack.iter()
    }

    pub fn push(&mut self, hash: u64) {
        self.ply += 1;
        self.stack.push(hash);
    }

    pub fn pop(&mut self) {
        self.ply -= 1;
        self.stack.pop();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SquareMask {
    pub moves: u8,
    pub from: u8,
    pub to: u8,
    pub path: u64,
}

impl Default for SquareMask {
    fn default() -> Self {
        SquareMask {
            moves: 0,
            from: u8::MAX,
            to: u8::MAX,
            path: 0,
        }
    }
}
