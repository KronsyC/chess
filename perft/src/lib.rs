use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct PerftResults{
    pub captures : u64,
    pub castles : u64,
    pub nodes : u64,
    pub enpassant : u64,
    pub promotions: u64
}


impl std::ops::Add for PerftResults{
    type Output = PerftResults;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            captures: self.captures + rhs.captures,
            castles : self.castles + rhs.castles,
            nodes : self.nodes + rhs.nodes,
            enpassant: self.enpassant + rhs.enpassant,
            promotions : self.promotions + rhs.promotions
        }
    }
}

impl std::iter::Sum for PerftResults{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

impl Default for PerftResults{
    fn default() -> Self {
        Self{
            castles: 0,
            captures: 0,
            nodes: 0,
            enpassant: 0,
            promotions: 0,
        }
    }
}

pub fn perft(game : libchess::game::Game, limit : u32) -> PerftResults{

    if limit == 0{
        return PerftResults::default();
    }
    let mut mb = Vec::new();
    mb.reserve(80);
    if let Ok(moves) = game.get_all_moves(&mut mb){
        if limit == 1{
            let mut captures = 0;
            let mut castles = 0;
            let mut enpassant = 0;
            let mut promotions = 0;
            let nodes = moves.len() as u64;

            for mov in moves{
                match mov{
                    libchess::game::GameMove::Capture(_) => captures += 1,
                    libchess::game::GameMove::CastleKingside(_) | libchess::game::GameMove::CastleQueenside(_) => castles+=1,
                    libchess::game::GameMove::Enpassant(_) => {
                        captures += 1;
                        enpassant += 1;
                    },
                    libchess::game::GameMove::CapturePromote { .. } => {
                        captures+=1;
                        promotions+=1;
                    },
                    libchess::game::GameMove::Promote { .. } => {
                        promotions += 1;
                    }
                    _ => {}
                }
            }
            return PerftResults{
                nodes,
                castles,
                captures,
                enpassant,
                promotions
            };
        }

        moves.par_iter().map(|mov| {
            let mut cl = game.clone();
            cl.make_move(*mov);
            perft(cl, limit - 1)
        }).sum()
    }
    else{
        PerftResults::default()
    }
  
}
