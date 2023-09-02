use chess_rs::precalc::masks;
use bitboard::Bitboard;
use rand::{Rng};
use rayon::prelude::*;
fn test_magic(magic : u64, mask : Bitboard, hs : &mut std::collections::HashSet<u16>) -> bool{

    hs.clear();
    let count = mask.count();
    for p in mask.mask_permutations(){
        let (result, _) = p.data.overflowing_mul(magic);
        let shifted = (result >> (64 - count)) as u16;
        let absent = hs.insert(shifted);

        if ! absent {
            return false;
        }
    }

    true
}

#[derive(Debug, Copy, Clone)]
struct MagicResult{
    magic : u64,
    shift : u8,
    mask : u64
}

fn rook_attack_bb(idx : u8, world : Bitboard) -> Bitboard{
    let piece = 1u64 << idx;

    let mut up: u64 = piece;
    let mut down : u64 = piece;
    let mut left : u64 = piece;
    let mut right : u64 = piece;

    let mut up_row = idx / 8;
    let mut down_row = idx / 8;
    let mut left_col = idx % 8;
    let mut right_col = idx % 8;
    loop{
        if (up & world.data != 0) || up_row == 7 {
            break;
        }
        up = up << 8 | up;
        up_row+=1;
    }
    loop{
        if down & world.data != 0 || down_row == 0{
            break;
        }
        down = down >> 8 | down;
        down_row -= 1;
    }
    loop{
        if right & world.data != 0 || right_col == 7{ 
            break;
        }
        right = right << 1 | right;
        right_col+=1;
    }
    loop{
        if left & world.data != 0 || left_col == 0{ 
            break;
        }
        left = (left >> 1) | left;
        left_col-=1;
    }

    let ret = Bitboard::from_bits(up | down | left | right).where_not(Bitboard::from_bits(piece));

        return ret;
}

fn calculate_attacks(idx : u8,magic : MagicResult) -> Vec<Bitboard>{
    let mut boards = Vec::<Bitboard>::new();
    let total_states = 2u64.pow(64 - magic.shift as u32);
    boards.resize(total_states as usize, Bitboard::default());

    for perm in Bitboard::from_bits(magic.mask).mask_permutations() {
        let (mul, _) = perm.data.overflowing_mul(magic.magic);
        let shift = mul >> magic.shift;
        let bb = rook_attack_bb(idx, perm);
        boards[shift as usize] = bb;
    }

    boards
}

fn calc_magic_for_mask(mask : Bitboard) -> MagicResult{
        let mut hs = std::collections::HashSet::new();

        let mut rng = rand::thread_rng();

        loop{
            // println!("Attempt #{}", attempt);
            let rand = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
            let is_valid_magic = test_magic(rand, mask, &mut hs);

            if is_valid_magic{
                return MagicResult{magic : rand, shift : (64u8 - mask.count()) as u8, mask : mask.data};
            }
        }
}

fn main() {


    let jobs : Vec<_> = masks::ROOK_MOVEMENT.par_iter().map(|mask| calc_magic_for_mask(*mask)).collect();
    let attacks : Vec<_> = jobs.par_iter().enumerate().map(|(i, job)| calculate_attacks(i as u8, *job)).collect();

    use std::io::prelude::*;

    attacks.iter().enumerate().for_each(|(idx, atks)|{

        let mut attacks_file = std::fs::File::create(format!("data/attacks/rook/{}", idx)).unwrap();

        for a in atks{
            attacks_file.write(&a.data.to_be_bytes()).unwrap();
        }
    });


    let mut magics_file = std::fs::File::create("data/magics/rook").unwrap();
    jobs.iter().for_each(|m| {magics_file.write(&m.magic.to_be_bytes()).unwrap();});
}
