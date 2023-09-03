use bitboard::Bitboard;
use std::fs::File;
use std::io::prelude::*;
#[derive(Clone, Copy)]
enum SlidingPiece {
    Rook,
    Bishop,
    Queen,
}

impl SlidingPiece {
    fn to_str(self) -> &'static str {
        match self {
            SlidingPiece::Rook => "rook",
            SlidingPiece::Queen => "queen",
            SlidingPiece::Bishop => "bishop",
        }
    }
}

fn load_attacks(piece: SlidingPiece) -> [Vec<Bitboard>; 64] {
    (0..64)
        .map(|i| {
            let mut file = File::open(format!("data/attacks/{}/{}", piece.to_str(), i)).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();

            (0..buf.len() / 8)
                .map(|j| {
                    let window = &buf[j * 8..j * 8 + 8];
                    Bitboard::from_bits(u64::from_be_bytes(window.try_into().unwrap()))
                })
                .collect()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn load_magics(piece: SlidingPiece) -> [u64; 64] {
    let mut file = File::open(format!("data/magics/{}", piece.to_str())).unwrap();
    let mut buf = [0u8; 64 * 8];
    let read_count = file.read(&mut buf).unwrap();
    if read_count != buf.len() {
        panic!("Magic bitboard file did not contain enough data");
    }

    let mut res = [0u64; 64];

    for i in 0..64 {
        let window = &buf[8 * i..8 * i + 8];
        res[i] = u64::from_be_bytes(window.try_into().unwrap());
    }
    res
}

fn generate_precalc_src(piece: SlidingPiece) -> String {
    let attacks = load_attacks(piece);
    let magics = load_magics(piece);

    let mut src = String::new();
    src += "use super::util::Magic;\n";
    src += "use bitboard::Bitboard;";
    src += &format!("use crate::precalc::masks::{}_MOVEMENT;\n", piece.to_str().to_uppercase());
    src += "\n";
    src += &format!("pub const {}_MAGIC_INFO : [Magic;64] = [\n", piece.to_str().to_uppercase());
    for (idx, (atk, magic)) in attacks.iter().zip(magics).enumerate() {
        let r : String = atk
            .iter()
            .map(|a| format!("Bitboard::from_bits({}),", a.data))
            .fold(String::new(), |p, n| p.clone() + &n);
        let mask = format!("{}_MOVEMENT[{}]", piece.to_str().to_uppercase(), idx);
        let moves = format!("&[{}]", r);
        src += &format!(
            "Magic{{ multiplier : {}, shift : {}, mask : {}, moves : {} }},\n",
            magic, "64 - ".to_owned() + &mask+".count() as u64", mask, moves 
        );
    }

    src += "\n];";
    src
}

fn main() {
    println!("cargo:rerun-if-changed=data");

    let rooks_magics = generate_precalc_src(SlidingPiece::Rook);
    let mut rooks_magics_f = File::create("src/magic_bitboard/rooks.rs").unwrap();
    rooks_magics_f.write_all(rooks_magics.as_bytes()).unwrap();

    let bishops_magics = generate_precalc_src(SlidingPiece::Bishop);
    let mut bishops_magics_f = File::create("src/magic_bitboard/bishops.rs").unwrap();
    bishops_magics_f.write_all(bishops_magics.as_bytes()).unwrap();

    // let queens_magics = generate_precalc_src(SlidingPiece::Queen);
    // let mut queens_magics_f = File::create("src/magic_bitboard/queens.rs").unwrap();
    // queens_magics_f.write_all(queens_magics.as_bytes()).unwrap();
}
