macro_rules! init_lookup {
    (| $bb:ident | $($intern:expr)+) => {
        init_lookup!(|64, index| {
            let $bb = 1u64 << index;

            $($intern)+
        })
    };

    (| $size:expr, $index:ident | $($intern:expr)+) => {{
        let mut $index = 0usize;
        let mut attacks = [$($intern)+; $size];

        while $index != $size - 1 {
            $index += 1;

            attacks[$index] = $($intern)+;
        }

        attacks
    }};
}

pub fn orthogonal(sq: usize) -> u64 {
    const ATTACKS: [u64; 64] = init_lookup!(|bb| {
        ((bb << 1) & 0xfefefefefefefefe) | ((bb >> 1) & 0x7f7f7f7f7f7f7f7f) | (bb << 8) | (bb >> 8)
    });

    ATTACKS[sq]
}
