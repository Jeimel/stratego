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
    SPY: usize = 2,
    SCOUT: usize = 3,
    MINER: usize = 4,
    GENERAL: usize = 5,
    MARSHAL: usize = 6,
    BOMB: usize = 7,
    FLAG: usize = 8,
});
