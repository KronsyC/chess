use chess_rs::{
    board,
    piece,
};

fn main() {

    let b = board::ChessBoard::default();
  
    println!("E1 : {} {}", bitboard::Bitboard::COL_E, bitboard::Bitboard::ROW_1);
    
    let moves1 = b.get_piece_legal_moves(9, piece::PieceInfo{team : piece::Team::White, kind: piece::PieceKind::Pawn});
    let moves2 = b.get_piece_legal_moves(54, piece::PieceInfo{team : piece::Team::Black, kind: piece::PieceKind::Pawn});
    println!("Hello, world!,\n {} \n{}", moves1, moves2);
}
