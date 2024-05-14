use crate::board::ChessBoard;
use crate::piece::{
    GBishop, GBlack, GKing, GKnight, GPawn, GQueen, GRook, GWhite, PieceInfo, PieceKind,
    TPieceKind, TTeam, Team,
};
use crate::precalc::masks;
use crate::zobrist::ZKeySet;
use crate::zobrist::ZobKeys;
use crate::zobrist::ZobristHash;
use positioning::Bitboard;
use positioning::Position;
#[derive(Debug, Clone, Copy)]
pub enum Promotion {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Clone, Copy, PartialEq)]
pub struct RawMove {
    pub to: Position,
    pub from: Position,
    pub piece: PieceInfo,
}

impl RawMove {
    fn new(from: Position, to: Position, piece: PieceInfo) -> Self {
        RawMove { from, to, piece }
    }
}

impl std::fmt::Debug for RawMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Move {} -> {} as {:?} {:?}",
            self.from.as_alphanum(),
            self.to.as_alphanum(),
            self.piece.team,
            self.piece.kind
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameMove {
    Promote { promotion: Promotion, mov: RawMove },
    CapturePromote { promotion: Promotion, mov: RawMove },
    CastleKingside(Team),
    CastleQueenside(Team),
    Enpassant(RawMove),
    Regular(RawMove),
    Capture(RawMove),
}

impl GameMove {
    // pub fn smith(&self) -> String{
    //     match self{
    //         Self::Regular(m) => {
    //
    //         }
    //     }
    // }
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    WhiteToMove,
    BlackToMove,
    WhiteVictory,
    BlackVictory,
    Stalemate,
}

#[derive(Debug, thiserror::Error)]
pub enum MoveGenerationError {
    #[error("Game Finished")]
    GameFinished(GameState),
}

impl Default for GameState {
    fn default() -> Self {
        Self::WhiteToMove
    }
}

#[derive(Debug, Clone, Copy)]

///
/// Castling Information
///
/// uses the bottom 4 bits
/// [3] = white kingside
/// [2] = white queenside
/// [1] = black kingside
/// [0] = black queenside
///
pub struct CastleInfo(u8);

impl CastleInfo {
    pub fn unset_white_kingside(&mut self) {
        self.0 &= 0b0111;
    }
    pub fn unset_white_queenside(&mut self) {
        self.0 &= 0b1011;
    }
    pub fn unset_black_kingside(&mut self) {
        self.0 &= 0b1101;
    }
    pub fn unset_black_queenside(&mut self) {
        self.0 &= 0b1110;
    }

    pub fn unset_white(&mut self) {
        self.0 &= 0b0011;
    }
    pub fn unset_black(&mut self) {
        self.0 &= 0b1100;
    }

    pub fn white_kingside(&self) -> bool {
        self.0 & 0b1000 != 0
    }

    pub fn white_queenside(&self) -> bool {
        self.0 & 0b0100 != 0
    }

    pub fn black_kingside(&self) -> bool {
        self.0 & 0b0010 != 0
    }

    pub fn black_queenside(&self) -> bool {
        self.0 & 0b0001 != 0
    }
}

impl Default for CastleInfo {
    fn default() -> Self {
        Self(0b1111)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub board: ChessBoard,
    pub halfmove_num: u64,
    pub fullmove_num: u64,
    pub enpassant: Bitboard,
    pub castling: CastleInfo,
    pub state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: ChessBoard::default(),
            halfmove_num: 0,
            fullmove_num: 0,
            enpassant: Bitboard::default(),
            castling: CastleInfo::default(),
            state: GameState::default(),
        }
    }
}

