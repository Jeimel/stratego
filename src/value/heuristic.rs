use crate::{
    bitboard_loop,
    deployment::heuristic::evaluate_bb,
    stratego::{chebyshev, flip_bb, Piece, Position, StrategoState},
};

pub fn heuristic(pos: &mut StrategoState, scaling: f32) -> f32 {
    (evaluate(pos) * scaling).tanh()
}

pub fn evaluate(pos: &mut StrategoState) -> f32 {
    const VALUES: [f32; 7] = [
        15.0, // Spy
        5.0,  // Miner
        5.0,  // Scout
        15.0, // General
        30.0, // Marshal
        0.0,  // Unknown
        10.0, // Bomb
    ];

    let board = pos.board();
    let info = pos.information();
    let stm = pos.stm() as usize;

    let mut sum = 0.0;
    for side in [stm, stm ^ 1] {
        let us = board.get(side);
        let them = board.get(side ^ 1);
        let unknown = info.get(side);

        for piece in Piece::SPY..=Piece::BOMB {
            if piece == Piece::UNKNOWN {
                continue;
            }

            let mut mask = board.get(piece) & us;
            let count = mask.count_ones();

            let mut value = VALUES[piece - 3];

            if piece == Piece::MARSHAL && (board.get(Piece::SPY) & them) != 0 {
                value *= 0.5;
            }

            if (piece == Piece::SCOUT || piece == Piece::MINER || piece == Piece::BOMB)
                && count == 1
            {
                value *= 2.0;
            }

            if count > (them & board.get(piece)).count_ones() {
                value *= 1.5;
            }

            if piece == Piece::SPY && (them & board.get(Piece::MARSHAL)) == 0 {
                value /= 5.0;
            }

            bitboard_loop!(
                mask,
                sq,
                sum += if ((1u64 << sq) & unknown) == 0 {
                    value / 2.0
                } else {
                    value
                }
            );

            sum += lower_ranked(&board, side, piece);
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

        sum += evaluate_bb(bb) as f32;

        sum = -sum;
    }

    sum
}

fn lower_ranked(board: &Position, side: usize, piece: usize) -> f32 {
    let mut score = 0.0;

    let mut piece_bb = board.get(side) & board.get(piece);
    bitboard_loop!(piece_bb, sq, {
        let chebyshev = chebyshev(sq as usize);

        for lower in Piece::SPY..piece {
            let lower = board.get(side ^ 1) & board.get(lower);
            score += 5.0 * (lower & chebyshev).count_ones() as f32;
        }
    });

    score
}
