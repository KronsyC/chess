use chess_rs::{
    board,
    piece,
    game::Game
};

fn main() {

    let chess = Game::default();
    let moves = chess.get_piece_moves(9, piece::PieceInfo{team : piece::Team::White, kind: piece::PieceKind::Pawn});
    println!("Hello, world!, \n {:?}", moves);
}
