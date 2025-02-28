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

c_enum!(Piece {
    SPY: usize = 2,
    SCOUT: usize = 3,
    MINER: usize = 4,
    GENERAL: usize = 5,
    MARSHAL: usize = 6,
    BOMB: usize = 7,
    FLAG: usize = 8,
});
