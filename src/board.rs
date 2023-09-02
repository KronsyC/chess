use bitboard::Bitboard;
use crate::piece::*;
use crate::movement;


pub struct ChessBoard{
    pub kings : Bitboard,
    pub queens : Bitboard,
    pub knights : Bitboard,
    pub rooks : Bitboard,
    pub bishops : Bitboard,
    pub pawns : Bitboard,

    pub blacks : Bitboard,
    pub whites : Bitboard
}

impl Default for ChessBoard{
    fn default() -> Self {
        ChessBoard{
            whites : Bitboard::ROW_1 | Bitboard::ROW_2,
            blacks : Bitboard::ROW_7 | Bitboard::ROW_8,

            pawns : Bitboard::ROW_2 | Bitboard::ROW_7,
            knights : Bitboard::B1 | Bitboard::G1 | Bitboard::B8 | Bitboard::G8,
            rooks : Bitboard::A1 | Bitboard::A8 | Bitboard::H1 | Bitboard::H8,
            bishops : Bitboard::C1 | Bitboard::C8 | Bitboard::F1 | Bitboard::F8,
            queens : Bitboard::D1 | Bitboard::D8,
            kings : Bitboard::E1 | Bitboard::E8
        }
    }
}

impl ChessBoard{
    pub fn pieces(&self, info : PieceInfo) -> Bitboard{
       let team_bb = match info.team {
           Team::White => self.whites,
           Team::Black => self.blacks,
       };
       let typ_bb = match info.kind{
           PieceKind::King => self.kings,
           PieceKind::Queen => self.queens,
           PieceKind::Rook => self.rooks,
           PieceKind::Pawn => self.pawns,
           PieceKind::Bishop => self.bishops,
           PieceKind::Knight => self.knights
       };
       team_bb & typ_bb
    }

    pub fn team_pieces(&self, team : Team) -> Bitboard{
        match team{
            Team::Black => self.blacks,
            Team::White => self.whites
        }
    }


    pub fn get_piece_pseudo_moves(&self, pos : u8, piece : PieceInfo) -> Bitboard{

        if cfg!(debug_assertions){
            let bb = Bitboard::from_piece_index(pos);
            let matchers = self.pieces(piece.clone());

            let overlap = bb.where_also(matchers);
            
            assert!(!overlap.empty(), "Pseudomoves pos: '{}' does not match the piece info provided: {:?}", pos, piece);
            println!("Get pseudo moves");
        }
        
        match piece.kind {
            PieceKind::Pawn => movement::pawn_moves(pos, piece.team, self.whites | self.blacks),
            PieceKind::King => movement::king_moves(pos),
            PieceKind::Rook => movement::rook_moves(pos, self.whites | self.blacks),
            PieceKind::Queen => movement::queen_moves(pos, self.whites | self.blacks),
            PieceKind::Bishop => movement::bishop_moves(pos, self.whites | self.blacks),
            PieceKind::Knight => movement::knight_moves(pos),
        }
    }
}