impl Game {
    pub fn to_fen_str(&self) -> String {
        let bs = self.board.to_fen();
        let side = match self.state {
            GameState::WhiteToMove => 'w',
            GameState::BlackToMove => 'b',
            _ => '-',
        };

        let castling = {
            let mut s = String::new();
            if self.castling.white_kingside() {
                s += "K";
            }
            if self.castling.white_queenside() {
                s += "Q";
            }
            if self.castling.black_kingside() {
                s += "k";
            }
            if self.castling.black_queenside() {
                s += "q";
            }
            if s.len() == 0 {
                s += "-";
            }
            s
        };

        let ep = if self.enpassant.empty() {
            "-".to_owned()
        } else {
            self.enpassant.piece_position().as_alphanum()
        };

        format!(
            "{bs} {side} {castling} {ep} {} {}",
            self.halfmove_num, self.fullmove_num
        )
    }
    pub fn get_active_team(&self) -> Option<Team> {
        match self.state {
            GameState::WhiteToMove => Some(Team::White),
            GameState::BlackToMove => Some(Team::Black),
            _ => None,
        }
    }

    pub fn get_zobrist_hash(&self, zkeys: &ZobKeys) -> Option<ZobristHash> {
        let mut hash = ZobristHash::default();

        if self.castling.white_kingside() {
            hash.update(zkeys.castle_white_ks);
        }
        if self.castling.white_queenside() {
            hash.update(zkeys.castle_white_qs);
        }
        if self.castling.black_kingside() {
            hash.update(zkeys.castle_black_ks);
        }
        if self.castling.black_queenside() {
            hash.update(zkeys.castle_black_qs);
        }

        if !self.enpassant.empty() {
            let pos = self.enpassant.piece_position();
            println!("ENPASSANT");
            hash.update(zkeys.enpassant_keys_for(pos));
        }

        match self.state {
            GameState::BlackToMove => {
                hash.update(zkeys.black_to_move);
            }
            GameState::WhiteToMove => {}
            _ => return None,
        }

        for i in 0..64 {
            let pos = Position::from_integral(i);
            if let Some(pi) = self.board.get_piece_info(pos) {
                let hashes = zkeys.piece_keys_for(pi.kind, pi.team);
                let h = hashes.at_pos(pos);
                hash.update(h);
            }
        }

        Some(hash)
    }

    fn check_checkmated<T: TTeam>(&self) -> bool {
        let enemy_king = match T::TEAM {
            Team::White => self.board.pieces::<GBlack, GKing>(),
            Team::Black => self.board.pieces::<GWhite, GKing>(),
        }
        .piece_position();

        let check = match T::TEAM {
            Team::White => self.board.is_pos_attacked::<GWhite>(enemy_king),
            Team::Black => self.board.is_pos_attacked::<GBlack>(enemy_king),
        };
        let king_moves = match T::TEAM {
            Team::White => self
                .board
                .get_piece_legal_moves::<GBlack, GKing>(enemy_king),
            Team::Black => self
                .board
                .get_piece_legal_moves::<GWhite, GKing>(enemy_king),
        };

        check && king_moves.empty()
    }

