use crate::movement;
use crate::piece::*;
use bitboard::Bitboard;

#[derive(Clone, Copy, Debug)]
pub struct ChessBoard {
    pub kings: Bitboard,
    pub queens: Bitboard,
    pub knights: Bitboard,
    pub rooks: Bitboard,
    pub bishops: Bitboard,
    pub pawns: Bitboard,

    pub blacks: Bitboard,
    pub whites: Bitboard,
}

impl Default for ChessBoard {
    fn default() -> Self {
        ChessBoard {
            whites: Bitboard::ROW_1 | Bitboard::ROW_2,
            blacks: Bitboard::ROW_7 | Bitboard::ROW_8,

            pawns: Bitboard::ROW_2 | Bitboard::ROW_7,
            knights: Bitboard::B1 | Bitboard::G1 | Bitboard::B8 | Bitboard::G8,
            rooks: Bitboard::A1 | Bitboard::A8 | Bitboard::H1 | Bitboard::H8,
            bishops: Bitboard::C1 | Bitboard::C8 | Bitboard::F1 | Bitboard::F8,
            queens: Bitboard::D1 | Bitboard::D8,
            kings: Bitboard::E1 | Bitboard::E8,
        }
    }
}

impl ChessBoard {
    pub fn pieces(&self, info: PieceInfo) -> Bitboard {
        let team_bb = match info.team {
            Team::White => self.whites,
            Team::Black => self.blacks,
        };
        let typ_bb = match info.kind {
            PieceKind::King => self.kings,
            PieceKind::Queen => self.queens,
            PieceKind::Rook => self.rooks,
            PieceKind::Pawn => self.pawns,
            PieceKind::Bishop => self.bishops,
            PieceKind::Knight => self.knights,
        };
        team_bb & typ_bb
    }

    pub fn team_pieces(&self, team: Team) -> Bitboard {
        match team {
            Team::Black => self.blacks,
            Team::White => self.whites,
        }
    }

    pub fn get_piece_legal_moves(&self, pos: u8, piece: PieceInfo) -> Bitboard {
        let pseudo_moves = self.get_piece_pseudo_moves(pos, piece);
        let pos_mask = Bitboard::from_piece_index(pos);

        let mut ret = Bitboard::default();
        for m in pseudo_moves.bit_masks() {

            let mut state = self.clone();
            // Step 1: Construct the post-move state
            state.kill_piece(m);
            let switch = m | pos_mask;
            *state.get_mut_team_bb(piece.team) ^= switch;
            *state.get_mut_piece_bb(piece.kind) ^= switch;

            // Step 2: Check if the state is valid postmove state
            //         for the team
            if state.is_valid_postmove(piece.team) {
                ret |= m;
            }
        }
        ret
    }

    pub fn get_piece_info(&self, piece : u8) -> PieceInfo{
        let b= Bitboard::from_piece_index(piece);
        let f = |d : Bitboard| !(b & d).empty();

        let team = if f(self.whites){ Team::White } else {Team::Black};

        let kind = if f(self.bishops) {PieceKind::Bishop} 
                   else if f(self.rooks) {PieceKind::Rook}
                   else if f(self.pawns) {PieceKind::Pawn}
                   else if f(self.kings) {PieceKind::King}
                   else if f(self.knights) {PieceKind::Knight}
                   else {PieceKind::Queen};

        PieceInfo{
            team : team,
            kind : kind
        }
    }
    pub fn get_piece_pseudo_moves(&self, pos: u8, piece: PieceInfo) -> Bitboard {
        if cfg!(debug_assertions) {
            let bb = Bitboard::from_piece_index(pos);
            let matchers = self.pieces(piece.clone());

            let overlap = bb.where_also(matchers);

            assert!(
                !overlap.empty(),
                "Pseudomoves pos: '{}' does not match the piece info provided: {:?}",
                pos,
                piece
            );
        }

        let friends = self.team_pieces(piece.team);

        match piece.kind {
            PieceKind::Pawn => movement::pawn_moves(pos, piece.team, self.whites | self.blacks),
            PieceKind::King => movement::king_moves(pos),
            PieceKind::Rook => movement::rook_moves(pos, self.whites | self.blacks),
            PieceKind::Queen => movement::queen_moves(pos, self.whites | self.blacks),
            PieceKind::Bishop => movement::bishop_moves(pos, self.whites | self.blacks),
            PieceKind::Knight => movement::knight_moves(pos),
        }
        .where_not(friends)
    }



    fn get_mut_team_bb(&mut self, team : Team) -> &mut Bitboard{
        match team{
            Team::White => &mut self.whites,
            Team::Black => &mut self.blacks
        }
    }

    fn get_mut_piece_bb(&mut self, piece : PieceKind) -> &mut Bitboard{
        match piece{
            PieceKind::King => &mut self.kings,
            PieceKind::Rook => &mut self.rooks,
            PieceKind::Pawn => &mut self.pawns,
            PieceKind::Queen => &mut self.queens,
            PieceKind::Bishop => &mut self.bishops,
            PieceKind::Knight => &mut self.knights
        }
    }

    ///
    /// Removes a piece from the bitboard 
    /// if the piece doesn't exist, this acts as a no-op
    ///
    fn kill_piece(&mut self, pos : Bitboard){
        // Hopefully this code gets vectorized
        // 8 * u64 => 512 bit avx register
        let neg = pos.negative();
        self.kings &= neg;
        self.queens &= neg;
        self.rooks &= neg;
        self.knights &= neg;
        self.bishops &= neg;
        self.pawns &= neg;
        self.whites &= neg;
        self.blacks &= neg;
    }

    pub fn is_pos_attacked(&self, pos : u8, attacked_by : Team) -> bool{
        let world = self.whites | self.blacks;
        let enemies = self.team_pieces(attacked_by);

        let moves_as_rook = movement::rook_moves(pos, world);
        let moves_as_bish = movement::bishop_moves(pos, world);
        let moves_as_knight = movement::knight_moves(pos);
        let moves_as_king = movement::king_moves(pos);
        let moves_from_pawn = movement::pawn_attackers(pos, attacked_by.enemy());
      
        // we have rooklikes and bishlikes, as queens act as a rook and bishop 
        // this saves a movement::queen_moves call
        let rooklikes = (self.rooks | self.queens) & enemies;
        let bishlikes = (self.bishops | self.queens) & enemies;
        let knights = self.knights & enemies;
        let kings = self.kings & enemies;
        let pawns = self.pawns & enemies;


        let killer_rooks = moves_as_rook & rooklikes;
        let killer_bishes = moves_as_bish & bishlikes;
        let killer_knights = moves_as_knight & knights;
        let killer_kings = moves_as_king & kings;
        let killer_pawns = pawns & moves_from_pawn;


        !(killer_knights | killer_bishes | killer_rooks | killer_kings | killer_pawns).empty()
    } 
    #[inline(never)]
    fn is_valid_postmove(&self, team : Team) -> bool{

        let team_king = self.pieces(PieceInfo{kind : PieceKind::King, team : team});

        let king_idx = team_king.piece_index();

        return !self.is_pos_attacked(king_idx, team.enemy());
    }
}
