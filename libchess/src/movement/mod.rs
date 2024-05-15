use crate::magic_bitboard::{bishops::BISHOP_MAGIC_INFO, rooks::ROOK_MAGIC_INFO};
use crate::piece::Team;
use crate::piece::TTeam;
use crate::precalc::masks::{B_PAWN_ATTACKS, KING_MOVEMENT, KNIGHT_MOVEMENT, W_PAWN_ATTACKS};


use positioning::Bitboard;
use positioning::Position;

pub fn knight_moves(pos: Position) -> Bitboard {
    unsafe { *KNIGHT_MOVEMENT.get_unchecked(pos.integral() as usize) }
}

pub fn king_moves(pos: Position) -> Bitboard {
    unsafe { *KING_MOVEMENT.get_unchecked(pos.integral() as usize) }
}

pub fn bishop_moves(pos: Position, world: Bitboard) -> Bitboard {
    let info = unsafe { BISHOP_MAGIC_INFO.get_unchecked(pos.integral() as usize) };
    let mut raw = world.data;
    raw &= info.mask.data;
    (raw, _) = raw.overflowing_mul(info.multiplier);
    raw >>= info.shift;
    unsafe { *info.moves.get_unchecked(raw as usize) }
}

pub fn rook_moves(pos: Position, world: Bitboard) -> Bitboard {
    let info = unsafe { ROOK_MAGIC_INFO.get_unchecked(pos.integral() as usize) };
    let mut raw = world.data;
    raw &= info.mask.data;
    (raw, _) = raw.overflowing_mul(info.multiplier);
    raw >>= info.shift;
    unsafe { *info.moves.get_unchecked(raw as usize) }
}



pub fn queen_moves(pos: Position, world: Bitboard) -> Bitboard {

    let (bish_info, rook_info) = unsafe {
        (
            *BISHOP_MAGIC_INFO.get_unchecked(pos.integral() as usize),
            *ROOK_MAGIC_INFO.get_unchecked(pos.integral() as usize),
        )
    };

    let maskb = world.data & bish_info.mask.data;
    let maskr = world.data & rook_info.mask.data;


    let (mulb, _) = maskb.overflowing_mul(bish_info.multiplier);
    let (mulr, _) = maskr.overflowing_mul(rook_info.multiplier);

    let shb = mulb >> bish_info.shift;
    let shr = mulr >> rook_info.shift;

    let atkb = unsafe{bish_info.moves.get_unchecked(shb as usize)};
    let atkr = unsafe{rook_info.moves.get_unchecked(shr as usize)};

    return atkb.combine_with(*atkr);


    // let info = unsafe{QUEEN_MAGIC_INFO.get_unchecked(pos as usize)};
    // let worlds = simd::u64x2::from_array([world.data, world.data]);
    // let masked = info.mask.data & worlds;
    // let mulled = masked * info.multiplier;
    // let shifted = mulled >> info.shift;
    // let keymasked = shifted & info.keymask;
    // let pointers = info.moves.wrapping_add(keymasked.cast());
    //
    // let [p1, p2] = pointers.to_array();
    //
    // unsafe{
    //     let bishop_aspect = *p1;
    //     let rook_aspect = *p2;
    //     return Bitboard::from_bits(bishop_aspect | rook_aspect);
    // }
    // let bish_moves = unsafe{info.bish_moves.get_unchecked(bish_idx as usize)};
    // let rook_moves = unsafe{info.rook_moves.get_unchecked(rook_idx as usize)};
    // return bish_moves.combine_with(*rook_moves);
}

pub fn pawn_moves<T : TTeam>(pos: Position, world: Bitboard) -> Bitboard {
    let free_spots = world.negative();
    let piece = Bitboard::from(pos);
    match T::TEAM {
        Team::White => unsafe {
            let mut moves = piece
                .shift_up()
                .where_also(free_spots);


            if !(piece & Bitboard::WHITE_PAWNS_HOME).empty() {
                moves = moves.combine_with(moves.shift_up().where_also(free_spots));
            }
            moves =
                moves.combine_with((*W_PAWN_ATTACKS.get_unchecked(pos.integral() as usize)).where_also(world));
            return moves;
        },
        Team::Black => unsafe {
            let mut moves = piece
                .shift_down()
                .where_also(free_spots);
            if !(piece & Bitboard::BLACK_PAWNS_HOME).empty() {
                moves = moves.combine_with(moves.shift_down().where_also(free_spots));
            }
            moves =
                moves.combine_with((*B_PAWN_ATTACKS.get_unchecked(pos.integral() as usize)).where_also(world));
            return moves;
        },
    }
}

///
/// Yields the positions from which pawns may attack you 
/// the team argument is the team of your piece
///
pub fn pawn_attackers<T : TTeam>(pos : Position) -> Bitboard{
    match T::TEAM{
        Team::White => unsafe{ *W_PAWN_ATTACKS.get_unchecked(pos.integral() as usize) },
        Team::Black => unsafe{ *B_PAWN_ATTACKS.get_unchecked(pos.integral() as usize) }
    }
}
