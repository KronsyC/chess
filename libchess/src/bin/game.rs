use libchess::{
    board,
    piece,
    game::Game
};

use positioning::Position;
use rand::prelude::*;
fn main() {

    let mut chess = Game::default();

    let mut mb = Vec::new();

    let mut rng = rand::thread_rng();

    let mut movn = 1;
    loop{
        chess.board.verify();
        // chess.board.debug();
        println!("{}", chess.board.to_fen());
        mb.clear();

        let moves = chess.get_all_moves(&mut mb).unwrap();
        
        
        let chosen_move = moves.choose(&mut rng).unwrap();
      
        println!("(#{movn}) Chosen move (out of {}): {chosen_move:?}", moves.len());
        chess.make_move(*chosen_move);
        for m in moves{
            // println!("M: {m:?}");
        }
        movn +=1;
    }
}
