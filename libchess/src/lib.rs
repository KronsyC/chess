 #![warn(
    clippy::all,
    clippy::nursery,
    clippy::cargo,
    clippy::style,
    clippy::perf
 )]
pub mod zobrist;
pub mod piece;
pub mod board;
pub mod movement;
pub mod precalc;
pub mod magic_bitboard;
pub mod game;
