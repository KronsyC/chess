use std::ops;

///
/// The bitboard
/// LSB is the bottom left,
/// MSB is the top right,
/// Row 1 is the bottom
/// Col 1 is the left
///
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Bitboard {
    pub data: u64,
}

impl Bitboard {
    pub const fn const_default() -> Self {
        Self { data: 0 }
    }
    pub const ROW_1: Self = Self::with_rows(0xFF, 0, 0, 0, 0, 0, 0, 0);
    pub const ROW_2: Self = Self::with_rows(0, 0xFF, 0, 0, 0, 0, 0, 0);
    pub const ROW_3: Self = Self::with_rows(0, 0, 0xFF, 0, 0, 0, 0, 0);
    pub const ROW_4: Self = Self::with_rows(0, 0, 0, 0xFF, 0, 0, 0, 0);
    pub const ROW_5: Self = Self::with_rows(0, 0, 0, 0, 0xFF, 0, 0, 0);
    pub const ROW_6: Self = Self::with_rows(0, 0, 0, 0, 0, 0xFF, 0, 0);
    pub const ROW_7: Self = Self::with_rows(0, 0, 0, 0, 0, 0, 0xFF, 0);
    pub const ROW_8: Self = Self::with_rows(0, 0, 0, 0, 0, 0, 0, 0xFF);

    pub const COL_A: Self = Self::with_cols(0xFF, 0, 0, 0, 0, 0, 0, 0);
    pub const COL_B: Self = Self::with_cols(0, 0xFF, 0, 0, 0, 0, 0, 0);
    pub const COL_C: Self = Self::with_cols(0, 0, 0xFF, 0, 0, 0, 0, 0);
    pub const COL_D: Self = Self::with_cols(0, 0, 0, 0xFF, 0, 0, 0, 0);
    pub const COL_E: Self = Self::with_cols(0, 0, 0, 0, 0xFF, 0, 0, 0);
    pub const COL_F: Self = Self::with_cols(0, 0, 0, 0, 0, 0xFF, 0, 0);
    pub const COL_G: Self = Self::with_cols(0, 0, 0, 0, 0, 0, 0xFF, 0);
    pub const COL_H: Self = Self::with_cols(0, 0, 0, 0, 0, 0, 0, 0xFF);

