use crate::board::ChessBoard;
use crate::piece::{PieceInfo, PieceKind, Team};
use crate::precalc::masks;
use bitboard::Bitboard;
#[derive(Debug, Clone, Copy)]
pub enum Promotion {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Debug, Clone, Copy)]
pub struct RawMove {
    pub to: u8,
    pub from: u8,
    pub piece: PieceInfo,
}

impl RawMove {
    fn new(from: u8, to: u8, piece: PieceInfo) -> Self {
        RawMove {
            from: from,
            to: to,
            piece: piece,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameMove {
    Promote { promotion: Promotion, mov: RawMove },
    CapturePromote { promotion: Promotion, mov: RawMove },
    CastleKingside,
    CastleQueenside,
    Enpassant(RawMove),
    Regular(RawMove),
    Capture(RawMove),
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    WhiteToMove,
    BlackToMove,
    WhiteVictory,
    BlackVictory,
    Stalemate,
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
    fn get_king_moves(&self, piece: u8, team: Team) -> Vec<GameMove> {
        let info = PieceInfo {
            team: team,
            kind: PieceKind::King,
        };
        let legals = self.board.get_piece_legal_moves(piece, info);
        let enemies = self.board.team_pieces(team.enemy());
        let mut ret: Vec<_> = legals
            .bit_masks()
            .map(|mask| (mask.piece_index(), !mask.where_also(enemies).empty()))
            .map(|(pos, is_cap)| {
                if is_cap {
                    GameMove::Capture(RawMove::new(piece, pos, info))
                } else {
                    GameMove::Regular(RawMove::new(piece, pos, info))
                }
            })
            .collect();
        let world = self.board.whites | self.board.blacks;
        let (castle_ks, castle_qs) = if team == Team::White {
            (self.castling.white_kingside, self.castling.white_queenside)
        } else {
            (self.castling.black_kingside, self.castling.black_queenside)
        };
        let (ks_clears, qs_clears, ks_safes, qs_safes) = if team == Team::White {
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
                .bit_masks()
                .all(|mask| self.board.is_pos_attacked(mask.piece_index(), team.enemy()));

            if !attacked {
                ret.push(GameMove::CastleKingside);
            }
        }
        if castle_qs && world.where_also(qs_clears).empty() {
            // Make sure we are not under attack
            let attacked = qs_safes
                .bit_masks()
                .all(|mask| self.board.is_pos_attacked(mask.piece_index(), team.enemy()));

            if !attacked {
                ret.push(GameMove::CastleQueenside);
            }
        }

        return ret;
    }
    fn get_queen_moves(&self, piece: u8, team: Team) -> Vec<GameMove> {
        let info = PieceInfo {
            team: team,
            kind: PieceKind::Queen,
        };
        let legals = self.board.get_piece_legal_moves(piece, info);
        let enemies = self.board.team_pieces(team.enemy());
        legals
            .bit_masks()
            .map(|mask| (mask.piece_index(), !mask.where_also(enemies).empty()))
            .map(|(pos, is_cap)| {
                if is_cap {
                    GameMove::Capture(RawMove::new(piece, pos, info))
                } else {
                    GameMove::Regular(RawMove::new(piece, pos, info))
                }
            })
            .collect()
    }
    fn get_rook_moves(&self, piece: u8, team: Team) -> Vec<GameMove> {
        let info = PieceInfo {
            team: team,
            kind: PieceKind::Rook,
        };
        let legals = self.board.get_piece_legal_moves(piece, info);
        let enemies = self.board.team_pieces(team.enemy());
        legals
            .bit_masks()
            .map(|mask| (mask.piece_index(), !mask.where_also(enemies).empty()))
            .map(|(pos, is_cap)| {
                if is_cap {
                    GameMove::Capture(RawMove::new(piece, pos, info))
                } else {
                    GameMove::Regular(RawMove::new(piece, pos, info))
                }
            })
            .collect()
    }
    fn get_knight_moves(&self, piece: u8, team: Team) -> Vec<GameMove> {
        let info = PieceInfo {
            team: team,
            kind: PieceKind::Knight,
        };
        let legals = self.board.get_piece_legal_moves(piece, info);
        let enemies = self.board.team_pieces(team.enemy());
        legals
            .bit_masks()
            .map(|mask| (mask.piece_index(), !mask.where_also(enemies).empty()))
            .map(|(pos, is_cap)| {
                if is_cap {
                    GameMove::Capture(RawMove::new(piece, pos, info))
                } else {
                    GameMove::Regular(RawMove::new(piece, pos, info))
                }
            })
            .collect()
    }
    fn get_bishop_moves(&self, piece: u8, team: Team) -> Vec<GameMove> {
        let info = PieceInfo {
            team: team,
            kind: PieceKind::Bishop,
        };
        let legals = self.board.get_piece_legal_moves(piece, info);
        let enemies = self.board.team_pieces(team.enemy());
        legals
            .bit_masks()
            .map(|mask| (mask.piece_index(), !mask.where_also(enemies).empty()))
            .map(|(pos, is_cap)| {
                if is_cap {
                    GameMove::Capture(RawMove::new(piece, pos, info))
                } else {
                    GameMove::Regular(RawMove::new(piece, pos, info))
                }
            })
            .collect()
    }
    fn get_pawn_moves(&self, piece: u8, team: Team) -> Vec<GameMove> {
        let info = PieceInfo {
            team: team,
            kind: PieceKind::Pawn,
        };
        let legals = self.board.get_piece_legal_moves(piece, info);
        let enemies = self.board.team_pieces(team.enemy());
        let promotion_row = if team == Team::White {
            Bitboard::ROW_8
        } else {
            Bitboard::ROW_1
        };
        let all_pawn_attacks = if team == Team::White {
            &masks::W_PAWN_ATTACKS
        } else {
            &masks::B_PAWN_ATTACKS
        };
        let pawn_attacks = unsafe { all_pawn_attacks.get_unchecked(piece as usize) };

        let mut r = Vec::<GameMove>::new();

        if !pawn_attacks.where_also(self.enpassant).empty() {
            let ep_idx = self.enpassant.piece_index();
            r.push(GameMove::Enpassant(RawMove::new(piece, ep_idx, info)));
        }

        legals.bit_masks().for_each(|mask| {
            let is_capture = !(mask & enemies).empty();
            let is_promote = !(mask & promotion_row).empty();
            let index = mask.piece_index();
            let mov = RawMove::new(
                piece,
                index,
                PieceInfo {
                    team: team,
                    kind: PieceKind::Pawn,
                },
            );

            match (is_capture, is_promote) {
                (true, true) => {
                    r.push(GameMove::CapturePromote {
                        mov: mov,
                        promotion: Promotion::Rook,
                    });
                    r.push(GameMove::CapturePromote {
                        mov: mov,
                        promotion: Promotion::Bishop,
                    });
                    r.push(GameMove::CapturePromote {
                        mov: mov,
                        promotion: Promotion::Queen,
                    });
                    r.push(GameMove::CapturePromote {
                        mov: mov,
                        promotion: Promotion::Knight,
                    });
                }
                (false, true) => {
                    r.push(GameMove::Promote {
                        mov: mov,
                        promotion: Promotion::Rook,
                    });
                    r.push(GameMove::Promote {
                        mov: mov,
                        promotion: Promotion::Bishop,
                    });
                    r.push(GameMove::Promote {
                        mov: mov,
                        promotion: Promotion::Queen,
                    });
                    r.push(GameMove::Promote {
                        mov: mov,
                        promotion: Promotion::Knight,
                    });
                }
                (true, false) => {
                    r.push(GameMove::Capture(mov));
                }
                (false, false) => {
                    r.push(GameMove::Regular(mov));
                }
            }
        });

        return r;
    }

    pub fn get_piece_moves(&self, piece: u8, info: PieceInfo) -> Vec<GameMove> {
        match info.kind {
            PieceKind::King => self.get_king_moves(piece, info.team),
            PieceKind::Queen => self.get_queen_moves(piece, info.team),
            PieceKind::Rook => self.get_rook_moves(piece, info.team),
            PieceKind::Knight => self.get_knight_moves(piece, info.team),
            PieceKind::Bishop => self.get_bishop_moves(piece, info.team),
            PieceKind::Pawn => self.get_pawn_moves(piece, info.team),
        }
    }
}
