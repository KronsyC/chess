use libchess::{
    board,
    piece,
    game::Game
};

use positioning::Position;
use rand::prelude::*;
fn main() {

    // let mut chess = Game::default();
    //
    // let mut mb = Vec::new();
    //
    // let mut rng = rand::rngs::StdRng::from_seed(std::array::from_fn(|i| 10));
    //
    // let zobs = libchess::zobrist::ZobKeys::generate(&mut rng);
    // let mut zhash = chess.get_zobrist_hash(&zobs).unwrap();
    // let mut movn = 1;
    // loop{
    //     mb.clear();
    //
    //
    //     let moves = chess.get_all_moves(&mut mb).unwrap();
    //    
    //
    //     let chosen_move = moves.choose(&mut rng).unwrap();
    //   
    //     println!("(#{movn}) Making chosen move (out of {}): {chosen_move:?}", moves.len());
    //     chess.make_move(*chosen_move, &mut zhash, &zobs);
    //     movn +=1;
    //
    //     println!("Freshly calculating zobrist hash...");
    //     let fresh_zob = chess.get_zobrist_hash(&zobs).unwrap();
    //    
    //     println!("Incremental Hash: {zhash:?}. Fresh Hash: {fresh_zob:?}");
    //     println!("FEN: {}", chess.to_fen_str());
    //
    //     assert_eq!(zhash, fresh_zob, "Hashes match");
    // }
}
