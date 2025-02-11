mod lookups;
// mod magic; IT WILL HAPPEN I SWEAR
mod pext;

// when doing pext and magic i'll use #[cfg()] and then #[not(cfg())], if feature pext, use pext, if not, use magic
use lookups::{
    get_gold_attacks_internal, get_king_attacks_internal, get_knight_attacks_internal,
    get_silver_attacks_internal,
};
use pext::{get_bishop_attacks_internal, get_lance_attacks_internal, get_rook_attacks_internal};

use crate::types::{bitboard::Bitboard, square::Square};

pub fn get_rook_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    get_rook_attacks_internal(sq, occ)
}

pub fn get_bishop_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    get_bishop_attacks_internal(sq, occ)
}

pub fn get_lance_attacks(sq: Square, occ: Bitboard, stm: u8) -> Bitboard {
    get_lance_attacks_internal(sq, occ, stm)
}

pub const fn get_king_attacks(sq: Square) -> Bitboard {
    get_king_attacks_internal(sq)
}

pub const fn get_knight_attacks(sq: Square, stm: u8) -> Bitboard {
    get_knight_attacks_internal(sq, stm)
}

pub const fn get_silver_attacks(sq: Square, stm: u8) -> Bitboard {
    get_silver_attacks_internal(sq, stm)
}

pub const fn get_gold_attacks(sq: Square, stm: u8) -> Bitboard {
    get_gold_attacks_internal(sq, stm)
}

pub fn setwise_pawns(our_pawns: Bitboard, stm: u8) -> Bitboard {
    if stm == 0 {
        our_pawns << 9
    } else {
        our_pawns >> 9
    }
}
