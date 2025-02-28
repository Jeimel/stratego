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
