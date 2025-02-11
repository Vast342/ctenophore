use core::slice;
use std::arch::x86_64::_pext_u64;
use std::fs::File;
use std::io::{self, Write};

use crate::types::{bitboard::Bitboard, square::Square};

use crate::movegen::{
    classical::{get_bishop_attacks_old, get_lance_attacks_old, get_rook_attacks_old},
    lookups::BLOCKER_MASKS,
};

pub mod types;
pub mod movegen;
// Parallel Bits Deposit (simplified for runtime)
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

const LANCE_BLOCKERS: usize = 1 << 7;
const ROOK_BLOCKERS: usize = 1 << 14;
const BISHOP_BLOCKERS: usize = 1 << 12;

fn init_lance() -> Box<[[[Bitboard; 2]; LANCE_BLOCKERS]; 81]> {
    let mut res = Box::new([[[Bitboard::EMPTY; 2]; LANCE_BLOCKERS]; 81]);
    for sq in 0..81 {
        for blocker_id in 0..LANCE_BLOCKERS {
            for stm in 0..2 {
                let mask = Bitboard(BLOCKER_MASKS[stm][sq]);
                let blockers = pdep(Bitboard(blocker_id as u128), mask);
                res[sq][blocker_id][stm] = get_lance_attacks_old(Square(sq as u8), blockers, stm as u8);
            }
        }
    }
    res
}

fn init_rook() -> Box<[[Bitboard; ROOK_BLOCKERS]; 81]> {
    let mut res = Box::new([[Bitboard::EMPTY; ROOK_BLOCKERS]; 81]);
    for sq in 0..81 {
        for blocker_id in 0..ROOK_BLOCKERS {
            let mask = Bitboard(BLOCKER_MASKS[3][sq]);
            let blockers = pdep(Bitboard(blocker_id as u128), mask);
            res[sq][blocker_id] = get_rook_attacks_old(Square(sq as u8), blockers);
        }
    }
    res
}

fn init_bishop() -> Box<[[Bitboard; BISHOP_BLOCKERS]; 81]> {
    let mut res = Box::new([[Bitboard::EMPTY; BISHOP_BLOCKERS]; 81]);
    for sq in 0..81 {
        for blocker_id in 0..BISHOP_BLOCKERS {
            let mask = Bitboard(BLOCKER_MASKS[2][sq]);
            let blockers = pdep(Bitboard(blocker_id as u128), mask);
            res[sq][blocker_id] = get_bishop_attacks_old(Square(sq as u8), blockers);
        }
    }
    res
}

fn write_to_file<T: Copy>(filename: &str, data: &[T]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let bytes = unsafe {
        slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<T>())
    };
    file.write_all(bytes)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let lance_attacks = init_lance();
    let rook_attacks = init_rook();
    let bishop_attacks = init_bishop();

    write_to_file("lance_attacks.bin", lance_attacks.as_ref())?;
    write_to_file("rook_attacks.bin", rook_attacks.as_ref())?;
    write_to_file("bishop_attacks.bin", bishop_attacks.as_ref())?;

    Ok(())
}