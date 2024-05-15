//!
//! Precalculate movement masks for use in quick move lookups
//!

use crate::piece::Team;
use positioning::Bitboard;
use positioning::Position;
pub type PrecalcBB = [Bitboard; 64];

pub const KNIGHT_MOVEMENT: PrecalcBB = precalc_knight_moves();
pub const KING_MOVEMENT: PrecalcBB = precalc_king_moves();
pub const W_PAWN_ATTACKS: PrecalcBB = precalc_pawn_attack_mask(Team::White);
pub const B_PAWN_ATTACKS: PrecalcBB = precalc_pawn_attack_mask(Team::Black);
pub const ROOK_MOVEMENT: PrecalcBB = precalc_rook_masks();
pub const BISHOP_MOVEMENT : PrecalcBB = precalc_bishop_masks();
pub const QUEEN_MOVEMENT : PrecalcBB = precalc_queen_masks();


pub mod castling{
    use positioning::Bitboard;
    pub const W_KS_KING_SWITCH : Bitboard = Bitboard::G1.combine_with(Bitboard::E1);
    pub const W_KS_ROOK_SWITCH : Bitboard = Bitboard::H1.combine_with(Bitboard::F1);


    pub const W_QS_KING_SWITCH : Bitboard = Bitboard::C1.combine_with(Bitboard::E1);
    pub const W_QS_ROOK_SWITCH : Bitboard = Bitboard::A1.combine_with(Bitboard::D1);


    pub const B_KS_KING_SWITCH : Bitboard = Bitboard::G8.combine_with(Bitboard::E8);
    pub const B_KS_ROOK_SWITCH : Bitboard = Bitboard::H8.combine_with(Bitboard::F8);


    pub const B_QS_KING_SWITCH : Bitboard = Bitboard::C8.combine_with(Bitboard::E8);
    pub const B_QS_ROOK_SWITCH : Bitboard = Bitboard::A8.combine_with(Bitboard::D8);
}


const fn offset(index: usize, cols: isize, rows: isize) -> Bitboard {
    let col = (index as isize) % 8;
    let row = (index as isize) / 8;

    let new_col = col + cols;
    let new_row = row + rows;

    if new_col < 0 || new_col > 7 || new_row < 0 || new_row > 7 {
        Bitboard { data: 0 }
    } else {
        Bitboard {
            data: 1 << (new_row * 8 + new_col) as u64,
        }
    }
}

///
/// Precalculate an array of attack bitboards 
/// for knights
///
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

    moves
}

///
/// Precalculate an array of attack bitboards for kings
///
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

    moves
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
                board = board.combine_with(offset(i, 1, -1));
                board = board.combine_with(offset(i, -1, -1));
            }
        }

        moves[i] = board;
        i += 1;
    }

    moves
}

const fn precalc_rook_mask(idx: u8) -> Bitboard {
    let pos = Position::from_integral(idx);
    let row = pos.row();
    let col = pos.col();

    let board = Bitboard::from_piece_index(idx);
    let up_count = if row == 7 { 0 } else { 6 - row };
    let down_count = if row == 0 { 0 } else { row - 1 };

    let right_count = if col == 7 { 0 } else { 6 - col  };
    let left_count = if col == 0 { 0 } else { col - 1 };

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

const fn min(a: u8, b: u8) -> u8 {
    if a < b {
        a
    } else {
        b
    }
}

const fn precalc_bishop_mask(idx: u8) -> Bitboard {
    let pos = Position::from_integral(idx);
    let row = pos.row();
    let col = pos.col();

    let board = Bitboard::from_piece_index(idx);

    let up_count = if row == 7 { 0 } else { 6 - row };
    let down_count = if row == 0 { 0 } else { row - 1 };
    let right_count = if col == 7 { 0 } else { 6 - col };
    let left_count = if col == 0 {0} else {col - 1};

    let mut upleft_count = min(up_count, left_count);
    let mut upright_count = min(up_count, right_count);
    let mut downleft_count = min(down_count, left_count);
    let mut downright_count = min(down_count, right_count);

    let mut upleft = board.data;
    let mut upright = board.data;
    let mut downleft = board.data;
    let mut downright = board.data;

    loop{
        if upleft_count == 0{
            break;
        }
        upleft_count-=1;
        upleft |= upleft << 7;
    }
    loop{
        if upright_count == 0{
            break;
        }
        upright_count-=1;
        upright |= upright << 9;
    }
    loop{
        if downleft_count == 0{
            break;
        }
        downleft_count-=1;
        downleft |= downleft >> 9;
    }
    loop{
        if downright_count == 0{
            break;
        }
        downright_count-=1;
        downright |= downright >> 7;
    }

    Bitboard::const_default()
        .combine_with(Bitboard::from_bits(upleft))
        .combine_with(Bitboard::from_bits(upright))
        .combine_with(Bitboard::from_bits(downleft))
        .combine_with(Bitboard::from_bits(downright))
        .where_not(board)
}
const fn precalc_bishop_masks() -> [Bitboard; 64] {
    let mut i = 0u8;

    let mut boards = [Bitboard::const_default(); 64];
    loop {
        if i == 64 {
            break;
        }

        boards[i as usize] = precalc_bishop_mask(i);

        i += 1;
    }

    boards
}

const fn precalc_queen_masks() -> [Bitboard;64]{
    let mut ret = [Bitboard::const_default();64];
    let rooks = precalc_rook_masks();
    let bishops = precalc_bishop_masks();
    let mut i = 0;
    
    loop{
        if i == 64{
            break;
        }
        ret[i] = rooks[i].combine_with(bishops[i]);
        i+=1;
    }
    ret
}