    fn handle_cap_side_effects(
        &mut self,
        piece: PieceInfo,
        from: Position,
        to: Position,
        cap: PieceInfo,
        #[cfg(feature = "zobrist")] hash: &mut ZobristHash,
        #[cfg(feature = "zobrist")] zkeys: &ZobKeys,
    ) {
        use PieceKind::*;
        use Team::*;
        assert_eq!(piece.team.enemy(), cap.team, "Capturing Enemy");
        match (cap.team, cap.kind) {
            (White, Rook) => match to.integral() {
                0 => self.castling.unset_white_queenside(),
                7 => self.castling.unset_white_kingside(),
                _ => {}
            },
            (Black, Rook) => match to.integral() {
                56 => self.castling.unset_black_queenside(),
                63 => self.castling.unset_white_kingside(),
                _ => {}
            },
            (_, _) => {}
        }

        self.halfmove_num = 0;
    }
    fn handle_move_side_effects(
        &mut self,
        piece: PieceInfo,
        from: Position,
        to: Position,
        #[cfg(feature = "zobrist")] hash: &mut ZobristHash,
        #[cfg(feature = "zobrist")] zkeys: &ZobKeys,
    ) {
        let prev_ep = self.enpassant;
        self.enpassant = Bitboard::default();
        match (piece.kind, piece.team) {
            (PieceKind::King, Team::White) => {
                #[cfg(feature = "zobrist")]
                {
                    if self.castling.white_kingside(){
                        hash.update(zkeys.castle_white_ks);
                    }
                    if self.castling.white_queenside(){
                        hash.update(zkeys.castle_white_qs);
                    }
                }
                self.castling.unset_white();
            }
            (PieceKind::King, Team::Black) => {
                #[cfg(feature = "zobrist")]
                {
                    if self.castling.black_kingside(){
                        hash.update(zkeys.castle_black_ks);
                    }
                    if self.castling.black_queenside(){
                        hash.update(zkeys.castle_black_qs);
                    }
                }
                self.castling.unset_black();
            }
            (PieceKind::Rook, Team::White) => match from.integral() {
                0 => {
                    #[cfg(feature = "zobrist")]
                    if self.castling.white_queenside(){
                        hash.update(zkeys.castle_white_qs);
                    }
                    self.castling.unset_white_queenside();
                },
                7 => {
                    #[cfg(feature = "zobrist")]
                    if self.castling.white_kingside(){
                        hash.update(zkeys.castle_white_ks);
                    }
                    self.castling.unset_white_kingside();
                },
                _ => {}
            },
            (PieceKind::Rook, Team::Black) => match from.integral() {
                56 => {
                    #[cfg(feature = "zobrist")]
                    if self.castling.black_queenside(){
                        hash.update(zkeys.castle_black_qs);
                    }
                    self.castling.unset_black_queenside();
                },
                63 => {
                    #[cfg(feature = "zobrist")]
                    if self.castling.black_kingside(){
                        hash.update(zkeys.castle_black_ks);
                    }
                    self.castling.unset_black_kingside();
                },
                _ => {}
            },
            (PieceKind::Pawn, Team::White) => {
                // En passant check
                if to.integral() - from.integral() == 16 {
                    self.enpassant = Bitboard::from(from).shift_up();
                }
            }
            (PieceKind::Pawn, Team::Black) => {
                // En passant check
                if from.integral() - to.integral() == 16 {
                    self.enpassant = Bitboard::from(from).shift_down();
                }
            }
            (_, _) => {}
        }

        match piece.team {
            Team::White => {
                if self.check_checkmated::<GBlack>() {
                    self.state = GameState::WhiteVictory;
                }
            }
            Team::Black => {
                if self.check_checkmated::<GWhite>() {
                    self.state = GameState::BlackVictory;
                }
            }
        }
        match piece.kind {
            PieceKind::Pawn => self.halfmove_num = 0,
            _ => self.halfmove_num += 1,
        }

        #[cfg(feature = "zobrist")]
        {
            if prev_ep.empty() && !self.enpassant.empty(){
               hash.update(zkeys.enpassant_keys_for(to));
            }
            else if !prev_ep.empty() && self.enpassant.empty(){
                hash.update(zkeys.enpassant_keys_for(prev_ep.piece_position()));
            }
            else if !prev_ep.empty() && !self.enpassant.empty(){
               hash.update(zkeys.enpassant_keys_for(prev_ep.piece_position()));
               hash.update(zkeys.enpassant_keys_for(to));
            }
        }
    }

