use crate::{
    bitboard_loop,
    deployment::heuristic::evaluate_bb,
    stratego::{chebyshev, flip_bb, orthogonal, Piece, Position, StrategoState},
};

const VALUES: [f32; 8] = [
    10000.0, // Flag
    200.0,   // Spy
    25.0,    // Miner
    30.0,    // Scout
    200.0,   // General
    400.0,   // Marshal
    0.0,     // Unknown
    20.0,    // Bomb
];

pub fn heuristic(pos: &mut StrategoState, scaling: f32) -> f32 {
    (evaluate(pos) / scaling).tanh()
}

pub fn evaluate(pos: &mut StrategoState) -> f32 {
    let board = pos.board();
    let info = pos.information();
    let stm = pos.stm() as usize;

    let mut sum = 0.0;
    for side in [stm, stm ^ 1] {
        let us = board.get(side);
        let them = board.get(side ^ 1);
        let unknown = info.get(side);

        let flag = (board.get(Piece::FLAG) & them).trailing_zeros() as usize;
        if flag == 0 {
            sum -= VALUES[0];

            continue;
        }

        let flag_chebyshev = chebyshev(flag);

        let mut max = 0.0;
        for piece in Piece::SPY..=Piece::BOMB {
            if piece == Piece::UNKNOWN {
                continue;
            }

            let mut mask = board.get(piece) & us;
            let count = mask.count_ones();

            let mut value = VALUES[piece - 2];

            if piece == Piece::BOMB {
                value = max * 0.5;
            }

            if piece == Piece::MARSHAL && (board.get(Piece::SPY) & them) != 0 {
                value *= 0.5;
            }

            if (piece == Piece::SCOUT || piece == Piece::MINER || piece == Piece::BOMB)
                && count == 1
            {
                value *= 1.5;
            }

            if count > (them & board.get(piece)).count_ones() {
                value *= 1.5;
            }

            if piece == Piece::SPY && (them & board.get(Piece::MARSHAL)) == 0 {
                value /= 5.0;
            }

            if value > max {
                max = value;
            }

            bitboard_loop!(mask, sq, {
                let mut value = value;

                if (flag_chebyshev & (1u64 << sq)) != 0 {
                    value *= 5.0;
                }

                sum += if ((1u64 << sq) & unknown) == 0 {
                    value / 2.0
                } else {
                    value
                }
            });

            sum += lower_ranked(&board, side, piece, side == stm);
        }

        let mut bb = [
            us,
            0,
            board.get(Piece::FLAG) & us,
            board.get(Piece::SPY) & us,
            board.get(Piece::SCOUT) & us,
            board.get(Piece::MINER) & us,
            board.get(Piece::GENERAL) & us,
            board.get(Piece::MARSHAL) & us,
            0,
            board.get(Piece::BOMB) & us,
        ];

        if side == 1 {
            for i in 0..10 {
                bb[i] = flip_bb(bb[i]);
            }
        }

        sum += 20.0 * evaluate_bb(bb) as f32;

        sum = -sum;
    }

    sum
}

fn lower_ranked(board: &Position, side: usize, piece: usize, bonus: bool) -> f32 {
    let mut score = 0.0;
    let mut piece_bb = board.get(side) & board.get(piece);

    bitboard_loop!(piece_bb, sq, {
        let orthogonal = orthogonal(sq as usize);

        let more = match piece {
            Piece::SPY => vec![Piece::MARSHAL],
            Piece::MINER => vec![Piece::BOMB],
            _ => vec![],
        };

        for lower in (Piece::SPY..piece).chain(more) {
            let lower_bb = board.get(side ^ 1) & board.get(lower);
            score += (lower_bb & orthogonal).count_ones() as f32
                * if bonus { VALUES[lower - 2] / 2.0 } else { 5.0 };
        }
    });

    score
}
