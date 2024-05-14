#[test]
fn perft_0(){
    let data = libchess_perft::perft(
        libchess::game::Game::default(),
        0
    );
    assert_eq!(data.enpassant, 0);
    assert_eq!(data.nodes, 0);
    assert_eq!(data.captures, 0);
    assert_eq!(data.castles, 0);
    assert_eq!(data.promotions, 0);
}

#[test]
fn perft_1(){
    let data = libchess_perft::perft(
        libchess::game::Game::default(),
        1
    );
    assert_eq!(data.enpassant, 0);
    assert_eq!(data.nodes, 20);
    assert_eq!(data.captures, 0);
    assert_eq!(data.castles, 0);
    assert_eq!(data.promotions, 0);
}

#[test]
fn perft_2(){
    let data = libchess_perft::perft(
        libchess::game::Game::default(),
        2
    );
    assert_eq!(data.enpassant, 0);
    assert_eq!(data.nodes, 400);
    assert_eq!(data.captures, 0);
    assert_eq!(data.castles, 0);
    assert_eq!(data.promotions, 0);
}

#[test]
fn perft_3(){
    let data = libchess_perft::perft(
        libchess::game::Game::default(),
        3
    );
    assert_eq!(data.enpassant, 0);
    assert_eq!(data.nodes, 8902);
    assert_eq!(data.captures, 34);
    assert_eq!(data.castles, 0);
    assert_eq!(data.promotions, 0);
}

#[test]
fn perft_4(){
    let data = libchess_perft::perft(
        libchess::game::Game::default(),
        4
    );
    assert_eq!(data.enpassant, 0);
    assert_eq!(data.nodes, 197281);
    assert_eq!(data.captures, 1576);
    assert_eq!(data.castles, 0);
    assert_eq!(data.promotions, 0);
}
