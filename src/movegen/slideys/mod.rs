// when doing pext and magic i'll use #[cfg()] and then #[not(cfg())], if feature pext, use pext, if not, use magic
use classical::{
    get_bishop_attacks_internal, get_lance_attacks_internal, get_rook_attacks_internal,
};

use crate::types::{bitboard::Bitboard, square::Square};

mod classical;
//mod pext;
//mod magic;

pub fn get_rook_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    get_rook_attacks_internal(sq, occ)
}

pub fn get_bishop_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    get_bishop_attacks_internal(sq, occ)
}

pub fn get_lance_attacks(sq: Square, occ: Bitboard, stm: u8) -> Bitboard {
    get_lance_attacks_internal(sq, occ, stm)
}
