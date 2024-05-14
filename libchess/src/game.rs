use crate::board::ChessBoard;
use crate::piece::{PieceInfo, PieceKind, Team, TTeam, TPieceKind, GKing, GQueen, GRook, GBishop, GKnight, GPawn, GWhite, GBlack};
use crate::precalc::masks;
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
        RawMove {
            from,
            to,
            piece,
        }
    }
}

impl std::fmt::Debug for RawMove{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move {} -> {} as {:?} {:?}", self.from.as_alphanum(), self.to.as_alphanum(), self.piece.team, self.piece.kind)
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


impl GameMove{
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
pub enum MoveGenerationError{
    #[error("Game Finished")]
    GameFinished(GameState)
}

impl Default for GameState {
    fn default() -> Self {
        Self::WhiteToMove
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CastleInfo {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Default for CastleInfo {
    fn default() -> Self {
        Self {
            white_kingside: true,
            black_kingside: true,
            white_queenside: true,
            black_queenside: true,
        }
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

    pub fn to_fen_str(&self) -> String{
        let bs = self.board.to_fen();
        let side = match self.state{
            GameState::WhiteToMove => 'w',
            GameState::BlackToMove => 'b',
            _ => '-'
        };

        let castling = {
            let mut s = String::new();
            if self.castling.white_kingside{
                s += "K";
            }
            if self.castling.white_queenside{
                s += "Q";
            }
            if self.castling.black_kingside{
                s += "k";
            }
            if self.castling.black_queenside{
                s += "q";
            }
            if s.len() == 0{
                s += "-";
            }
            s
        };

        let ep = if self.enpassant.empty(){
            "-".to_owned()
        }            
        else{
            self.enpassant.piece_position().as_alphanum()
        };


        format!("{bs} {side} {castling} {ep} {} {}", self.halfmove_num, self.fullmove_num)
    }
    pub fn get_active_team(&self) -> Option<Team>{
        match self.state{
            GameState::WhiteToMove => Some(Team::White),
            GameState::BlackToMove => Some(Team::Black),
            _ => None
        }
    }

    fn check_checkmated<T : TTeam>(&self) -> bool{
        let enemy_king = match T::TEAM{
            Team::White => self.board.pieces::<GBlack, GKing>(),
            Team::Black => self.board.pieces::<GWhite, GKing>()
        }.piece_position();

        let check = match T::TEAM{
            Team::White => self.board.is_pos_attacked::<GWhite>(enemy_king), 
            Team::Black => self.board.is_pos_attacked::<GBlack>(enemy_king), 
        };
        let king_moves = match T::TEAM{
            Team::White => self.board.get_piece_legal_moves::<GBlack, GKing>(enemy_king), 
            Team::Black => self.board.get_piece_legal_moves::<GWhite, GKing>(enemy_king), 
        };

        check && king_moves.empty()
    }


    fn handle_cap_side_effects(&mut self, piece : PieceInfo, from : Position, to : Position, cap : PieceInfo){ 
        use Team::*;
        use PieceKind::*;
        assert_eq!(piece.team.enemy(), cap.team, "Capturing Enemy");
        match (cap.team, cap.kind){
            (White, Rook) => {
                match to.integral(){
                    0 => {
                        self.castling.white_queenside = false
                    },
                    7 => self.castling.white_kingside = false,
                    _ => {}
                }
            },
            (Black, Rook) => {
                match to.integral(){
                    56 => self.castling.black_queenside = false,
                    63 => self.castling.black_kingside = false,
                    _ => {}
                }
            },
            (_, _) => {}
        }

        self.halfmove_num = 0;
    }
    fn handle_move_side_effects(&mut self, piece : PieceInfo, from : Position, to : Position){
        self.enpassant = Bitboard::default();
        match (piece.kind, piece.team){
            (PieceKind::King, Team::White) => {
                self.castling.white_kingside = false;
                self.castling.white_queenside = false;
            },
            (PieceKind::King, Team::Black) => {
                self.castling.black_kingside = false;
                self.castling.black_queenside = false;
            },
            (PieceKind::Rook, Team::White) => {
                match from.integral(){
                    0 => self.castling.white_queenside = false,
                    7 => self.castling.white_kingside = false,
                    _ => {}
                }
            },
            (PieceKind::Rook, Team::Black) => {
                match from.integral(){
                    56 => self.castling.black_queenside = false,
                    63 => self.castling.black_kingside = false,
                    _ => {}
                }
            },
            (PieceKind::Pawn, Team::White) => {
                // En passant check
                if to.integral() - from.integral() == 16{
                    self.enpassant = Bitboard::from(from).shift_up();
                }
            },
            (PieceKind::Pawn, Team::Black) => {
                // En passant check
                if from.integral() - to.integral() == 16{
                    self.enpassant = Bitboard::from(from).shift_down();
                }
            }
            (_, _) => {}
        }

        match piece.team{
            Team::White => {
                if self.check_checkmated::<GBlack>(){
                    self.state = GameState::WhiteVictory;
                }
            },
            Team::Black => {
                if self.check_checkmated::<GWhite>(){
                    self.state = GameState::BlackVictory;
                }
            }
        }
        match piece.kind{
            PieceKind::Pawn => {
                self.halfmove_num = 0
            },
            _ => self.halfmove_num += 1
        }
        
    }

    pub fn make_move(&mut self, gmove : GameMove){
        match gmove{
            GameMove::Regular(mov) => {

                assert_eq!(self.board.get_piece_info(mov.to), None, "Regular move goes to empty square");
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;
                *self.board.get_mut_piece_bb_rt(mov.piece.kind) ^= switch;

                self.handle_move_side_effects(mov.piece, mov.from, mov.to);

            },
            GameMove::Capture(mov) => {
                
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);

                let cap_pi = self.board.get_piece_info(mov.to).unwrap();

                self.mask(Bitboard::from(mov.to).negative());
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;
                *self.board.get_mut_piece_bb_rt(mov.piece.kind) ^= switch;
                self.handle_move_side_effects(mov.piece, mov.from, mov.to);
                self.handle_cap_side_effects(mov.piece, mov.from, mov.to, cap_pi);
            },
            GameMove::Promote { promotion, mov } => {
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;

                self.mask(Bitboard::from(mov.from).negative());

                let pbb = match promotion{
                    Promotion::Rook => self.board.get_mut_piece_bb::<GRook>(),
                    Promotion::Queen => self.board.get_mut_piece_bb::<GQueen>(),
                    Promotion::Bishop => self.board.get_mut_piece_bb::<GBishop>(),
                    Promotion::Knight => self.board.get_mut_piece_bb::<GKnight>()
                };

                *pbb |= Bitboard::from(mov.to);
                self.handle_move_side_effects(mov.piece, mov.from, mov.to);
            },
            GameMove::CapturePromote { promotion, mov } => {
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);
                let cap_pi = self.board.get_piece_info(mov.to).unwrap();

                self.mask(Bitboard::from(mov.to).negative());
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;

                self.mask(Bitboard::from(mov.from).negative());

                let pbb = match promotion{
                    Promotion::Rook => self.board.get_mut_piece_bb::<GRook>(),
                    Promotion::Queen => self.board.get_mut_piece_bb::<GQueen>(),
                    Promotion::Bishop => self.board.get_mut_piece_bb::<GBishop>(),
                    Promotion::Knight => self.board.get_mut_piece_bb::<GKnight>()
                };

                *pbb |= Bitboard::from(mov.to);

                self.handle_move_side_effects(mov.piece, mov.from, mov.to);
                self.handle_cap_side_effects(mov.piece, mov.from, mov.to, cap_pi);
            },
            GameMove::Enpassant(mov) => {

                let cap_pos = match mov.piece.team{
                    Team::White => self.enpassant.shift_down(),
                    Team::Black => self.enpassant.shift_up()
                };

                let cap_pi = self.board.get_piece_info(cap_pos.piece_position()).unwrap();

                // Delete the captured piece 
                self.mask(cap_pos.negative());

                // Switch pawn to new position
                let switch = Bitboard::from(mov.from) | Bitboard::from(mov.to);

                // Apply switch to team and piece, i.e move
                *self.board.get_mut_team_bb_rt(mov.piece.team) ^= switch;
                *self.board.get_mut_piece_bb_rt(mov.piece.kind) ^= switch;

                self.handle_move_side_effects(mov.piece, mov.from, mov.to);
                self.handle_cap_side_effects(mov.piece, mov.from, mov.to, cap_pi);
            },
            GameMove::CastleKingside(team) => {
                use crate::precalc::masks::castling;
                let (ks, rs) = match team{
                    Team::White => (castling::W_KS_KING_SWITCH, castling::W_KS_ROOK_SWITCH),
                    Team::Black => (castling::B_KS_KING_SWITCH, castling::B_KS_ROOK_SWITCH),
                };

                let team_switch = ks | rs;
                *self.board.get_mut_team_bb_rt(team) ^= team_switch;
                *self.board.get_mut_piece_bb::<GKing>() ^= ks;
                *self.board.get_mut_piece_bb::<GRook>() ^= rs;

                match team{
                    Team::White => {
                        self.castling.white_kingside = false;
                        self.castling.white_queenside = false;
                    },
                    Team::Black => {
                        self.castling.black_kingside = false;
                        self.castling.black_queenside = false;
                    }
                };

                self.halfmove_num+=1;

                match team{
                    Team::White => {
                        if self.check_checkmated::<GBlack>(){
                            self.state = GameState::WhiteVictory;
                        }
                    },
                    Team::Black=> {
                        if self.check_checkmated::<GWhite>(){
                            self.state = GameState::BlackVictory;
                        }
                    }
                }
            },
            GameMove::CastleQueenside(team) => {
                use crate::precalc::masks::castling;
                let (ks, rs) = match team{
                    Team::White => (castling::W_QS_KING_SWITCH, castling::W_QS_ROOK_SWITCH),
                    Team::Black => (castling::B_QS_KING_SWITCH, castling::B_QS_ROOK_SWITCH),
                };

                let team_switch = ks | rs;
                *self.board.get_mut_team_bb_rt(team) ^= team_switch;
                *self.board.get_mut_piece_bb::<GKing>() ^= ks;
                *self.board.get_mut_piece_bb::<GRook>() ^= rs;

                match team{
                    Team::White => {
                        self.castling.white_kingside = false;
                        self.castling.white_queenside = false;
                    },
                    Team::Black => {
                        self.castling.black_kingside = false;
                        self.castling.black_queenside = false;
                    }
                };

                self.halfmove_num+=1;

                match team{
                    Team::White => {
                        if self.check_checkmated::<GBlack>(){
                            self.state = GameState::WhiteVictory;
                        }
                    },
                    Team::Black=> {
                        if self.check_checkmated::<GWhite>(){
                            self.state = GameState::BlackVictory;
                        }
                    }
                }
            }
        }


        match self.state{
            GameState::WhiteToMove => self.state = GameState::BlackToMove,
            GameState::BlackToMove => self.state = GameState::WhiteToMove,
            _ => {}
        }

        // 50 full moves have been made with no "progress"
        if self.halfmove_num == 100{
            self.state = GameState::Stalemate;
        }
    }

    fn switch(&mut self, sw : Bitboard){
        self.board.kings ^= sw;
        self.board.queens ^= sw;
        self.board.rooks ^= sw;
        self.board.bishops ^= sw; 
        self.board.knights ^= sw;
        self.board.pawns ^= sw; 
        self.board.whites ^= sw;
        self.board.blacks ^= sw;
    }

    fn mask(&mut self, mask : Bitboard){
        self.board.kings &= mask;
        self.board.queens &= mask;
        self.board.rooks &= mask;
        self.board.bishops &= mask; 
        self.board.knights &= mask;
        self.board.pawns &= mask; 
        self.board.whites &= mask;
        self.board.blacks &= mask;
    }

    fn get_king_moves<'m, T : TTeam>(&self, piece: Position, move_buf : &'m mut Vec<GameMove>) -> &'m Vec<GameMove> {

        let info = PieceInfo{
            kind : PieceKind::King,
            team : T::TEAM
        };
        let legals = self.board.get_piece_legal_moves::<T, GKing>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();


        // Add basic moves
        for cap_pos in captures.positions(){
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions(){
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        let world = self.board.whites | self.board.blacks;

        // Determine Castling Ability
        let (castle_ks, castle_qs) = if T::TEAM == Team::White {
            (self.castling.white_kingside, self.castling.white_queenside)
        } else {
            (self.castling.black_kingside, self.castling.black_queenside)
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
    fn get_queen_moves<'m, T : TTeam>(&self, piece: Position, move_buf : &'m mut Vec<GameMove>) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team : T::TEAM,
            kind: PieceKind::Queen,
        };
        let legals = self.board.get_piece_legal_moves::<T, GQueen>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        for cap_pos in captures.positions(){
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions(){
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        move_buf
    }

    fn get_rook_moves<'m, T : TTeam>(&self, piece: Position, move_buf : &'m mut Vec<GameMove>) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team : T::TEAM,
            kind: PieceKind::Rook,
        };
        let legals = self.board.get_piece_legal_moves::<T, GRook>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        for cap_pos in captures.positions(){
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions(){
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        move_buf
    }
    fn get_knight_moves<'m, T : TTeam>(&self, piece: Position, move_buf : &'m mut Vec<GameMove>) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team : T::TEAM,
            kind: PieceKind::Knight,
        };
        let legals = self.board.get_piece_legal_moves::<T, GKnight>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();
        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        for cap_pos in captures.positions(){
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions(){
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        move_buf
    }
    fn get_bishop_moves<'m, T : TTeam>(&self, piece: Position, move_buf : &'m mut Vec<GameMove>) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team : T::TEAM,
            kind: PieceKind::Bishop,
        };
        let legals = self.board.get_piece_legal_moves::<T, GBishop>(piece);
        let enemies = self.board.team_pieces::<T::Enemy>();

