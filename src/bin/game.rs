use chess_rs::{
    board,
    piece,
    precalc::masks::ROOK_MOVEMENT
};

fn main() {

    let b = board::ChessBoard::default();
    
    let moves1 = b.get_piece_pseudo_moves(0, piece::PieceInfo{team : piece::Team::White, kind: piece::PieceKind::Rook});
    let moves2 = b.get_piece_pseudo_moves(63, piece::PieceInfo{team : piece::Team::Black, kind: piece::PieceKind::Rook});
    println!("Hello, world!,\n {} \n{}", moves1, moves2);
    let world = b.whites.combine_with(b.blacks);
    println!("World: {}", world);
    println!("{}", ROOK_MOVEMENT[0]);
}
