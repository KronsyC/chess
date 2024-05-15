use crate::piece::{Team, PieceKind};
use positioning::Position;

pub struct ZobKeys{
    pub white_pawn : ZKeySet,
    pub black_pawn : ZKeySet,

    pub white_bishop : ZKeySet,
    pub black_bishop : ZKeySet,
    
    pub white_rook : ZKeySet,
    pub black_rook : ZKeySet,

    pub white_knight : ZKeySet,
    pub black_knight : ZKeySet,

    pub white_king : ZKeySet,
    pub black_king : ZKeySet,

    pub white_queen : ZKeySet,
    pub black_queen : ZKeySet,

    pub black_to_move : ZKey,

    pub castle_white_ks : ZKey,
    pub castle_white_qs : ZKey,
    pub castle_black_ks : ZKey,
    pub castle_black_qs : ZKey,

    pub enpassant_a : ZKey,
    pub enpassant_b : ZKey,
    pub enpassant_c : ZKey,
    pub enpassant_d : ZKey,
    pub enpassant_e : ZKey,
    pub enpassant_f : ZKey,
    pub enpassant_g : ZKey,
    pub enpassant_h : ZKey,
}

type HashT = u64;

pub struct ZKeySet([ZKey; 64]);

#[derive(Clone, Copy, Debug, Default)]
pub struct ZKey(HashT);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZobristHash(HashT);

impl ZobristHash{
    pub fn update(&mut self, key : ZKey){
        // println!("{}", key.0);
        self.0 ^= key.0;
    }
}




impl ZKey{
    pub fn generate(rng : &mut impl rand::Rng) -> Self{
        Self(rng.gen::<HashT>())
        // ZKey(rng.gen::<HashT>() & rng.gen::<HashT>() & rng.gen::<HashT>())
    }
}

impl ZKeySet{
    pub fn generate(rng : &mut impl rand::Rng) -> Self{
        Self(std::array::from_fn(|_| ZKey::generate(rng)))
    }

    pub fn at_pos(&self, pos : Position) -> ZKey{
        *self.0.get(pos.integral() as usize).unwrap()
    }
}


impl ZobKeys{
    pub fn generate(rng : &mut impl rand::Rng) -> Self{
        Self{
            black_knight: ZKeySet::generate(rng),
            black_rook: ZKeySet::generate(rng),
            black_queen: ZKeySet::generate(rng),
            black_king: ZKeySet::generate(rng),
            black_bishop: ZKeySet::generate(rng),
            black_pawn: ZKeySet::generate(rng),
            white_knight: ZKeySet::generate(rng),
            white_rook: ZKeySet::generate(rng),
            white_queen: ZKeySet::generate(rng),
            white_king: ZKeySet::generate(rng),
            white_bishop: ZKeySet::generate(rng),
            white_pawn: ZKeySet::generate(rng),
            black_to_move : ZKey::generate(rng),
            enpassant_a: ZKey::generate(rng),
            enpassant_b: ZKey::generate(rng),
            enpassant_c: ZKey::generate(rng),
            enpassant_d: ZKey::generate(rng),
            enpassant_e: ZKey::generate(rng),
            enpassant_f: ZKey::generate(rng),
            enpassant_g: ZKey::generate(rng),
            enpassant_h: ZKey::generate(rng),
            castle_black_ks : ZKey::generate(rng),
            castle_black_qs : ZKey::generate(rng),
            castle_white_ks : ZKey::generate(rng),
            castle_white_qs : ZKey::generate(rng),
        } 
    }



    pub const fn piece_keys_for(&self, piece : PieceKind, team : Team) -> &ZKeySet{
        use Team::*;
        use PieceKind::*;
        match (team, piece){
            (White, Pawn) => &self.white_pawn,
            (White, King) => &self.white_king,
            (White, Knight) => &self.white_knight,
            (White, Rook) => &self.white_rook,
            (White, Bishop) => &self.white_bishop,
            (White, Queen) => &self.white_queen,
            (Black, Pawn) => &self.black_pawn,
            (Black, King) => &self.black_king,
            (Black, Knight) => &self.black_knight,
            (Black, Rook) => &self.black_rook,
            (Black, Bishop) => &self.black_bishop,
            (Black, Queen) => &self.black_queen,
        }
    }

    pub const fn enpassant_keys_for(&self, pos : Position) -> ZKey{
        match pos.col(){
            0 => self.enpassant_a,
            1 => self.enpassant_b,
            2 => self.enpassant_c,
            3 => self.enpassant_d,
            4 => self.enpassant_e,
            5 => self.enpassant_f,
            6 => self.enpassant_g,
            7 => self.enpassant_h,
            _ => ZKey(0)
        }
    }
}





