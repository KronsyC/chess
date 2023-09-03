use super::util::{Magic, QueenSIMDMagic};
use crate::precalc::masks::BISHOP_MOVEMENT;
use bitboard::Bitboard;
use super::rooks::ROOK_MAGIC_INFO;
use super::bishops::BISHOP_MAGIC_INFO;


pub const QUEEN_MAGIC_INFO : [QueenSIMDMagic;64] = precalc_queen_magics();


use std::simd::u64x2;
use std::simd::usizex2;

const fn precalc_queen_magics() -> [QueenSIMDMagic;64]{

    let mut i = 0;

    let mut ret = [QueenSIMDMagic::const_default();64];

    loop{
        if i == 64{ break; }

        let rook = ROOK_MAGIC_INFO[i];
        let bish = BISHOP_MAGIC_INFO[i];

        let bish_p = bish.moves.as_ptr() as *const u64;
        let rook_p = rook.moves.as_ptr() as *const u64;

        let bish_km = (1u64 << bish.mask.count()) - 1;
        let rook_km = (1u64 << rook.mask.count()) - 1;
        ret[i].moves = std::simd::Simd::from_array([bish_p, rook_p]);
        ret[i].shift = u64x2::from_array([bish.shift, rook.shift]);
        ret[i].multiplier = u64x2::from_array([bish.multiplier, rook.multiplier]);
        ret[i].keymask = u64x2::from_array([bish_km, rook_km]);
        ret[i].mask = u64x2::from_array([bish.mask.data, rook.mask.data]);
        i += 1;
    }
    ret
}