    pub const A1: Self = Self::ROW_1.where_also(Self::COL_A);
    pub const A2: Self = Self::ROW_2.where_also(Self::COL_A);
    pub const A3: Self = Self::ROW_3.where_also(Self::COL_A);
    pub const A4: Self = Self::ROW_4.where_also(Self::COL_A);
    pub const A5: Self = Self::ROW_5.where_also(Self::COL_A);
    pub const A6: Self = Self::ROW_6.where_also(Self::COL_A);
    pub const A7: Self = Self::ROW_7.where_also(Self::COL_A);
    pub const A8: Self = Self::ROW_8.where_also(Self::COL_A);
    pub const B1: Self = Self::ROW_1.where_also(Self::COL_B);
    pub const B2: Self = Self::ROW_2.where_also(Self::COL_B);
    pub const B3: Self = Self::ROW_3.where_also(Self::COL_B);
    pub const B4: Self = Self::ROW_4.where_also(Self::COL_B);
    pub const B5: Self = Self::ROW_5.where_also(Self::COL_B);
    pub const B6: Self = Self::ROW_6.where_also(Self::COL_B);
    pub const B7: Self = Self::ROW_7.where_also(Self::COL_B);
    pub const B8: Self = Self::ROW_8.where_also(Self::COL_B);
    pub const C1: Self = Self::ROW_1.where_also(Self::COL_C);
    pub const C2: Self = Self::ROW_2.where_also(Self::COL_C);
    pub const C3: Self = Self::ROW_3.where_also(Self::COL_C);
    pub const C4: Self = Self::ROW_4.where_also(Self::COL_C);
    pub const C5: Self = Self::ROW_5.where_also(Self::COL_C);
    pub const C6: Self = Self::ROW_6.where_also(Self::COL_C);
    pub const C7: Self = Self::ROW_7.where_also(Self::COL_C);
    pub const C8: Self = Self::ROW_8.where_also(Self::COL_C);
    pub const D1: Self = Self::ROW_1.where_also(Self::COL_D);
    pub const D2: Self = Self::ROW_2.where_also(Self::COL_D);
    pub const D3: Self = Self::ROW_3.where_also(Self::COL_D);
    pub const D4: Self = Self::ROW_4.where_also(Self::COL_D);
    pub const D5: Self = Self::ROW_5.where_also(Self::COL_D);
    pub const D6: Self = Self::ROW_6.where_also(Self::COL_D);
    pub const D7: Self = Self::ROW_7.where_also(Self::COL_D);
    pub const D8: Self = Self::ROW_8.where_also(Self::COL_D);
    pub const E1: Self = Self::ROW_1.where_also(Self::COL_E);
    pub const E2: Self = Self::ROW_2.where_also(Self::COL_E);
    pub const E3: Self = Self::ROW_3.where_also(Self::COL_E);
    pub const E4: Self = Self::ROW_4.where_also(Self::COL_E);
    pub const E5: Self = Self::ROW_5.where_also(Self::COL_E);
    pub const E6: Self = Self::ROW_6.where_also(Self::COL_E);
    pub const E7: Self = Self::ROW_7.where_also(Self::COL_E);
    pub const E8: Self = Self::ROW_8.where_also(Self::COL_E);
    pub const F1: Self = Self::ROW_1.where_also(Self::COL_F);
    pub const F2: Self = Self::ROW_2.where_also(Self::COL_F);
    pub const F3: Self = Self::ROW_3.where_also(Self::COL_F);
    pub const F4: Self = Self::ROW_4.where_also(Self::COL_F);
    pub const F5: Self = Self::ROW_5.where_also(Self::COL_F);
    pub const F6: Self = Self::ROW_6.where_also(Self::COL_F);
    pub const F7: Self = Self::ROW_7.where_also(Self::COL_F);
    pub const F8: Self = Self::ROW_8.where_also(Self::COL_F);
    pub const G1: Self = Self::ROW_1.where_also(Self::COL_G);
    pub const G2: Self = Self::ROW_2.where_also(Self::COL_G);
    pub const G3: Self = Self::ROW_3.where_also(Self::COL_G);
    pub const G4: Self = Self::ROW_4.where_also(Self::COL_G);
    pub const G5: Self = Self::ROW_5.where_also(Self::COL_G);
    pub const G6: Self = Self::ROW_6.where_also(Self::COL_G);
    pub const G7: Self = Self::ROW_7.where_also(Self::COL_G);
    pub const G8: Self = Self::ROW_8.where_also(Self::COL_G);
    pub const H1: Self = Self::ROW_1.where_also(Self::COL_H);
    pub const H2: Self = Self::ROW_2.where_also(Self::COL_H);
    pub const H3: Self = Self::ROW_3.where_also(Self::COL_H);
    pub const H4: Self = Self::ROW_4.where_also(Self::COL_H);
    pub const H5: Self = Self::ROW_5.where_also(Self::COL_H);
    pub const H6: Self = Self::ROW_6.where_also(Self::COL_H);
    pub const H7: Self = Self::ROW_7.where_also(Self::COL_H);
    pub const H8: Self = Self::ROW_8.where_also(Self::COL_H);

    pub const WHITE_PAWNS_HOME: Self = Self::ROW_2;
    pub const BLACK_PAWNS_HOME: Self = Self::ROW_7;

