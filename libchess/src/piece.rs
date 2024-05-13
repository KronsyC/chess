#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team{
    White,
    Black
}


impl Team{
    pub fn enemy(self) -> Team{
        match self{
            Team::Black => Team::White,
            Team::White => Team::Black
        }
    }
}

pub trait TPieceKind{
    const KIND : PieceKind;
}
pub trait TTeam{
    const TEAM : Team;
    type Enemy : TTeam;
}


pub struct GKing;
pub struct GQueen;
pub struct GRook;
pub struct GBishop;
pub struct GKnight;
pub struct GPawn;

pub struct GWhite;
pub struct GBlack;

impl TPieceKind for GKing{
    const KIND : PieceKind = PieceKind::King;
}
impl TPieceKind for GQueen{
    const KIND : PieceKind = PieceKind::Queen;
}
impl TPieceKind for GRook{
    const KIND : PieceKind = PieceKind::Rook;
}
impl TPieceKind for GBishop{
    const KIND : PieceKind = PieceKind::Bishop;
}
impl TPieceKind for GKnight{
    const KIND : PieceKind = PieceKind::Knight;
}
impl TPieceKind for GPawn{
    const KIND : PieceKind = PieceKind::Pawn;
}

impl TTeam for GWhite{
    const TEAM : Team = Team::White;
    type Enemy = GBlack;
}
impl TTeam for GBlack{
    const TEAM : Team = Team::Black;
    type Enemy = GWhite;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PieceInfo{
    pub kind : PieceKind,
    pub team : Team
}


pub const WHITE_KING : PieceInfo = PieceInfo{kind: PieceKind::King, team: Team::White};
pub const WHITE_QUEEN : PieceInfo = PieceInfo{kind: PieceKind::Queen, team: Team::White};
pub const WHITE_BISHOP : PieceInfo = PieceInfo{kind: PieceKind::Bishop, team: Team::White};
pub const WHITE_ROOK : PieceInfo = PieceInfo{kind: PieceKind::Rook, team: Team::White};
pub const WHITE_KNIGHT : PieceInfo = PieceInfo{kind: PieceKind::Knight, team: Team::White};
pub const WHITE_PAWN : PieceInfo = PieceInfo{kind: PieceKind::Pawn, team: Team::White};

pub const BLACK_KING : PieceInfo = PieceInfo{kind: PieceKind::King, team: Team::Black};
pub const BLACK_QUEEN : PieceInfo = PieceInfo{kind: PieceKind::Queen, team: Team::Black};
pub const BLACK_BISHOP : PieceInfo = PieceInfo{kind: PieceKind::Bishop, team: Team::Black};
pub const BLACK_ROOK : PieceInfo = PieceInfo{kind: PieceKind::Rook, team: Team::Black};
pub const BLACK_KNIGHT : PieceInfo = PieceInfo{kind: PieceKind::Knight, team: Team::Black};
pub const BLACK_PAWN : PieceInfo = PieceInfo{kind: PieceKind::Pawn, team: Team::Black};
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(unused)]
pub enum EnPassant{
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
    None
}


impl EnPassant{
    pub fn from_index(idx : u8) -> EnPassant{
        assert!(idx < 64, "Index out of bounds");
        unsafe{
            std::mem::transmute(idx)
        }
    }

    pub fn to_index(&self) -> u8{
        
        assert!(*self == Self::None, "Cannot translate no enpassant to an index");
        unsafe{
            std::mem::transmute(*self)
        }
    }
}
