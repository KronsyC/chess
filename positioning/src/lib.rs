 #![warn(
    clippy::all,
    clippy::nursery,
    clippy::cargo,
    clippy::style,
    clippy::perf
 )]

pub mod bitboard;
pub mod position;

pub use bitboard::*;
pub use position::*;
