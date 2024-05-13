use rayon::prelude::*;
pub fn perft(mut game : libchess::game::Game, limit : u32) -> u64{

    if limit == 0{
        return 0;
    }
    let mut mb = Vec::new();
    mb.reserve(80);
    let moves = game.get_all_moves(&mut mb).unwrap();
  
    if limit == 1{
        return moves.len() as u64;
    }

    moves.iter().map(|mov| {
        let mut cl = game.clone();
        cl.make_move(*mov);
        perft(cl, limit - 1)
    }).sum()
}
