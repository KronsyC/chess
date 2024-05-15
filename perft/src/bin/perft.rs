fn main(){
    let args : Vec<_> = std::env::args().collect();

    let ply = args.get(1).expect("Expected 'ply' argument").parse::<u32>().expect("Expected valid ply");

    let board = libchess::game::Game::default();

    let begin = std::time::Instant::now();

    #[cfg(not(feature = "zobrist"))]
    let result = libchess_perft::perft(board, ply);


    #[cfg(feature = "zobrist")]
    let result = {
        // let mut rng = rand::rngs::StdRng::from_seed(std::array::from_fn(|i| (i * 6 + 5) as u8));
        let mut rng = rand::thread_rng();
        let zkeys = libchess::zobrist::ZobKeys::generate(&mut rng);
        libchess_perft::perft(board, ply, &zkeys)
    };

    let elapsed = begin.elapsed();
    println!("Perft({ply}) = {result:?} [{elapsed:?}]");
}
