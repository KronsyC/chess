use crate::magic_bitboard::{bishops::BISHOP_MAGIC_INFO, rooks::ROOK_MAGIC_INFO};
use crate::piece::Team;
use crate::precalc::masks::{B_PAWN_ATTACKS, KING_MOVEMENT, KNIGHT_MOVEMENT, W_PAWN_ATTACKS};

use bitboard::Bitboard;

pub fn knight_moves(pos: u8) -> Bitboard {
    unsafe { *KNIGHT_MOVEMENT.get_unchecked(pos as usize) }
}

pub fn king_moves(pos: u8) -> Bitboard {
    unsafe { *KING_MOVEMENT.get_unchecked(pos as usize) }
}

pub fn bishop_moves(pos: u8, world: Bitboard) -> Bitboard {
    let info = unsafe { BISHOP_MAGIC_INFO.get_unchecked(pos as usize) };
    let mut raw = world.data;
    raw &= info.mask.data;
    (raw, _) = raw.overflowing_mul(info.multiplier);
    raw >>= info.shift;
    unsafe { *info.moves.get_unchecked(raw as usize) }
}

pub fn rook_moves(pos: u8, world: Bitboard) -> Bitboard {
    let info = unsafe { ROOK_MAGIC_INFO.get_unchecked(pos as usize) };
    let mut raw = world.data;
    raw &= info.mask.data;
    (raw, _) = raw.overflowing_mul(info.multiplier);
    raw >>= info.shift;
    unsafe { *info.moves.get_unchecked(raw as usize) }
}

#[inline(never)]
pub fn queen_moves(pos: u8, world: Bitboard) -> Bitboard {
    // TODO: Try to get this working to be faster than the current linear approach,
    // it may not be faster, but try
    // use packed_simd::u64x2;
    //
    // let (bish_info, rook_info) = unsafe {
    //     (
    //         *BISHOP_MAGIC_INFO.get_unchecked(pos as usize),
    //         *ROOK_MAGIC_INFO.get_unchecked(pos as usize),
    //     )
    // };
    //
    // let shifts = u64x2::from([bish_info.shift, rook_info.shift]);
    // let masks = u64x2::from([bish_info.mask.data, rook_info.mask.data]);
    // let mulls = u64x2::from([bish_info.multiplier, rook_info.multiplier]);
    //
    // let masked = masks * world.data;
    // let mulled = masked * mulls;
    // let shifted = mulled >> shifts;
    //
    // let mut a = [0u64; 2];
    // {
    //     let mut outs = a.as_mut_slice();
    //     shifted.write_to_slice_unaligned(&mut outs);
    // }
    // let r = bish_info.moves[a[0] as usize].combine_with(rook_info.moves[a[1] as usize]);
    //
    // println!("Outputs: {}", r);
    // world
    // use simd::prelude::*;
    //
    // let raw_world = world.data;
    //
    // use std::simd;
    // let worlds = simd::u64x2::from_array([raw_world, raw_world]);
    // let masks = simd::u64x2::from_array([rook_info.mask.data, bish_info.mask.data]);
    // let mults = simd::u64x2::from_array([rook_info.multiplier, bish_info.multiplier]);
    // let shifts = simd::u64x2::from_array([rook_info.shift, bish_info.shift]);
    //
    // let masked_worlds = masks & worlds;
    // let mulled_worlds = masked_worlds * mults;
    // let shifted_worlds = mulled_worlds >> shifts;
    // let [i1, i2] = shifted_worlds.as_array();
    //
    //
    // unsafe{
    // let b1 = *rook_info.moves.get_unchecked(*i1 as usize);
    // let b2 = *bish_info.moves.get_unchecked(*i2 as usize);
    // return b1.combine_with(b2);
    // }

    let rook_aspect = rook_moves(pos, world);
    let bishop_aspect = bishop_moves(pos, world);

    rook_aspect.combine_with(bishop_aspect)

    //a550:       40 0f b6 c7             movzbl %dil,%eax
    //a554:       48 c1 e0 03             shl    $0x3,%rax
    //a558:       48 8d 04 80             lea    (%rax,%rax,4),%rax
    //a55c:       48 8d 15 b5 cb 11 00    lea    0x11cbb5(%rip),%rdx        # 127118 <_ZN9ansi_term4ansi5RESET17h75976bfb87b4cb84E+0xc28>
    //a563:       48 8b 7c 02 20          mov    0x20(%rdx,%rax,1),%rdi
    //a568:       48 21 f7                and    %rsi,%rdi
    //a56b:       48 0f af 7c 02 10       imul   0x10(%rdx,%rax,1),%rdi
    //a571:       0f b6 4c 02 18          movzbl 0x18(%rdx,%rax,1),%ecx
    //a576:       48 d3 ef                shr    %cl,%rdi
    //a579:       48 8b 14 02             mov    (%rdx,%rax,1),%rdx
    //a57d:       4c 8d 05 94 c1 11 00    lea    0x11c194(%rip),%r8        # 126718 <_ZN9ansi_term4ansi5RESET17h75976bfb87b4cb84E+0x228>
    //a584:       49 23 74 00 20          and    0x20(%r8,%rax,1),%rsi
    //a589:       49 0f af 74 00 10       imul   0x10(%r8,%rax,1),%rsi
    //a58f:       41 0f b6 4c 00 18       movzbl 0x18(%r8,%rax,1),%ecx
    //a595:       48 d3 ee                shr    %cl,%rsi
    //a598:       49 8b 04 00             mov    (%r8,%rax,1),%rax
    //a59c:       48 8b 04 f0             mov    (%rax,%rsi,8),%rax
    //a5a0:       48 0b 04 fa             or     (%rdx,%rdi,8),%rax
    //a5a4:       c3                      ret
}

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
