use positioning::Bitboard;

#[derive(Clone, Copy)]
pub struct Magic {
    pub multiplier: u64,
    pub shift: u64,
    pub mask: Bitboard,
    pub moves: &'static [Bitboard],
}

use std::simd;

///
/// Queen magic moves are calculated by aggregating 
/// both rook and bishop moves 
///
/// so we vectorize it, so we can do both simultaneously
///
#[derive(Clone, Copy)]
pub struct QueenSIMDMagic {
    pub multiplier: simd::u64x2,
    pub shift: simd::u64x2,
    pub mask: simd::u64x2,
    pub keymask: simd::u64x2,
    pub moves: simd::Simd<*const u64, 2>,
}

impl QueenSIMDMagic {
    pub const fn const_default() -> Self {
        QueenSIMDMagic {
            multiplier: simd::u64x2::from_array([0, 0]),
            mask: simd::u64x2::from_array([0, 0]),
            shift: simd::u64x2::from_array([0, 0]),
            keymask: simd::u64x2::from_array([0, 0]),
            moves: simd::Simd::from_array([std::ptr::null(), std::ptr::null()]),
        }
    }
}
