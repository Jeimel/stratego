#[derive(Copy, Clone, Default)]
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

        write!(f, "{}{}", to_notation(self.from), to_notation(self.to))
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
