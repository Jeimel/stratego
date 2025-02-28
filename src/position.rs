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
