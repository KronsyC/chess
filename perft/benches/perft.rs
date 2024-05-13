use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn perft(n : u32){
    let game = libchess::game::Game::default();

    libchess_perft::perft(black_box(game), n);
}


fn benchmark(c : &mut Criterion){
    c.bench_function("perft 5", |b| b.iter(|| perft(black_box(5))));
    c.bench_function("perft 6", |b| b.iter(|| perft(black_box(6))));
    c.bench_function("perft 7", |b| b.iter(|| perft(black_box(7))));
}

criterion_group!(benches, benchmark);


criterion_main!(benches);
