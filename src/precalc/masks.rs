use bitboard::Bitboard;
use crate::coord::Coord;
use crate::piece::Team;

pub type PrecalcBB = [Bitboard; 64];

pub const KNIGHT_MOVEMENT: PrecalcBB = precalc_knight_moves();
pub const KING_MOVEMENT: PrecalcBB = precalc_king_moves();
pub const W_PAWN_ATTACKS: PrecalcBB = precalc_pawn_attack_mask(Team::White);
pub const B_PAWN_ATTACKS: PrecalcBB = precalc_pawn_attack_mask(Team::Black);
pub const ROOK_MOVEMENT: PrecalcBB = precalc_rook_masks();

const fn offset(index: usize, cols: isize, rows: isize) -> Bitboard {
    let col = (index as isize) % 8;
    let row = (index as isize) / 8;

    let new_col = col + cols;
    let new_row = row + rows;

    if new_col < 0 || new_col > 7 || new_row < 0 || new_row > 7 {
        return Bitboard { data: 0 };
    } else {
        return Bitboard {
            data: 1 << (new_row * 8 + new_col) as u64,
        };
    }
}

const fn precalc_knight_moves() -> [Bitboard; 64] {
    let mut moves = [Bitboard::const_default(); 64];

    let mut i: usize = 0;
    loop {
        if i == 64 {
            break;
        }
        let mut board = Bitboard::const_default();
        board = board.combine_with(offset(i, -2, 1));
        board = board.combine_with(offset(i, -2, -1));
        board = board.combine_with(offset(i, 2, 1));
        board = board.combine_with(offset(i, 2, -1));

        board = board.combine_with(offset(i, 1, -2));
        board = board.combine_with(offset(i, -1, -2));
        board = board.combine_with(offset(i, 1, 2));
        board = board.combine_with(offset(i, -1, 2));

        moves[i] = board;
        i += 1;
    }

    return moves;
}

const fn precalc_king_moves() -> [Bitboard; 64] {
    let mut moves = [Bitboard::const_default(); 64];

    let mut i: usize = 0;
    loop {
        if i == 64 {
            break;
        }
        let mut board = Bitboard::const_default();
        board = board.combine_with(offset(i, -1, 1));
        board = board.combine_with(offset(i, -1, -1));
        board = board.combine_with(offset(i, 1, 1));
        board = board.combine_with(offset(i, 1, -1));

        board = board.combine_with(offset(i, 1, 0));
        board = board.combine_with(offset(i, -1, 0));
        board = board.combine_with(offset(i, 0, 1));
        board = board.combine_with(offset(i, 0, -1));

        moves[i] = board;
        i += 1;
    }

    return moves;
}

const fn precalc_pawn_attack_mask(team: Team) -> [Bitboard; 64] {
    let mut moves = [Bitboard::const_default(); 64];

    let mut i: usize = 0;
    loop {
        if i == 64 {
            break;
        }
        let mut board = Bitboard::const_default();

        match team {
            Team::White => {
                board = board.combine_with(offset(i, 1, 1));
                board = board.combine_with(offset(i, -1, 1));
            }
            Team::Black => {
                board = board.combine_with(offset(i, -1, -1));
                board = board.combine_with(offset(i, 1, -1));
            }
        }

        moves[i] = board;
        i += 1;
    }

    return moves;
}

const fn precalc_rook_mask(idx: u8) -> Bitboard {
    let Coord { row, col } = Coord::from_idx(idx);

    let board = Bitboard::from_piece_index(idx);
    let up_count = if row == 7 { 0 } else { 6 - row };
    let down_count = if row == 0 { 0 } else { row - 1 };

    let right_count = if col == 0 { 0 } else { col - 1 };
    let left_count = if col == 7 { 0 } else { 6 - col };

    let up = board.ray_up(up_count);
    let down = board.ray_down(down_count);
    let left = board.ray_left(left_count);
    let right = board.ray_right(right_count);

    Bitboard::const_default()
        .combine_with(up)
        .combine_with(down)
        .combine_with(left)
        .combine_with(right)
}
const fn precalc_rook_masks() -> [Bitboard; 64] {
    let mut i = 0u8;

    let mut boards = [Bitboard::const_default(); 64];
    loop {
        if i == 64 {
            break;
        }

        boards[i as usize] = precalc_rook_mask(i);

        i += 1;
    }

    boards
}

