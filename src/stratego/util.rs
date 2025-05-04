macro_rules! c_enum {
    ($name:ident { $($(#[$meta:meta])? $key:ident: $type:ty = $value:expr,)+ }) => {
        pub struct $name;

        impl $name {
            $(
                $(#[$meta])?
                pub const $key: $type = $value;
            )+
        }
    };
}

#[macro_export]
macro_rules! bitboard_loop {
    ($bitboard:expr, $square:ident, $func: expr) => {
        while $bitboard != 0 {
            let $square = $bitboard.trailing_zeros() as u8;
            $bitboard &= $bitboard.wrapping_sub(1);
            $func;
        }
    };
}

pub fn flip_bb(bb: u64) -> u64 {
    const K1: u64 = 0x00FF00FF00FF00FF;
    const K2: u64 = 0x0000FFFF0000FFFF;

    let mut bb = bb;

    bb = ((bb >> 8) & K1) | ((bb & K1) << 8);
    bb = ((bb >> 16) & K2) | ((bb & K2) << 16);
    bb = (bb >> 32) | (bb << 32);

    bb
}

c_enum!(Piece {
    FLAG: usize = 2,
    SPY: usize = 3,
    SCOUT: usize = 4,
    MINER: usize = 5,
    GENERAL: usize = 6,
    MARSHAL: usize = 7,
    UNKNOWN: usize = 8,
    BOMB: usize = 9,
});

impl Piece {
    pub const PIECES: [usize; 7] = [
        Piece::FLAG,
        Piece::SPY,
        Piece::SCOUT,
        Piece::MINER,
        Piece::GENERAL,
        Piece::MARSHAL,
        Piece::BOMB,
    ];

    pub fn rank(piece: usize) -> String {
        match piece {
            Piece::SPY => "1",
            Piece::SCOUT => "2",
            Piece::MINER => "3",
            Piece::GENERAL => "9",
            Piece::MARSHAL => "10",
            Piece::BOMB => "b",
            Piece::FLAG => "f",
            _ => {
                println!("piece {}", piece);
                unreachable!()
            }
        }
        .to_string()
    }
}

c_enum!(Flag {
    QUIET: u8 = 1,
    CAPTURE: u8 = 2,
    EVADING: u8 = 4,
});

pub struct Zobrist(());

impl Zobrist {
    const HASHES: [u64; 2 * 8 * 64] = {
        let mut seed: u64 = 1070372;
        let mut hashes = [0u64; 2 * 8 * 64];

        let mut i = 0;
        while i < hashes.len() {
            seed ^= seed >> 12;
            seed ^= seed << 25;
            seed ^= seed >> 27;

            hashes[i] = seed.wrapping_mul(2685821657736338717);
            i += 1;
        }

        hashes
    };

    pub fn get(stm: usize, sq: usize, piece: usize) -> u64 {
        Zobrist::HASHES[stm * 8 * 64 + (piece - 2) * 64 + sq]
    }
}
