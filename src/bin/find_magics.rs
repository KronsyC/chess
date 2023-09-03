use bitboard::Bitboard;
use chess_rs::precalc::masks;
use rand::Rng;
use rayon::prelude::*;
fn rook_attack_bb(idx: u8, world: Bitboard) -> Bitboard {
    let piece = 1u64 << idx;

    let mut up: u64 = piece;
    let mut down: u64 = piece;
    let mut left: u64 = piece;
    let mut right: u64 = piece;

    let mut up_row = idx / 8;
    let mut down_row = idx / 8;
    let mut left_col = idx % 8;
    let mut right_col = idx % 8;
    loop {
        if (up & world.data != 0) || up_row == 7 {
            break;
        }
        up = up << 8 | up;
        up_row += 1;
    }
    loop {
        if down & world.data != 0 || down_row == 0 {
            break;
        }
        down = down >> 8 | down;
        down_row -= 1;
    }
    loop {
        if right & world.data != 0 || right_col == 7 {
            break;
        }
        right = right << 1 | right;
        right_col += 1;
    }
    loop {
        if left & world.data != 0 || left_col == 0 {
            break;
        }
        left = (left >> 1) | left;
        left_col -= 1;
    }

    let ret = Bitboard::from_bits(up | down | left | right).where_not(Bitboard::from_bits(piece));

    return ret;
}

fn bishop_attack_bb(idx: u8, world: Bitboard) -> Bitboard {
    let piece = 1u64 << idx;

    let mut upleft: u64 = piece;
    let mut upright: u64 = piece;
    let mut downleft: u64 = piece;
    let mut downright: u64 = piece;

    let row = idx / 8;
    let col = idx % 8;

    let mut upleft_cnt = (7 - row).min(col);
    let mut upright_cnt = (7 - row).min(7 - col);
    let mut downleft_cnt = (row).min(col);
    let mut downright_cnt = (row).min(7 - col);
    loop {
        if (upleft & world.data != 0) || upleft_cnt == 0 {
            break;
        }
        upleft |= upleft << 7;
        upleft_cnt -= 1;
    }
    loop {
        if upright & world.data != 0 || upright_cnt == 0 {
            break;
        }
        upright |= upright << 9;
        upright_cnt -= 1;
    }
    loop {
        if downleft & world.data != 0 || downleft_cnt == 0 {
            break;
        }
        downleft |= downleft >> 9;
        downleft_cnt -= 1;
    }
    loop {
        if downright & world.data != 0 || downright_cnt == 0 {
            break;
        }
        downright |= downright >> 7;
        downright_cnt -= 1;
    }

    let ret = Bitboard::from_bits(upleft | upright | downleft | downright)
        .where_not(Bitboard::from_bits(piece));

    return ret;
}
fn calculate_rook_attacks(idx: u8, magic: MagicResult) -> Vec<Bitboard> {
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
fn calculate_bishop_attacks(idx: u8, magic: MagicResult) -> Vec<Bitboard> {
    let mut boards = Vec::<Bitboard>::new();
    let total_states = 2u64.pow(64 - magic.shift as u32);
    boards.resize(total_states as usize, Bitboard::default());

    for perm in Bitboard::from_bits(magic.mask).mask_permutations() {
        let (mul, _) = perm.data.overflowing_mul(magic.magic);
        let shift = mul >> magic.shift;
        let bb = bishop_attack_bb(idx, perm);
        boards[shift as usize] = bb;
    }

    boards
}

fn calculate_queen_attacks(idx: u8, magic: MagicResult) -> Vec<Bitboard> {
    let mut boards = Vec::<Bitboard>::new();
    let total_states = 2u64.pow(64 - magic.shift as u32);
    boards.resize(total_states as usize, Bitboard::default());

    for perm in Bitboard::from_bits(magic.mask).mask_permutations() {
        let (mul, _) = perm.data.overflowing_mul(magic.magic);
        let shift = mul >> magic.shift;
        let bb = bishop_attack_bb(idx, perm);
        let br = rook_attack_bb(idx, perm);
        boards[shift as usize] = bb.combine_with(br);
    }

    boards
}

const KEYSPACE_INCREASE : u8 = 0;

fn test_magic(magic: u64, mask: Bitboard, hs: &mut std::collections::HashSet<u32>) -> bool {
    hs.clear();
    let count = mask.count();
    for p in mask.mask_permutations() {
        let (result, _) = p.data.overflowing_mul(magic);
        let shifted = (result >> (64 - count - KEYSPACE_INCREASE)) as u32;
        let absent = hs.insert(shifted);

        if !absent {
            return false;
        }
    }

    true
}

#[derive(Debug, Copy, Clone)]
struct MagicResult {
    magic: u64,
    shift: u8,
    mask: u64,
}

fn calc_magic_for_mask(mask: Bitboard) -> MagicResult {
    let mut hs = std::collections::HashSet::new();

    let mut rng = rand::thread_rng();

    loop {
        let rand = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
        let is_valid_magic = test_magic(rand, mask, &mut hs);

        if is_valid_magic {
            return MagicResult {
                magic: rand,
                shift: (64u8 - mask.count() - KEYSPACE_INCREASE) as u8,
                mask: mask.data,
            };
        }
    }
}

fn write_data(magics: Vec<MagicResult>, attacks: Vec<Vec<Bitboard>>, name: &str) {
    use std::io::prelude::*;

    attacks.iter().enumerate().for_each(|(idx, atks)| {
        let mut attacks_file =
            std::fs::File::create(format!("data/attacks/{}/{}", name, idx)).unwrap();

        for a in atks {
            attacks_file.write(&a.data.to_be_bytes()).unwrap();
        }
    });

    let mut magics_file = std::fs::File::create(format!("data/magics/{}", name)).unwrap();
    magics.iter().for_each(|m| {
        magics_file.write(&m.magic.to_be_bytes()).unwrap();
    });
}

fn main() {

    let start = std::time::SystemTime::now();
    println!("Starting at unix time: {}", start.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs());
    let results : Vec<_> = [masks::ROOK_MOVEMENT, masks::BISHOP_MOVEMENT]
        .par_iter().map(|mask_set| mask_set.par_iter().map(|mask| {
                        let r = calc_magic_for_mask(*mask);
                        println!("Found magic after {:?} seconds", start.elapsed().unwrap().as_secs());
                        return r;
        }).collect::<Vec<_>>() ).collect();
    let rook_magics = results[0].to_vec();
    let bishop_magics = results[1].to_vec();


    let rook_attacks: Vec<_> = rook_magics
        .par_iter()
        .enumerate()
        .map(|(i, job)| calculate_rook_attacks(i as u8, *job))
        .collect();
    let bishop_attacks: Vec<_> = bishop_magics
        .par_iter()
        .enumerate()
        .map(|(i, magic)| calculate_bishop_attacks(i as u8, *magic))
        .collect();
    // let queen_attacks: Vec<_> = queen_magics
    //     .par_iter()
    //     .enumerate()
    //     .map(|(i, magic)| calculate_queen_attacks(i as u8, *magic))
    //     .collect();
    write_data(rook_magics, rook_attacks, "rook");
    write_data(bishop_magics, bishop_attacks, "bishop");
    // write_data(queen_magics, queen_attacks, "queen");
}