    pub fn make_move(
        &mut self,
        gmove: GameMove,
        #[cfg(feature = "zobrist")] hash: &mut ZobristHash,
        #[cfg(feature = "zobrist")] zkeys: &ZobKeys,
    ) {
        match gmove {
            GameMove::Regular(mov) => {
                assert_eq!(
                    self.board.get_piece_info(mov.to),
                    None,
                    "Regular move goes to empty square"
                );
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;
                *self.board.get_mut_piece_bb_rt(mov.piece.kind) ^= switch;

                #[cfg(feature = "zobrist")]
                {
                    // Zobrist Update
                    let zk = zkeys.piece_keys_for(mov.piece.kind, mov.piece.team);

                    hash.update(zk.at_pos(mov.from));
                    hash.update(zk.at_pos(mov.to));
                }
                self.handle_move_side_effects(
                    mov.piece,
                    mov.from,
                    mov.to,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
            }
            GameMove::Capture(mov) => {
                assert!(
                    matches!(self.board.get_piece_info(mov.to), Some(_)),
                    "Capture move goes to occupied square"
                );
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);

                let cap_pi = self.board.get_piece_info(mov.to).unwrap();

                #[cfg(feature = "zobrist")]
                {
                    // Zobrist Update
                    let p_zk = zkeys.piece_keys_for(mov.piece.kind, mov.piece.team);
                    let c_zk = zkeys.piece_keys_for(cap_pi.kind, cap_pi.team);

                    hash.update(p_zk.at_pos(mov.from));
                    hash.update(p_zk.at_pos(mov.to));
                    hash.update(c_zk.at_pos(mov.to));
                }

                self.mask(Bitboard::from(mov.to).negative());
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;
                *self.board.get_mut_piece_bb_rt(mov.piece.kind) ^= switch;

                self.handle_move_side_effects(
                    mov.piece,
                    mov.from,
                    mov.to,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
                self.handle_cap_side_effects(
                    mov.piece,
                    mov.from,
                    mov.to,
                    cap_pi,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
            }
            GameMove::Promote { promotion, mov } => {
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);

                let new_pk = match promotion {
                    Promotion::Knight => PieceKind::Knight,
                    Promotion::Bishop => PieceKind::Bishop,
                    Promotion::Queen => PieceKind::Queen,
                    Promotion::Rook => PieceKind::Rook,
                };

                #[cfg(feature = "zobrist")]
                {
                    // Zobrist Update
                    let p_zk = zkeys.piece_keys_for(mov.piece.kind, mov.piece.team);
                    let prom_zk = zkeys.piece_keys_for(new_pk, mov.piece.team);

                    hash.update(p_zk.at_pos(mov.from));
                    hash.update(prom_zk.at_pos(mov.to));
                }
                let pbb = match promotion {
                    Promotion::Rook => self.board.get_mut_piece_bb::<GRook>(),
                    Promotion::Queen => self.board.get_mut_piece_bb::<GQueen>(),
                    Promotion::Bishop => self.board.get_mut_piece_bb::<GBishop>(),
                    Promotion::Knight => self.board.get_mut_piece_bb::<GKnight>(),
                };

                *pbb |= Bitboard::from(mov.to);
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;
                self.mask(Bitboard::from(mov.from).negative());

                self.handle_move_side_effects(
                    mov.piece,
                    mov.from,
                    mov.to,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
            }
            GameMove::CapturePromote { promotion, mov } => {
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);
                let cap_pi = self.board.get_piece_info(mov.to).unwrap();

                let new_pk = match promotion {
                    Promotion::Knight => PieceKind::Knight,
                    Promotion::Bishop => PieceKind::Bishop,
                    Promotion::Queen => PieceKind::Queen,
                    Promotion::Rook => PieceKind::Rook,
                };

                #[cfg(feature = "zobrist")]
                {
                    let cap_pi = self.board.get_piece_info(mov.to).unwrap();

                    // Zobrist update
                    let p_zk = zkeys.piece_keys_for(mov.piece.kind, mov.piece.team);
                    let prom_zk = zkeys.piece_keys_for(new_pk, mov.piece.team);
                    let cap_zk = zkeys.piece_keys_for(cap_pi.kind, cap_pi.team);

                    hash.update(cap_zk.at_pos(mov.to));
                    hash.update(p_zk.at_pos(mov.from));
                    hash.update(prom_zk.at_pos(mov.to));
                }
                self.mask(Bitboard::from(mov.to).negative());

                self.mask(Bitboard::from(mov.to).negative());

                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;
                let pbb = match promotion {
                    Promotion::Rook => self.board.get_mut_piece_bb::<GRook>(),
                    Promotion::Queen => self.board.get_mut_piece_bb::<GQueen>(),
                    Promotion::Bishop => self.board.get_mut_piece_bb::<GBishop>(),
                    Promotion::Knight => self.board.get_mut_piece_bb::<GKnight>(),
                };

                *pbb |= Bitboard::from(mov.to);
                self.mask(Bitboard::from(mov.from).negative());

                self.handle_move_side_effects(
                    mov.piece,
                    mov.from,
                    mov.to,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
                self.handle_cap_side_effects(
                    mov.piece,
                    mov.from,
                    mov.to,
                    cap_pi,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
            }
            GameMove::Enpassant(mov) => {
                let cap_pos = match mov.piece.team {
                    Team::White => self.enpassant.shift_down(),
                    Team::Black => self.enpassant.shift_up(),
                };

                let cap_pi = self.board.get_piece_info(cap_pos.piece_position()).unwrap();

                #[cfg(feature = "zobrist")]
                {
                    let cap_pi = self.board.get_piece_info(cap_pos.piece_position()).unwrap();

                    // Zobrist Update
                    let p_zk = zkeys.piece_keys_for(mov.piece.kind, mov.piece.team);
                    let cap_zk = zkeys.piece_keys_for(cap_pi.kind, cap_pi.team);

                    hash.update(p_zk.at_pos(mov.from));
                    hash.update(p_zk.at_pos(mov.to));
                    hash.update(cap_zk.at_pos(cap_pos.piece_position()));
                }

                // Delete the captured piece
                self.mask(cap_pos.negative());

                // Switch pawn to new position
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);

                // Apply switch to team and piece, i.e move
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;
                *self.board.get_mut_piece_bb_rt(mov.piece.kind) ^= switch;

                self.handle_move_side_effects(
                    mov.piece,
                    mov.from,
                    mov.to,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
                self.handle_cap_side_effects(
                    mov.piece,
                    mov.from,
                    mov.to,
                    cap_pi,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
            }
            GameMove::CastleKingside(team) => {
                use crate::precalc::masks::castling;
                let (ks, rs) = match team {
                    Team::White => (castling::W_KS_KING_SWITCH, castling::W_KS_ROOK_SWITCH),
                    Team::Black => (castling::B_KS_KING_SWITCH, castling::B_KS_ROOK_SWITCH),
                };

                let (king_from, king_to) = match team{
                    Team::White => (Position::new(0, 4), Position::new(0, 6)),
                    Team::Black => (Position::new(7, 4), Position::new(7, 6))
                };
                #[cfg(feature = "zobrist")]
                {
                    // Zobrist Update
                    let k_zk = zkeys.piece_keys_for(PieceKind::King, team);
                    let r_zk = zkeys.piece_keys_for(PieceKind::Rook, team);

                    match team {
                        Team::White => {
                            hash.update(k_zk.at_pos(Position::new(0, 4)));
                            hash.update(r_zk.at_pos(Position::new(0, 7)));
                            hash.update(k_zk.at_pos(Position::new(0, 6)));
                            hash.update(r_zk.at_pos(Position::new(0, 5)));
                        }
                        Team::Black => {
                            hash.update(k_zk.at_pos(Position::new(7, 4)));
                            hash.update(r_zk.at_pos(Position::new(7, 7)));
                            hash.update(k_zk.at_pos(Position::new(7, 6)));
                            hash.update(r_zk.at_pos(Position::new(7, 5)));
                        }
                    }
                }

                let team_switch = ks | rs;
                *self.board.get_mut_team_bb_rt(team) ^= team_switch;
                *self.board.get_mut_piece_bb::<GKing>() ^= ks;
                *self.board.get_mut_piece_bb::<GRook>() ^= rs;

                self.handle_move_side_effects(
                    PieceInfo{team, kind: PieceKind::King},
                    king_from,
                    king_to,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
            }
            GameMove::CastleQueenside(team) => {
                use crate::precalc::masks::castling;
                let (ks, rs) = match team {
                    Team::White => (castling::W_QS_KING_SWITCH, castling::W_QS_ROOK_SWITCH),
                    Team::Black => (castling::B_QS_KING_SWITCH, castling::B_QS_ROOK_SWITCH),
                };

                let (king_from, king_to) = match team{
                    Team::White => (Position::new(0, 4), Position::new(0, 2)),
                    Team::Black => (Position::new(7, 4), Position::new(7, 2))
                };

                #[cfg(feature = "zobrist")]
                {
                    // Zobrist Update
                    let k_zk = zkeys.piece_keys_for(PieceKind::King, team);
                    let r_zk = zkeys.piece_keys_for(PieceKind::Rook, team);

                    match team {
                        Team::White => {
                            hash.update(k_zk.at_pos(Position::new(0, 4)));
                            hash.update(r_zk.at_pos(Position::new(0, 0)));
                            hash.update(k_zk.at_pos(Position::new(0, 2)));
                            hash.update(r_zk.at_pos(Position::new(0, 3)));
                        }
                        Team::Black => {
                            hash.update(k_zk.at_pos(Position::new(7, 4)));
                            hash.update(r_zk.at_pos(Position::new(7, 0)));
                            hash.update(k_zk.at_pos(Position::new(7, 2)));
                            hash.update(r_zk.at_pos(Position::new(7, 3)));
                        }
                    }
                }
                let team_switch = ks | rs;
                *self.board.get_mut_team_bb_rt(team) ^= team_switch;
                *self.board.get_mut_piece_bb::<GKing>() ^= ks;
                *self.board.get_mut_piece_bb::<GRook>() ^= rs;


                self.handle_move_side_effects(
                    PieceInfo{team, kind: PieceKind::King},
                    king_from,
                    king_to,
                    #[cfg(feature = "zobrist")]
                    hash,
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );
            }
        }

        match self.state {
            GameState::WhiteToMove => {
                self.state = GameState::BlackToMove;
                #[cfg(feature = "zobrist")]
                hash.update(zkeys.black_to_move);
            }
            GameState::BlackToMove => {
                self.state = GameState::WhiteToMove;
                #[cfg(feature = "zobrist")]
                hash.update(zkeys.black_to_move);
            }
            _ => {}
        }

        // 50 full moves have been made with no "progress"
        if self.halfmove_num == 100 {
            self.state = GameState::Stalemate;
        }
    }

    fn switch(&mut self, sw: Bitboard) {
        self.board.kings ^= sw;
        self.board.queens ^= sw;
        self.board.rooks ^= sw;
        self.board.bishops ^= sw;
        self.board.knights ^= sw;
        self.board.pawns ^= sw;
        self.board.whites ^= sw;
        self.board.blacks ^= sw;
    }

    fn mask(&mut self, mask: Bitboard) {
        self.board.kings &= mask;
        self.board.queens &= mask;
        self.board.rooks &= mask;
        self.board.bishops &= mask;
        self.board.knights &= mask;
        self.board.pawns &= mask;
        self.board.whites &= mask;
        self.board.blacks &= mask;
    }

    fn get_king_moves<'m, T: TTeam>(
        &self,
        piece: Position,
        move_buf: &'m mut Vec<GameMove>,
    ) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            kind: PieceKind::King,
            team: T::TEAM,
        };
        let legals = self.board.get_piece_legal_moves::<T, GKing>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        // Add basic moves
        for cap_pos in captures.positions() {
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions() {
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        let world = self.board.whites | self.board.blacks;

        // Determine Castling Ability
        let (castle_ks, castle_qs) = if T::TEAM == Team::White {
            (
                self.castling.white_kingside(),
                self.castling.white_queenside(),
            )
        } else {
            (
                self.castling.black_kingside(),
                self.castling.black_queenside(),
            )
        };

        // Required safe and clear squares in order to castle
        let (ks_clears, qs_clears, ks_safes, qs_safes) = if T::TEAM == Team::White {
            (
                Bitboard::W_KINGSIDE_CLEARS,
                Bitboard::W_QUEENSIDE_CLEARS,
                Bitboard::W_KINGSIDE_SAFES,
                Bitboard::W_QUEENSIDE_SAFES,
            )
        } else {
            (
                Bitboard::B_KINGSIDE_CLEARS,
                Bitboard::B_QUEENSIDE_CLEARS,
                Bitboard::B_KINGSIDE_SAFES,
                Bitboard::B_QUEENSIDE_SAFES,
            )
        };

        if castle_ks && world.where_also(ks_clears).empty() {
            // Make sure we are not under attack
            let attacked = ks_safes
                .positions()
                .all(|pos| self.board.is_pos_attacked::<T::Enemy>(pos));

            if !attacked {
                move_buf.push(GameMove::CastleKingside(T::TEAM));
            }
        }
        if castle_qs && world.where_also(qs_clears).empty() {
            // Make sure we are not under attack
            let attacked = qs_safes
                .positions()
                .all(|pos| self.board.is_pos_attacked::<T::Enemy>(pos));

            if !attacked {
                move_buf.push(GameMove::CastleQueenside(T::TEAM));
            }
        }

        move_buf
    }
    fn get_queen_moves<'m, T: TTeam>(
        &self,
        piece: Position,
        move_buf: &'m mut Vec<GameMove>,
    ) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team: T::TEAM,
            kind: PieceKind::Queen,
        };
        let legals = self.board.get_piece_legal_moves::<T, GQueen>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        for cap_pos in captures.positions() {
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions() {
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        move_buf
    }

    fn get_rook_moves<'m, T: TTeam>(
        &self,
        piece: Position,
        move_buf: &'m mut Vec<GameMove>,
    ) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team: T::TEAM,
            kind: PieceKind::Rook,
        };
        let legals = self.board.get_piece_legal_moves::<T, GRook>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        for cap_pos in captures.positions() {
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions() {
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        move_buf
    }
    fn get_knight_moves<'m, T: TTeam>(
        &self,
        piece: Position,
        move_buf: &'m mut Vec<GameMove>,
    ) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team: T::TEAM,
            kind: PieceKind::Knight,
        };
        let legals = self.board.get_piece_legal_moves::<T, GKnight>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        for cap_pos in captures.positions() {
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions() {
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        move_buf
    }
    fn get_bishop_moves<'m, T: TTeam>(
        &self,
        piece: Position,
        move_buf: &'m mut Vec<GameMove>,
    ) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team: T::TEAM,
            kind: PieceKind::Bishop,
        };
        let legals = self.board.get_piece_legal_moves::<T, GBishop>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();

        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        for cap_pos in captures.positions() {
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions() {
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        move_buf
    }
    fn get_pawn_moves<'m, T: TTeam>(
        &self,
        piece: Position,
        move_buf: &'m mut Vec<GameMove>,
    ) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team: T::TEAM,
            kind: PieceKind::Pawn,
        };
        let legals = self.board.get_piece_legal_moves::<T, GPawn>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let promotion_row = if T::TEAM == Team::White {
            Bitboard::ROW_8
        } else {
            Bitboard::ROW_1
        };
        let all_pawn_attacks = if T::TEAM == Team::White {
            &masks::W_PAWN_ATTACKS
        } else {
            &masks::B_PAWN_ATTACKS
        };
        let pawn_attacks = unsafe { all_pawn_attacks.get_unchecked(piece.integral() as usize) };

        if !(*pawn_attacks & self.enpassant).empty() {
            let ep_idx = self.enpassant.piece_position();
            move_buf.push(GameMove::Enpassant(RawMove::new(piece, ep_idx, info)));
        }

        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();
        let cap_promotes = captures & promotion_row;
        let reg_promotes = noncaptures & promotion_row;

        for cap in captures.positions() {
            move_buf.push(GameMove::Capture(RawMove {
                piece: info,
                from: piece,
                to: cap,
            }))
        }
        for noncap in noncaptures.positions() {
            move_buf.push(GameMove::Regular(RawMove {
                piece: info,
                from: piece,
                to: noncap,
            }))
        }
        for cap_prom in cap_promotes.positions() {
            let mov = RawMove {
                piece: info,
                from: piece,
                to: cap_prom,
            };
            move_buf.push(GameMove::CapturePromote {
                mov,
                promotion: Promotion::Rook,
            });
            move_buf.push(GameMove::CapturePromote {
                mov,
                promotion: Promotion::Bishop,
            });
            move_buf.push(GameMove::CapturePromote {
                mov,
                promotion: Promotion::Queen,
            });
            move_buf.push(GameMove::CapturePromote {
                mov,
                promotion: Promotion::Knight,
            });
        }
        for noncap_prom in reg_promotes.positions() {
            let mov = RawMove {
                piece: info,
                from: piece,
                to: noncap_prom,
            };
            move_buf.push(GameMove::Promote {
                mov,
                promotion: Promotion::Rook,
            });
            move_buf.push(GameMove::Promote {
                mov,
                promotion: Promotion::Bishop,
            });
            move_buf.push(GameMove::Promote {
                mov,
                promotion: Promotion::Queen,
            });
            move_buf.push(GameMove::Promote {
                mov,
                promotion: Promotion::Knight,
            });
        }

        move_buf
    }

    pub fn get_piece_moves<'m, T: TTeam, K: TPieceKind>(
        &self,
        piece: Position,
        move_buf: &'m mut Vec<GameMove>,
    ) -> &'m Vec<GameMove> {
        match K::KIND {
            PieceKind::King => self.get_king_moves::<T>(piece, move_buf),
            PieceKind::Queen => self.get_queen_moves::<T>(piece, move_buf),
            PieceKind::Rook => self.get_rook_moves::<T>(piece, move_buf),
            PieceKind::Knight => self.get_knight_moves::<T>(piece, move_buf),
            PieceKind::Bishop => self.get_bishop_moves::<T>(piece, move_buf),
            PieceKind::Pawn => self.get_pawn_moves::<T>(piece, move_buf),
        }
    }

    pub fn get_all_moves<'m>(
        &self,
        move_buf: &'m mut Vec<GameMove>,
    ) -> Result<&'m [GameMove], MoveGenerationError> {
        match self.get_active_team() {
            Some(Team::White) => Ok(self.static_get_all_moves::<GWhite>(move_buf)),
            Some(Team::Black) => Ok(self.static_get_all_moves::<GBlack>(move_buf)),
            None => Err(MoveGenerationError::GameFinished(self.state)),
        }
    }
    fn static_get_all_moves<'m, T: TTeam>(
        &self,
        move_buf: &'m mut Vec<GameMove>,
    ) -> &'m [GameMove] {
        let bb = match T::TEAM {
            Team::White => self.board.whites,
            Team::Black => self.board.blacks,
        };

        let pawns = bb & self.board.pawns;
        let rooks = bb & self.board.rooks;
        let knights = bb & self.board.knights;
        let kings = bb & self.board.kings;
        let queens = bb & self.board.queens;
        let bishops = bb & self.board.bishops;

        for pos in pawns.positions() {
            self.get_pawn_moves::<T>(pos, move_buf);
        }
        for pos in rooks.positions() {
            self.get_rook_moves::<T>(pos, move_buf);
        }
        for pos in knights.positions() {
            self.get_knight_moves::<T>(pos, move_buf);
        }
        for pos in kings.positions() {
            self.get_king_moves::<T>(pos, move_buf);
        }
        for pos in queens.positions() {
            self.get_queen_moves::<T>(pos, move_buf);
        }
        for pos in bishops.positions() {
            self.get_bishop_moves::<T>(pos, move_buf);
        }

        move_buf
    }
}
