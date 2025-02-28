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