    pub const fn with_rows(r1: u8, r2: u8, r3: u8, r4: u8, r5: u8, r6: u8, r7: u8, r8: u8) -> Self {
        let dat = [r8, r7, r6, r5, r4, r3, r2, r1];
        Self {
            data: u64::from_be_bytes(dat),
        }
    }
    pub const fn with_cols(c1: u8, c2: u8, c3: u8, c4: u8, c5: u8, c6: u8, c7: u8, c8: u8) -> Self {
        let r1 = c1 >> 0 | c2 << 1 | c3 << 2 | c4 << 3 | c5 << 4 | c6 << 5 | c7 << 6 | c8 << 7;
        let r2 = c1 >> 1 | c2 << 0 | c3 << 1 | c4 << 2 | c5 << 3 | c6 << 4 | c7 << 5 | c8 << 6;
        let r3 = c1 >> 2 | c2 >> 1 | c3 << 0 | c4 << 1 | c5 << 2 | c6 << 3 | c7 << 4 | c8 << 5;
        let r4 = c1 >> 3 | c2 >> 2 | c3 >> 1 | c4 << 0 | c5 << 1 | c6 << 2 | c7 << 3 | c8 << 4;
        let r5 = c1 >> 4 | c2 >> 3 | c3 >> 2 | c4 >> 1 | c5 << 0 | c6 << 1 | c7 << 2 | c8 << 3;
        let r6 = c1 >> 5 | c2 >> 4 | c3 >> 3 | c4 >> 2 | c5 >> 1 | c6 << 0 | c7 << 1 | c8 << 2;
        let r7 = c1 >> 6 | c2 >> 5 | c3 >> 4 | c4 >> 3 | c5 >> 2 | c6 >> 1 | c7 << 0 | c8 << 1;
        let r8 = c1 >> 7 | c2 >> 6 | c3 >> 5 | c4 >> 4 | c5 >> 3 | c6 >> 2 | c7 >> 1 | c8 << 0;
        Self::with_rows(r1, r2, r3, r4, r5, r6, r7, r8)
    }

    ///
    /// Returns a bitboard representing the squares shared
    /// by this board and the other board
    ///
    pub const fn where_also(self, other: Bitboard) -> Bitboard {
        Self {
            data: self.data & other.data,
        }
    }

    ///
    /// Returns a bitboard representing the squares
    /// exclusive to this board
    ///
    #[inline(always)]
    pub const fn where_not(self, other: Bitboard) -> Bitboard {
        Self {
            data: self.data & !other.data,
        }
    }

    ///
    /// Returns a bitboard representing the squares
    /// of this board and the other board
    ///
    #[inline(always)]
    pub const fn combine_with(self, other: Bitboard) -> Bitboard {
        Self {
            data: self.data | other.data,
        }
    }

    ///
    /// Returns a bitboard representing the squares
    /// of this board and the other board
    /// without the shared squares
    ///
    pub const fn combine_with_exclusively(self, other: Bitboard) -> Bitboard {
        Self {
            data: self.data ^ other.data,
        }
    }

    ///
    /// Returns the index of the piece the bitboard represents
    ///
    pub const fn piece_index(self) -> u8 {
        assert!(
            self.data.count_ones() == 1,
            "Expected a singular piece to take the index of"
        );
        self.data.trailing_zeros() as u8
    }

    ///
    /// Create a bitboard from a piece index
    ///
    pub const fn from_piece_index(idx: u8) -> Bitboard {
        assert!(idx < 64, "Expected an in-bounds index");
        Bitboard { data: 1u64 << idx }
    }

    #[inline(always)]
    pub const fn shift_up(self) -> Bitboard {
        Bitboard {
            data: self.data << 8,
        }
    }
    #[inline(always)]
    pub const fn shift_down(self) -> Bitboard {
        Bitboard {
            data: self.data >> 8,
        }
    }

    #[inline(always)]
    pub const fn negative(self) -> Bitboard {
        Bitboard { data: !self.data }
    }

    #[inline(always)]
    pub const fn empty(self) -> bool {
        return self.data == 0;
    }

    #[inline(always)]
    pub const fn count(self) -> u8 {
        return self.data.count_ones() as u8;
    }

