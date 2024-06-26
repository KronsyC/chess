#![warn(
    clippy::all,
    clippy::nursery,
    clippy::cargo,
    clippy::style,
    clippy::perf
)]

#[cfg(feature = "parallelism")]
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerftKey {
    depth: u32,
    hash: libchess::zobrist::ZobristHash,
}

#[derive(Clone, Debug, Default)]
pub struct PerftResults {
    pub nodes: u64,
    pub captures: u64,
    pub enpassant: u64,
    pub castles: u64,
    pub kingside_castles: u64,
    pub queenside_castles: u64,
    pub promotions: u64,
    pub regulars: u64,
    pub checkmates: u64,
    pub stalemates: u64,
}

impl std::ops::Add for PerftResults {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            captures: self.captures + rhs.captures,
            castles: self.castles + rhs.castles,
            nodes: self.nodes + rhs.nodes,
            enpassant: self.enpassant + rhs.enpassant,
            promotions: self.promotions + rhs.promotions,
            kingside_castles: self.kingside_castles + rhs.kingside_castles,
            queenside_castles: self.queenside_castles + rhs.queenside_castles,
            regulars: self.regulars + rhs.regulars,
            checkmates: self.checkmates + rhs.checkmates,
            stalemates: self.stalemates + rhs.stalemates,
        }
    }
}

impl std::iter::Sum for PerftResults {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

fn _perft(
    game: &libchess::game::Game,
    depth: u32,
    #[cfg(feature = "zobrist")] map: &dashmap::DashMap<PerftKey, PerftResults>,
    #[cfg(feature = "zobrist")] hash: libchess::zobrist::ZobristHash,
    #[cfg(feature = "zobrist")] zkeys: &libchess::zobrist::ZobKeys,
) -> PerftResults {
    #[cfg(feature = "zobrist")]
    if let Some(val) = map.get(&PerftKey {
        hash: hash.clone(),
        depth,
    }) {
        return val.clone();
    }
    let mut mb = Vec::new();
    mb.reserve(80);

    match game.get_all_moves(&mut mb) {
        Ok(moves) => {
            if moves.is_empty(){
                return PerftResults{
                    stalemates: 1,
                    nodes: 0,
                    ..Default::default()
                }
            }
            if depth == 1 {
                let mut captures = 0;
                let mut castles = 0;
                let mut enpassant = 0;
                let mut promotions = 0;
                let mut regulars = 0;
                let mut kingside_castles = 0;
                let mut queenside_castles = 0;
                let nodes = moves.len() as u64;

                for mov in moves {
                    match mov {
                        libchess::game::GameMove::Capture(_) => captures += 1,
                        libchess::game::GameMove::CastleKingside(_) => {
                            castles += 1;
                            kingside_castles += 1;
                        }
                        libchess::game::GameMove::CastleQueenside(_) => {
                            castles += 1;
                            queenside_castles += 1;
                        }
                        libchess::game::GameMove::Enpassant(_) => {
                            captures += 1;
                            enpassant += 1;
                        }
                        libchess::game::GameMove::CapturePromote { .. } => {
                            captures += 1;
                            promotions += 1;
                        }
                        libchess::game::GameMove::Promote { .. } => {
                            promotions += 1;
                        }
                        libchess::game::GameMove::Regular(_) => {
                            regulars += 1;
                        }
                    }
                }
                return PerftResults {
                    nodes,
                    castles,
                    captures,
                    enpassant,
                    promotions,
                    kingside_castles,
                    queenside_castles,
                    regulars,
                    checkmates: 0,
                    stalemates: 0,
                };
            }

            #[cfg(feature = "parallelism")]
            let it = moves.par_iter();
            #[cfg(not(feature = "parallelism"))]
            let it = moves.iter();
            it.map(|mov| {
                let mut cl = game.clone();
                #[cfg(feature = "zobrist")]
                let hcl = {
                    let mut hcl = hash.clone();
                    cl.make_move(*mov, &mut hcl, zkeys);
                    hcl
                };

                #[cfg(not(feature = "zobrist"))]
                cl.make_move(*mov);
                #[allow(clippy::let_and_return)]
                let v = _perft(
                    &cl,
                    depth - 1,
                    #[cfg(feature = "zobrist")]
                    map,
                    #[cfg(feature = "zobrist")]
                    hcl.clone(),
                    #[cfg(feature = "zobrist")]
                    zkeys,
                );

                #[cfg(feature = "zobrist")]
                map.insert(PerftKey { hash: hcl, depth }, v.clone());
                v
            })
            .sum()
        },
        Err(e) => {
            use libchess::game::MoveGenerationError;
            use libchess::game::GameState;
            match e{
                MoveGenerationError::GameFinished(GameState::Stalemate) => PerftResults{
                    stalemates: 1,
                    nodes: 1,
                    ..Default::default()
                },
                MoveGenerationError::GameFinished(GameState::WhiteVictory | GameState::BlackVictory) => {
                    println!("CM: {}", game.to_fen_str());
                    PerftResults{
                        checkmates: 1,
                        nodes: 1,
                        ..Default::default()
                }},
                _ => unreachable!()
            }
        },
    }
}

pub fn perft(
    game: libchess::game::Game,
    limit: u32,
    #[cfg(feature = "zobrist")] zkeys: &libchess::zobrist::ZobKeys,
) -> PerftResults {
    _perft(
        &game,
        limit,
        #[cfg(feature = "zobrist")]
        &dashmap::DashMap::new(),
        #[cfg(feature = "zobrist")]
        game.get_zobrist_hash(zkeys).unwrap(),
        #[cfg(feature = "zobrist")]
        zkeys,
    )
}
