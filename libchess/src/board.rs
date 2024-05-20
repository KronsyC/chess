use crate::movement;
use crate::piece::*;
use positioning::Bitboard;
use positioning::Position;

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
        // Initialize a chess board with the standard layout
        Self {
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
    pub fn pieces<T: TTeam, K: TPieceKind>(&self) -> Bitboard {
        let team_bb = match T::TEAM {
            Team::White => self.whites,
            Team::Black => self.blacks,
        };
        let typ_bb = match K::KIND {
            PieceKind::King => self.kings,
            PieceKind::Queen => self.queens,
            PieceKind::Rook => self.rooks,
            PieceKind::Pawn => self.pawns,
            PieceKind::Bishop => self.bishops,
            PieceKind::Knight => self.knights,
        };
        team_bb & typ_bb
    }

    pub fn verify(&self) {
        assert!((self.whites & self.blacks).empty());
        assert_eq!(
            self.whites | self.blacks,
            self.knights | self.queens | self.rooks | self.pawns | self.bishops | self.kings
        );
        assert_eq!(
            self.whites ^ self.blacks,
            self.knights ^ self.queens ^ self.rooks ^ self.pawns ^ self.bishops ^ self.kings
        );
    }

    pub fn debug(&self) {
        println!("Whites: {}", self.whites);
        println!("Blacks: {}", self.blacks);
        println!("Rooks: {}", self.rooks);
        println!("Queens: {}", self.queens);
        println!("Bishops: {}", self.bishops);
        println!("Knights: {}", self.knights);
        println!("Pawns: {}", self.pawns);
        println!("Kings: {}", self.kings);
    }

    pub fn to_fen(&self) -> String {
        let mut ret = String::new();
        for row in (0..8).rev() {
            let mut blanks = 0;
            for col in 0..8 {
                match self.get_piece_info(Position::new(row, col)) {
                    Some(pi) => {
                        let shift = match pi.team {
                            Team::Black => b'a' - b'A',
                            Team::White => 0,
                        };
                        let ch = match pi.kind {
                            PieceKind::King => b'K',
                            PieceKind::Rook => b'R',
                            PieceKind::Pawn => b'P',
                            PieceKind::Queen => b'Q',
                            PieceKind::Bishop => b'B',
                            PieceKind::Knight => b'N',
                        };

                        let v = (ch + shift) as char;

                        if blanks != 0 {
                            ret += &format!("{blanks}");
                            blanks = 0;
                        }
                        ret.push(v);
                    }
                    None => {
                        blanks += 1;
                    }
                }
            }

            if blanks != 0 {
                ret += &format!("{blanks}");
            }
            if row != 0 {
                ret += "/";
            }
        }
        ret
    }

    pub const fn team_pieces<T: TTeam>(&self) -> Bitboard {
        match T::TEAM {
            Team::Black => self.blacks,
            Team::White => self.whites,
        }
    }

    pub fn get_piece_legal_moves<T: TTeam, K: TPieceKind>(&self, pos: Position) -> Bitboard {
        let pseudo_moves = self.get_piece_pseudo_moves::<T, K>(pos);
        let pos_mask = Bitboard::from(pos);

        let mut ret = Bitboard::default();
        for m in pseudo_moves.bit_masks() {
            let mut state = *self;
            // Step 1: Construct the post-move state
            state.kill_piece(m);
            let switch = m | pos_mask;
            *state.get_mut_team_bb::<T>() ^= switch;
            *state.get_mut_piece_bb::<K>() ^= switch;

            // Step 2: Check if the state is valid postmove state
            //         for the team
            if state.is_valid_postmove::<T>() {
                ret |= m;
            }
        }
        ret
    }

    pub fn get_piece_info(&self, piece: Position) -> Option<PieceInfo> {
        let b = Bitboard::from(piece);
        let f = |d: Bitboard| !(b & d).empty();

        let team = if f(self.whites) {
            Team::White
        } else {
            Team::Black
        };

        let kind = if f(self.bishops) {
            PieceKind::Bishop
        } else if f(self.rooks) {
            PieceKind::Rook
        } else if f(self.pawns) {
            PieceKind::Pawn
        } else if f(self.kings) {
            PieceKind::King
        } else if f(self.knights) {
            PieceKind::Knight
        } else if f(self.queens) {
            PieceKind::Queen
        } else {
            return None;
        };

        Some(PieceInfo { team, kind })
    }
    pub fn get_piece_pseudo_moves<T: TTeam, K: TPieceKind>(&self, pos: Position) -> Bitboard {
        if cfg!(debug_assertions) {
            let bb = Bitboard::from(pos);
            let matchers = self.pieces::<T, K>();

            let overlap = bb.where_also(matchers);

            debug_assert!(
                !overlap.empty(),
                "Pseudomoves pos: '{}' does not match the piece info provided: {:?}",
                pos,
                PieceInfo {
                    team: T::TEAM,
                    kind: K::KIND
                }
            );
        }

        let friends = self.team_pieces::<T>();
        match K::KIND {
            PieceKind::Pawn => movement::pawn_moves::<T>(pos, self.whites | self.blacks),
            PieceKind::King => movement::king_moves(pos),
            PieceKind::Rook => movement::rook_moves(pos, self.whites | self.blacks),
            PieceKind::Queen => movement::queen_moves(pos, self.whites | self.blacks),
            PieceKind::Bishop => movement::bishop_moves(pos, self.whites | self.blacks),
            PieceKind::Knight => movement::knight_moves(pos),
        }
        .where_not(friends)
    }

    pub(crate) fn get_mut_team_bb_rt(&mut self, team: Team) -> &mut Bitboard {
        match team {
            Team::White => &mut self.whites,
            Team::Black => &mut self.blacks,
        }
    }

    pub(crate) fn get_mut_piece_bb_rt(&mut self, kind: PieceKind) -> &mut Bitboard {
        match kind {
            PieceKind::King => &mut self.kings,
            PieceKind::Rook => &mut self.rooks,
            PieceKind::Pawn => &mut self.pawns,
            PieceKind::Queen => &mut self.queens,
            PieceKind::Bishop => &mut self.bishops,
            PieceKind::Knight => &mut self.knights,
        }
    }

    pub(crate) fn get_mut_team_bb<T: TTeam>(&mut self) -> &mut Bitboard {
        match T::TEAM {
            Team::White => &mut self.whites,
            Team::Black => &mut self.blacks,
        }
    }

    pub(crate) fn get_mut_piece_bb<K: TPieceKind>(&mut self) -> &mut Bitboard {
        match K::KIND {
            PieceKind::King => &mut self.kings,
            PieceKind::Rook => &mut self.rooks,
            PieceKind::Pawn => &mut self.pawns,
            PieceKind::Queen => &mut self.queens,
            PieceKind::Bishop => &mut self.bishops,
            PieceKind::Knight => &mut self.knights,
        }
    }

    ///
    /// Removes a piece from the bitboard
    /// if the piece doesn't exist, this acts as a no-op
    ///
    fn kill_piece(&mut self, pos: Bitboard) {
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

    pub fn pos_attacker_cnt<AtkBy : TTeam>(&self, pos : Position) -> u8{
        let world = self.whites | self.blacks;
        let enemies = self.team_pieces::<AtkBy>();

        // We project the attackers from the attacked spot
        // And then check if any enemy pieces exist within
        // those spots

        let moves_as_rook = movement::rook_moves(pos, world);
        let moves_as_bish = movement::bishop_moves(pos, world);
        let moves_as_knight = movement::knight_moves(pos);
        let moves_as_king = movement::king_moves(pos);
        let moves_as_queen = movement::queen_moves(pos, world);
        // Project pawn attacks out
        let moves_from_pawn = movement::pawn_attackers::<AtkBy::Enemy>(pos);

        // we have rooklikes and bishlikes, as queens act as a rook and bishop
        // this saves a movement::queen_moves call
        // let rooklikes = (self.rooks | self.queens) & enemies;
        // let bishlikes = (self.bishops | self.queens) & enemies;
        let knights = self.knights & enemies;
        let kings = self.kings & enemies;
        let pawns = self.pawns & enemies;
        let rooks = self.rooks & enemies;
        let bishops = self.bishops & enemies;
        let queens = self.queens & enemies;

        let killer_rooks = moves_as_rook & rooks;
        let killer_bishes = moves_as_bish & bishops;
        let killer_knights = moves_as_knight & knights;
        let killer_kings = moves_as_king & kings;
        let killer_pawns = moves_from_pawn & pawns;
        let killer_queens = moves_as_queen & queens;

        (killer_knights
            | killer_bishes
            | killer_rooks
            | killer_kings
            | killer_pawns
            | killer_queens).count()
    }
    pub fn is_pos_attacked<AtkBy: TTeam>(&self, pos: Position) -> bool {
        let world = self.whites | self.blacks;
        let enemies = self.team_pieces::<AtkBy>();

        // We project the attackers from the attacked spot
        // And then check if any enemy pieces exist within
        // those spots

        let moves_as_rook = movement::rook_moves(pos, world);
        let moves_as_bish = movement::bishop_moves(pos, world);
        let moves_as_knight = movement::knight_moves(pos);
        let moves_as_king = movement::king_moves(pos);
        let moves_as_queen = movement::queen_moves(pos, world);
        // Project pawn attacks out
        let moves_from_pawn = movement::pawn_attackers::<AtkBy::Enemy>(pos);

        // we have rooklikes and bishlikes, as queens act as a rook and bishop
        // this saves a movement::queen_moves call
        // let rooklikes = (self.rooks | self.queens) & enemies;
        // let bishlikes = (self.bishops | self.queens) & enemies;
        let knights = self.knights & enemies;
        let kings = self.kings & enemies;
        let pawns = self.pawns & enemies;
        let rooks = self.rooks & enemies;
        let bishops = self.bishops & enemies;
        let queens = self.queens & enemies;

        let killer_rooks = moves_as_rook & rooks;
        let killer_bishes = moves_as_bish & bishops;
        let killer_knights = moves_as_knight & knights;
        let killer_kings = moves_as_king & kings;
        let killer_pawns = moves_from_pawn & pawns;
        let killer_queens = moves_as_queen & queens;

        !(killer_knights
            | killer_bishes
            | killer_rooks
            | killer_kings
            | killer_pawns
            | killer_queens)
            .empty()
    }

    fn is_valid_postmove<T: TTeam>(&self) -> bool {
        let team_king = self.pieces::<T, GKing>();

        let king_idx = team_king.piece_position();

        !self.is_pos_attacked::<T::Enemy>(king_idx)
    }
}
