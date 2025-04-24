use super::random;
use crate::{
    bitboard_loop,
    stratego::{Piece, Position},
};

pub fn heuristic() -> String {
    const ATTEMPTS: usize = 50;

    let (deployment, _) = (0..ATTEMPTS)
        .map(|_| {
            let deployment = random();
            let score = evaluate(&deployment);

            (deployment, score)
        })
        .max_by_key(|(_, score)| *score)
        .unwrap();

    deployment
}

pub fn evaluate(deployment: &str) -> isize {
    evaluate_bb(bitboards(&deployment))
}

pub fn evaluate_bb(bb: [u64; 10]) -> isize {
    let occ = bb[0] | Position::LAKES;

    flag_placement(bb[Piece::FLAG])
        + flag_protection(occ, bb[Piece::FLAG], bb[Piece::BOMB], bb[Piece::MARSHAL])
        + general_spy(occ, bb[Piece::GENERAL], bb[Piece::SPY])
        + scout_movement(occ, bb[Piece::SCOUT])
        + miner_placement(bb[Piece::FLAG], bb[Piece::MINER])
        + front_row(bb[Piece::SCOUT], bb[Piece::MINER], bb[Piece::GENERAL])
}

fn flag_placement(bb: u64) -> isize {
    const FLAG: [isize; 24] = [
        10, 5, 8, 5, 5, 8, 5, 10, 1, 1, 5, 1, 1, 5, 1, 1, 0, 0, 10, 0, 0, 10, 0, 0,
    ];

    FLAG[bb.trailing_zeros() as usize]
}

fn flag_protection(occ: u64, flag: u64, bomb: u64, marshal: u64) -> isize {
    let mut score = 0isize;
    let flag_sq = flag.trailing_zeros() as usize;

    let occ = occ | flag << 1 | flag >> 1 | flag >> 8;
    let ranged = ranged(flag_sq, occ) & !occ;
    if ranged.count_ones() < 2 {
        score += 20;
    }

    let chebyshev = chebyshev(flag_sq);
    score += (chebyshev & marshal).count_ones() as isize * 7;

    let orthogonal = orthogonal(flag_sq);
    let count = (orthogonal & bomb).count_ones() as isize;
    score += count * 5;
    if count == 2 {
        score += 3;
    }

    score
}

fn general_spy(occ: u64, general: u64, spy: u64) -> isize {
    let mut score = 0;

    let spy_sq = spy.trailing_zeros() as usize;
    let chebyshev = chebyshev(spy_sq);

    score += (chebyshev & general).count_ones() as isize * 8;

    let occ = occ | spy << 1 | spy >> 1;
    let blocker = ranged(spy_sq, occ) & !occ;
    if blocker.count_ones() <= 2 {
        score += 10;
    }

    score
}

fn scout_movement(occ: u64, scout: u64) -> isize {
    let mut score = 0;

    let mut scout = scout;
    bitboard_loop!(scout, sq, {
        let attacks = ranged(sq as usize, occ) & !occ;

        score += attacks.count_ones() as isize;
    });

    score / 2
}

fn miner_placement(flag: u64, miner: u64) -> isize {
    let chebyshev = chebyshev(flag.trailing_zeros() as usize);
    (chebyshev & miner).count_ones().clamp(0, 1) as isize * 3
}

fn front_row(scout: u64, miner: u64, general: u64) -> isize {
    const FRONT: u64 = 0xdbdbdbdb0000;

    let mut score = 0;
    let (scout, miner, general) = (scout & FRONT, miner & FRONT, general & FRONT);

    let count = scout.count_ones();
    score += match count {
        1 => 5,
        2 => -5,
        _ => 0,
    };

    let count = miner.count_ones();
    score += match count {
        1 => 5,
        2 => -5,
        _ => 0,
    };

    if general.count_ones() == 1 {
        score += 5;
    }

    score
}

fn bitboards(deployment: &str) -> [u64; 10] {
    const SYMBOLS: [char; 8] = [
        Position::SYMBOLS[8],
        Position::SYMBOLS[9],
        Position::SYMBOLS[10],
        Position::SYMBOLS[11],
        Position::SYMBOLS[12],
        Position::SYMBOLS[13],
        Position::SYMBOLS[14],
        Position::SYMBOLS[15],
    ];

    let mut bb = [0u64; 10];

    let (mut file, mut rank) = (0, 0);
    for c in deployment.chars() {
        match c {
            c if c.is_numeric() => file += c as u32 - '0' as u32,
            '/' => (file, rank) = (0, rank + 1),
            _ => {
                let sq = (file + rank * 8) as usize;
                let piece = SYMBOLS.iter().position(|&symbol| symbol == c).unwrap();

                bb[0] |= 1u64 << sq;
                bb[piece + 2] |= 1u64 << sq;

                file += 1;
            }
        }
    }

    bb
}

fn ranged(sq: usize, occ: u64) -> u64 {
    if sq == 64 {
        return 0;
    }

    crate::stratego::ranged(sq, occ)
}

fn chebyshev(sq: usize) -> u64 {
    if sq == 64 {
        return 0;
    }

    crate::stratego::chebyshev(sq)
}

fn orthogonal(sq: usize) -> u64 {
    if sq == 64 {
        return 0;
    }

    crate::stratego::orthogonal(sq)
}
