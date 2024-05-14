fn main(){
    let args : Vec<_> = std::env::args().collect();

    let ply = u32::from_str_radix(args.get(1).expect("Expected 'ply' argument"), 10).expect("Expected valid ply");

    let board = libchess::game::Game::default();

    let begin = std::time::Instant::now();

    let result = libchess_perft::perft(board, ply);

    let elapsed = begin.elapsed();
    println!("Perft({ply}) = {result:?} [{elapsed:?}]");
}
