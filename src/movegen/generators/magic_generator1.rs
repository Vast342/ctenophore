// hastily thrown together magic calculation code

use std::time::Instant;

use movegen::classical::{get_bishop_attacks_old, get_lance_attacks_old, get_rook_attacks_old};
use movegen::lookups::BLOCKER_MASKS;
use types::bitboard::Bitboard;
use types::square::Square;

pub mod movegen;
pub mod types;

macro_rules! next {
    ($s:ident) => {{
        let e = $s.a.wrapping_sub($s.b.rotate_left(7));
        $s.a = $s.b ^ $s.c.rotate_left(13);
        $s.b = $s.c.wrapping_add($s.d.rotate_left(37));
        $s.c = $s.d.wrapping_add(e);
        $s.d = e.wrapping_add($s.a);
        $s.d
    }};
}

fn main() {
    println!("woah it compiled");
    // magic generation, maybe saving to file but I won't be able to run that on this platform
    // devving this on the rust playground at play.rust-lang.org

    let start = Instant::now();

    // 0xtastelesscascade from anura again (completely random 2 words)
    let mut gen = Generator::new(0x7a57e1e55ca5cade);
    // lance
    println!("const LANCE_MAGICS: [[u128; 81]; 2] = [");
    for stm in 0..2 {
        print!("[");
        for sq in 0..81 {
            let mask = BLOCKER_MASKS[stm][sq];
            let mut tuples = [(Bitboard::EMPTY, Bitboard::EMPTY); LANCE_BLOCKERS];
            for blocker_id in 0..LANCE_BLOCKERS {
                let blockers = pdep(Bitboard(blocker_id as u128), Bitboard(mask));
                let real = get_lance_attacks_old(Square(sq as u8), blockers, stm as u8);
                tuples[blocker_id] = (blockers, real);
            }
            loop {
                let mut table = [Bitboard::EMPTY; LANCE_BLOCKERS];
                let num = Bitboard(next!(gen) & next!(gen)) & Bitboard::FULL;
                let mut right = true;
                for blocker_id in 1..LANCE_BLOCKERS {
                    let blockers = tuples[blocker_id].0;
                    let real = tuples[blocker_id].1;
                    let idx = ((blockers.0.overflowing_mul(num.0)).0 & Bitboard::FULL.0)
                        >> (81 - LANCE_SHIFT);
                    if table[idx as usize] == Bitboard(0) {
                        table[idx as usize] = real;
                    } else {
                        if table[idx as usize] != real {
                            right = false;
                            break;
                        }
                    }
                }
                if right {
                    print!("{}, ", num.0);
                    break;
                }
            }
        }
        print!("], ");
    }
    println!("];");

    // rook
    println!("const ROOK_MAGICS: [u128; 81] = [");
    for sq in 0..81 {
        let mask = BLOCKER_MASKS[3][sq];
        let mut tuples = [(Bitboard::EMPTY, Bitboard::EMPTY); ROOK_BLOCKERS];
        for blocker_id in 0..ROOK_BLOCKERS {
            let blockers = pdep(Bitboard(blocker_id as u128), Bitboard(mask));
            let real = get_rook_attacks_old(Square(sq as u8), blockers);
            tuples[blocker_id] = (blockers, real);
        }
        loop {
            let mut table = [Bitboard::EMPTY; ROOK_BLOCKERS];
            let num = Bitboard(next!(gen) & next!(gen)) & Bitboard::FULL;
            let mut right = true;
            for blocker_id in 0..ROOK_BLOCKERS {
                let blockers = tuples[blocker_id].0;
                let real = tuples[blocker_id].1;
                let idx = ((blockers.0.overflowing_mul(num.0)).0 & Bitboard::FULL.0)
                    >> (81 - ROOK_SHIFT);
                if table[idx as usize] == Bitboard(0) {
                    table[idx as usize] = real;
                } else {
                    if table[idx as usize] != real {
                        right = false;
                        break;
                    }
                }
            }
            if right {
                print!("{}, ", num.0);
                break;
            }
        }
    }
    println!("];");

    // bishop
    println!("const BISHOP_MAGICS: [u128; 81] = [");
    for sq in 0..81 {
        let mask = BLOCKER_MASKS[2][sq];
        let mut tuples = [(Bitboard::EMPTY, Bitboard::EMPTY); BISHOP_BLOCKERS];
        for blocker_id in 0..BISHOP_BLOCKERS {
            let blockers = pdep(Bitboard(blocker_id as u128), Bitboard(mask));
            let real = get_bishop_attacks_old(Square(sq as u8), blockers);
            tuples[blocker_id] = (blockers, real);
        }
        loop {
            let mut table = [Bitboard::EMPTY; BISHOP_BLOCKERS];
            let num = Bitboard(next!(gen) & next!(gen)) & Bitboard::FULL;
            let mut right = true;
            for blocker_id in 0..BISHOP_BLOCKERS {
                let blockers = tuples[blocker_id].0;
                let real = tuples[blocker_id].1;
                let idx = ((blockers.0.overflowing_mul(num.0)).0 & Bitboard::FULL.0)
                    >> (81 - BISHOP_SHIFT);
                if table[idx as usize] == Bitboard(0) {
                    table[idx as usize] = real;
                } else {
                    if table[idx as usize] != real {
                        right = false;
                        break;
                    }
                }
            }
            if right {
                print!("{}, ", num.0);
                break;
            }
        }
    }
    println!("];");
    println!("Found all magics in {} seconds", start.elapsed().as_secs_f32());
}

pub struct Generator {
    a: u128,
    b: u128,
    c: u128,
    d: u128,
}

impl Generator {
    pub const fn new(seed: u128) -> Self {
        let mut thing = Self {
            a: seed,
            b: seed,
            c: seed,
            d: seed,
        };
        // run a few iterations
        let mut i = 0;
        while i < 15 {
            let _num = next!(thing);
            i += 1;
        }

        thing
    }
}

const LANCE_SHIFT: usize = 7;
const ROOK_SHIFT: usize = 14;
const BISHOP_SHIFT: usize = 12;

const LANCE_BLOCKERS: usize = 1 << 7;
const ROOK_BLOCKERS: usize = 1 << 14;
const BISHOP_BLOCKERS: usize = 1 << 12;

fn pdep(v: Bitboard, mut mask: Bitboard) -> Bitboard {
    let mut dst = Bitboard::EMPTY;
    let mut bit = Bitboard(1);
    while !mask.is_empty() {
        if !v.const_and(bit).is_empty() {
            dst = dst.const_or(mask.const_and(mask.const_neg()))
        }
        mask = mask.const_and(mask.const_sub(Bitboard(1)));
        bit = bit.const_shl(1);
    }
    dst
}
