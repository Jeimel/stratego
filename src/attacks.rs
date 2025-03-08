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

pub fn ranged(sq: usize, occ: u64) -> u64 {
    #[derive(Clone, Copy)]
    struct FileMask {
        bb: u64,
        file: u64,
    }

    const FILE_MASKS: [FileMask; 64] = init_lookup!(|bb| {
        const FILES: [u64; 8] = [
            0x0101010101010101,
            0x0202020202020202,
            0x0404040404040404,
            0x0808080808080808,
            0x1010101010101010,
            0x2020202020202020,
            0x4040404040404040,
            0x8080808080808080,
        ];

        let file = bb.trailing_zeros() as usize & 7;

        FileMask {
            bb,
            file: bb ^ FILES[file],
        }
    });

    const RANK_ATTACKS: [u64; 512] = init_lookup!(|512, index| {
        let index = index as u64;

        let file = index & 7;
        let occ = (index >> 2) | 129;

        let left = (1 << file) - 1;
        let right = u64::MAX << (file + 1);

        let occ_left = left & occ;
        let occ_right = right & occ;

        let attacks_left = if occ_left != 0 {
            let index = 63 - occ_left.leading_zeros();
            (u64::MAX << index) & left
        } else {
            0
        };
        let attacks_right = if occ_right != 0 {
            let index = occ_right.trailing_zeros() + 1;
            ((1 << index) - 1) & right
        } else {
            0
        };

        attacks_left | attacks_right
    });

    let file_mask = FILE_MASKS[sq];

    let mut file = occ & file_mask.file;
    let mut reverse = file.swap_bytes();
    file = file.wrapping_sub(file_mask.bb);
    reverse = reverse.wrapping_sub(file_mask.bb.swap_bytes());
    file ^= reverse.swap_bytes();
    file &= file_mask.file;

    let rook_x8 = sq & 56;
    let rank_occ_x2 = (occ >> rook_x8) as usize & 126;
    let rank = (RANK_ATTACKS[4 * rank_occ_x2 + (sq & 7)]) << rook_x8;

    file | rank
}

pub fn between_squares(from: u8, to: u8) -> u64 {
    let (from, to) = if from < to { (from, to) } else { (to, from) };

    let from_bit = 1u64 << from;
    let to_bit = 1u64 << to;

    let mask = if (from & 7) == (to & 7) {
        let file_mask = 0x0101010101010101 << (from & 7);
        (to_bit - from_bit) & file_mask
    } else {
        let rank_mask = 0xFF << (from & 56);
        (to_bit - from_bit) & rank_mask
    };

    mask ^ to_bit
}