        let captures = legals & enemies;
        let noncaptures = legals & enemies.negative();

        for cap_pos in captures.positions(){
            move_buf.push(GameMove::Capture(RawMove::new(piece, cap_pos, info)));
        }

        for noncap_pos in noncaptures.positions(){
            move_buf.push(GameMove::Regular(RawMove::new(piece, noncap_pos, info)));
        }

        move_buf
    }
    fn get_pawn_moves<'m, T : TTeam>(&self, piece: Position, move_buf : &'m mut Vec<GameMove>) -> &'m Vec<GameMove> {
        let info = PieceInfo {
            team : T::TEAM,
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

        for cap in captures.positions(){
            move_buf.push(GameMove::Capture(RawMove{piece: info, from: piece, to:cap})) 
        }
        for noncap in noncaptures.positions(){
            move_buf.push(GameMove::Regular(RawMove{piece: info, from: piece, to:noncap})) 
        }
        for cap_prom in cap_promotes.positions(){
            let mov = RawMove{piece: info, from : piece, to : cap_prom};
            move_buf.push(GameMove::CapturePromote{mov, promotion:Promotion::Rook});
            move_buf.push(GameMove::CapturePromote{mov, promotion:Promotion::Bishop});
            move_buf.push(GameMove::CapturePromote{mov, promotion:Promotion::Queen});
            move_buf.push(GameMove::CapturePromote{mov, promotion:Promotion::Knight});
        }
        for noncap_prom in reg_promotes.positions(){
            let mov = RawMove{piece: info, from : piece, to : noncap_prom};
            move_buf.push(GameMove::Promote{mov, promotion:Promotion::Rook});
            move_buf.push(GameMove::Promote{mov, promotion:Promotion::Bishop});
            move_buf.push(GameMove::Promote{mov, promotion:Promotion::Queen});
            move_buf.push(GameMove::Promote{mov, promotion:Promotion::Knight});

        }

        move_buf
    }

    pub fn get_piece_moves<'m, T : TTeam, K : TPieceKind>(&self, piece: Position, move_buf : &'m mut Vec<GameMove>) -> &'m Vec<GameMove> {
        match K::KIND {
            PieceKind::King => self.get_king_moves::<T>(piece, move_buf),
            PieceKind::Queen => self.get_queen_moves::<T>(piece, move_buf),
            PieceKind::Rook => self.get_rook_moves::<T>(piece, move_buf),
            PieceKind::Knight => self.get_knight_moves::<T>(piece, move_buf),
            PieceKind::Bishop => self.get_bishop_moves::<T>(piece, move_buf),
            PieceKind::Pawn => self.get_pawn_moves::<T>(piece, move_buf),
        }
    }


    pub fn get_all_moves<'m>(&self, move_buf : &'m mut Vec<GameMove>) -> Result<&'m [GameMove], MoveGenerationError>{
        match self.get_active_team(){
            Some(Team::White) => Ok(self.static_get_all_moves::<GWhite>(move_buf)),
            Some(Team::Black) => Ok(self.static_get_all_moves::<GBlack>(move_buf)),
            None => Err(MoveGenerationError::GameFinished(self.state))
        }
    }
    fn static_get_all_moves<'m, T : TTeam>(&self, move_buf : &'m mut Vec<GameMove>) -> &'m [GameMove]{
        let bb = match T::TEAM{
            Team::White => self.board.whites,
            Team::Black => self.board.blacks
        };

        let pawns = bb & self.board.pawns;
        let rooks = bb & self.board.rooks;
        let knights = bb & self.board.knights;
        let kings = bb & self.board.kings;
        let queens = bb & self.board.queens;
        let bishops = bb & self.board.bishops;

        for pos in pawns.positions(){
            self.get_pawn_moves::<T>(pos, move_buf);
        }
        for pos in rooks.positions(){
            self.get_rook_moves::<T>(pos, move_buf);
        }
        for pos in knights.positions(){
            self.get_knight_moves::<T>(pos, move_buf);
        }
        for pos in kings.positions(){
            self.get_king_moves::<T>(pos, move_buf);
        }
        for pos in queens.positions(){
            self.get_queen_moves::<T>(pos, move_buf);
        }
        for pos in bishops.positions(){
            self.get_bishop_moves::<T>(pos, move_buf);
        }

        move_buf
    } 
}
