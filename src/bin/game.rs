use chess_rs::{
    board,
    piece,
    precalc::masks::ROOK_MOVEMENT
};

fn main() {

    let b = board::ChessBoard::default();
    
    let moves1 = b.get_piece_pseudo_moves(3, piece::PieceInfo{team : piece::Team::White, kind: piece::PieceKind::Queen});
    let moves2 = b.get_piece_pseudo_moves(59, piece::PieceInfo{team : piece::Team::Black, kind: piece::PieceKind::Queen});
    println!("Hello, world!,\n {} \n{}", moves1, moves2);
}