    ///
    /// Effectively a series of n shifts up which are or'd together
    /// the initial bit is not preserved
    ///
    #[inline(always)]
    pub const fn ray_up(self, n: u8) -> Bitboard {
        let mut d: u64 = 0;
        let mut i = 0u8;
        loop {
            if i == n {
                break;
            }
            let before = d;
            d |= self.data << ((i + 1) * 8);

            assert!(d != before, "Out of bounds shift");
            i += 1;
        }
        Bitboard { data: d }
    }
    ///
    /// Effectively a series of n shifts down which are or'd together
    /// the initial bit is not preserved
    ///
    #[inline(always)]
    pub const fn ray_down(self, n: u8) -> Bitboard {
        let mut d: u64 = 0;
        let mut i = 0u8;
        loop {
            if i == n {
                break;
            }
            let before = d;
            d |= self.data >> ((i + 1) * 8);
            assert!(d != before, "Out of bounds shift");
            i += 1;
        }
        Bitboard { data: d }
    }
    ///
    /// Effectively a series of n shifts right which are or'd together
    /// the initial bit is not preserved
    ///
    #[inline(always)]
    pub const fn ray_right(self, n: u8) -> Bitboard {
        let mut d: u64 = 0;
        let mut i = 0u8;
        loop {
            if i == n {
                break;
            }
            let before = d;
            d |= self.data >> (i + 1);

            assert!(d != before, "Out of bounds shift");
            i += 1;
        }
        Bitboard { data: d }
    }
    ///
    /// Effectively a series of n shifts left which are or'd together
    /// the initial bit is not preserved
    ///
    #[inline(always)]
    pub const fn ray_left(self, n: u8) -> Bitboard {
        let mut d: u64 = 0;
        let mut i = 0u8;
        loop {
            if i == n {
                break;
            }
            let before = d;
            d |= self.data << (i + 1);

            assert!(d != before, "Out of bounds shift");
            i += 1;
        }
        Bitboard { data: d }
    }

    pub fn bit_indexes(self) -> Vec<u8> {
        let mut dat = self.data;
        let mut idxes = Vec::new();
        loop {
            if dat == 0 {
                break;
            }
            let count = dat.trailing_zeros();
            let mask = 1u64 << count;
            dat ^= mask;
            idxes.push(count as u8);
        }
        idxes
    }

    pub fn mask_permutations(self) -> MaskPermutations {
        MaskPermutations::new(self)
    }

    pub const fn from_bits(bits : u64) -> Bitboard{
        Bitboard{
            data : bits
        }
    }

}

pub struct MaskPermutations {
    mask: Bitboard,
    idx: u64,
    bytes: Vec<bool>,
    bit_indexes: Vec<u8>,
}

impl MaskPermutations {
    fn new(mask: Bitboard) -> MaskPermutations{
        let count = mask.count();
        let mut bytes = Vec::<bool>::with_capacity(count as usize);
        bytes.resize(count as usize, false);

        MaskPermutations{
            mask : mask,
            idx : 0,
            bytes : bytes,
            bit_indexes : mask.bit_indexes()
        }
    }
}

impl Iterator for MaskPermutations {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.mask.count();
        let max_idx = 2u64.pow(count.into()) - 1;
        if self.idx > max_idx {
            return None;
        }
        for i in 0..count {
            let mask = 1 << i;
            let is_set = (self.idx & mask) != 0;
            self.bytes[i as usize] = is_set;
        }

        let mut ret: u64 = 0;
        let mut bytes_idx: u8 = 0;
        for idx in &self.bit_indexes {
            if self.bytes[bytes_idx as usize] {
                ret |= 1 << idx;
            }
            bytes_idx += 1;
        }

        self.idx += 1;
        Some(Bitboard { data: ret })
    }
}

impl ops::BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data | rhs.data,
        }
    }
}
impl ops::BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data & rhs.data,
        }
    }
}

impl ops::Shl<u8> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self {
        Self {
            data: self.data << rhs,
        }
    }
}
impl ops::Shr<u8> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self {
        Self {
            data: self.data >> rhs,
        }
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for row in 0..8 {
            let fixed_row = 7 - row;
            for col in 0..8 {
                let fixed_col = col;
                let idx = 8 * fixed_row + fixed_col;
                let bit = (self.data >> idx) & 1;
                if bit == 1 {
                    s += &ansi_term::Color::Green.paint(" 1").to_string();
                } else {
                    s += &ansi_term::Color::Red.paint(" 0").to_string();
                }
            }

            s += "\n";
        }

        write!(f, "\n{}", s)
    }
}
