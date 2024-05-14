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
        println!("{}", chess.to_fen_str());
        mb.clear();

        let moves = chess.get_all_moves(&mut mb).unwrap();
        
        
        let chosen_move = moves.choose(&mut rng).unwrap();
      
        chess.make_move(*chosen_move);
        for m in moves{
            // println!("M: {m:?}");
        }
        movn +=1;
    }
}
