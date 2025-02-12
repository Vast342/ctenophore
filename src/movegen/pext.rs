use std::arch::x86_64::_pext_u64;

use crate::types::{bitboard::Bitboard, square::Square};

use super::lookups::BLOCKER_MASKS;

// just the max number of moves each piece can make (dead center r/b, back rank lance & no blockers)
const LANCE_SHIFT: usize = 7;
const ROOK_SHIFT: usize = 14;
const BISHOP_SHIFT: usize = 12;

const LANCE_BLOCKERS: usize = 1 << LANCE_SHIFT;
const ROOK_BLOCKERS: usize = 1 << ROOK_SHIFT;
const BISHOP_BLOCKERS: usize = 1 << BISHOP_SHIFT;

static LANCE_ATTACKS: [[[Bitboard; 2]; LANCE_BLOCKERS]; 81] =
    unsafe { std::mem::transmute(*include_bytes!("./pext/lance_attacks.bin")) };
static ROOK_ATTACKS: [[Bitboard; ROOK_BLOCKERS]; 81] =
    unsafe { std::mem::transmute(*include_bytes!("./pext/rook_attacks.bin")) };
static BISHOP_ATTACKS: [[Bitboard; BISHOP_BLOCKERS]; 81] =
    unsafe { std::mem::transmute(*include_bytes!("./pext/bishop_attacks.bin")) };

fn pext128(a: Bitboard, mask: Bitboard) -> u128 {
    let mask_lo = mask.lo_bits();
    let out0 = unsafe { _pext_u64(a.lo_bits(), mask_lo) } as u128;
    let out1 = unsafe { _pext_u64(a.hi_bits(), mask.hi_bits()) } as u128;
    out0 | (out1 << mask_lo.count_ones())
}

pub fn get_lance_attacks_internal(sq: Square, occ: Bitboard, stm: u8) -> Bitboard {
    let idx = pext128(occ, Bitboard(BLOCKER_MASKS[stm as usize][sq.as_usize()]));
    LANCE_ATTACKS[sq.as_usize()][idx as usize][stm as usize]
}

pub fn get_rook_attacks_internal(sq: Square, occ: Bitboard) -> Bitboard {
    let idx = pext128(occ, Bitboard(BLOCKER_MASKS[3][sq.as_usize()]));
    ROOK_ATTACKS[sq.as_usize()][idx as usize]
}

pub fn get_bishop_attacks_internal(sq: Square, occ: Bitboard) -> Bitboard {
    let idx = pext128(occ, Bitboard(BLOCKER_MASKS[2][sq.as_usize()]));
    BISHOP_ATTACKS[sq.as_usize()][idx as usize]
}
