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
