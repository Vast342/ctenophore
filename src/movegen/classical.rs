use crate::{
    movegen::lookups::RAYS,
    types::{bitboard::Bitboard, square::Square},
};

pub fn get_rook_attacks_internal(sq: Square, occ: Bitboard) -> Bitboard {
    let mut total_attacks: Bitboard = Bitboard::EMPTY;
    for (dir, item) in RAYS.iter().enumerate().take(4) {
        let mut current_attack: Bitboard = Bitboard(item[sq.as_usize()]);
        if (current_attack & occ).is_not_empty() {
            if dir == 0 || dir == 3 {
                // up and right
                current_attack ^= Bitboard(item[(current_attack & occ).lsb() as usize]);
            } else {
                current_attack ^= Bitboard(item[127 - (current_attack & occ).msb() as usize]);
            }
        }
        total_attacks |= current_attack;
    }
    total_attacks
}

pub fn get_bishop_attacks_internal(sq: Square, occ: Bitboard) -> Bitboard {
    let mut total_attacks: Bitboard = Bitboard::EMPTY;
    for (dir, item) in RAYS.iter().enumerate().skip(4) {
        let mut current_attack: Bitboard = Bitboard(item[sq.as_usize()]);

        if (current_attack & occ).is_not_empty() {
            if dir == 4 || dir == 6 {
                // up right and up left
                current_attack ^= Bitboard(item[(current_attack & occ).lsb() as usize]);
            } else {
                current_attack ^= Bitboard(item[127 - (current_attack & occ).msb() as usize]);
            }
        }
        total_attacks |= current_attack;
    }
    total_attacks
}

pub fn get_lance_attacks_internal(sq: Square, occ: Bitboard, stm: u8) -> Bitboard {
    let mut total_attacks: Bitboard = Bitboard::EMPTY;

    let dir = if stm == 0 { 0 } else { 1 };
    let ray = &RAYS[dir];

    let mut current_attack: Bitboard = Bitboard(ray[sq.as_usize()]);
    if (current_attack & occ).is_not_empty() {
        if dir == 1 {
            current_attack ^= Bitboard(ray[127 - (current_attack & occ).msb() as usize]);
        } else {
            current_attack ^= Bitboard(ray[(current_attack & occ).lsb() as usize]);
        }
    }
    total_attacks |= current_attack;

    total_attacks
}
