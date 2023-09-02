use bitboard::Bitboard;
use crate::piece::Team;
use crate::precalc::masks::{
    B_PAWN_ATTACKS, KING_MOVEMENT, KNIGHT_MOVEMENT, ROOK_MOVEMENT, W_PAWN_ATTACKS,
};
use crate::magic_bitboard::rooks::ROOK_MAGIC_INFO;

pub fn knight_moves(pos: u8) -> Bitboard {
    unsafe { *KNIGHT_MOVEMENT.get_unchecked(pos as usize) }
}

pub fn king_moves(pos: u8) -> Bitboard {
    unsafe { *KING_MOVEMENT.get_unchecked(pos as usize) }
}

pub fn bishop_moves(pos: u8, world: Bitboard) -> Bitboard {
    todo!()
}

#[inline(never)]
pub fn rook_moves(pos: u8, world: Bitboard) -> Bitboard {
    let info = &ROOK_MAGIC_INFO[pos as usize];
    let mut raw = world.data;
    raw = raw & info.mask.data;
    (raw, _) = raw.overflowing_mul(info.multiplier);
    raw = raw >> info.shift;
    unsafe{
        *info.moves.get_unchecked(raw as usize)
    }
}

pub fn queen_moves(pos: u8, world: Bitboard) -> Bitboard {
    todo!()
}

#[inline(never)]
pub fn pawn_moves(pos: u8, team: Team, world: Bitboard) -> Bitboard {
    let free_spots = world.negative();
    match team {
        Team::White => unsafe {
            let mut moves = Bitboard::from_piece_index(pos)
                .shift_up()
                .where_also(free_spots);
            moves = moves.combine_with(moves.shift_up().where_also(free_spots));
            moves =
                moves.combine_with((*W_PAWN_ATTACKS.get_unchecked(pos as usize)).where_also(world));
            return moves;
        },
        Team::Black => unsafe {
            let mut moves = Bitboard::from_piece_index(pos)
                .shift_down()
                .where_also(free_spots);
            moves = moves.combine_with(moves.shift_down().where_also(free_spots));
            moves =
                moves.combine_with((*B_PAWN_ATTACKS.get_unchecked(pos as usize)).where_also(world));
            return moves;
        },
    }
}
