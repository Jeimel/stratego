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

c_enum!(Flag {
    QUIET: u8 = 0,
    CAPTURE: u8 = 1,
    EVADING: u8 = 2,
    CHANCE: u8 = 32,
});

pub struct Zobrist(());

impl Zobrist {
    const HASHES: [u64; 2 * 64] = {
        let mut seed: u64 = 1070372;
        let mut hashes = [0u64; 2 * 64];

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

    pub fn get(stm: usize, sq: usize) -> u64 {
        Zobrist::HASHES[(stm + 1) * sq]
    }
}
