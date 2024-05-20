use rand::prelude::*;
use std::fs;
fn main() {

    let simcnt = 1000000;
    let mut rng = rand::thread_rng();

    let mut move_buf = Vec::new();

    let mut bar = indicatif::ProgressBar::new(simcnt); 

    fs::create_dir_all("games");
    for sim in 0..simcnt{
        bar.inc(1); 
        // let mut sf = fs::OpenOptions::new().write(true).create(true).open(format!("games/{sim}.sim")).unwrap();
        let mut game = libchess::game::Game::default();
        use std::io::Write;
        for i in 1..{
            game.board.verify();
            move_buf.clear();
            // sf.write_all(game.to_fen_str().as_bytes()).unwrap();
            // sf.flush().unwrap();
            match game.get_all_moves(&mut move_buf){
                Ok(moves) => {
                    if let Some(chosen_move) = moves.choose(&mut rng){
                        // sf.write_all(format!("; {chosen_move:?}").as_bytes()).unwrap();
                        game.make_move(*chosen_move);
                    }
                    else{
                        // println!("~ Stalemate");
                        break;

                    }
                },
                Err(libchess::game::MoveGenerationError::GameFinished(s)) => {
                    match s{
                        libchess::game::GameState::WhiteVictory | libchess::game::GameState::BlackVictory => println!("MATE!"),
                        _ => {}
                    }
                    // println!("~ {s:?}");
                    break;
                }
            }
            // sf.write_all(b"\n").unwrap();
        }
    }
    bar.finish();
}
