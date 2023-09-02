use bitboard::Bitboard;

pub struct Magic{
    pub multiplier : u64,
    pub shift : u64,
    pub mask : Bitboard,
    pub moves : &'static[Bitboard]
}