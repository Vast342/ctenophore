use crate::{
    movegen::lookups::RAYS,
    types::{bitboard::Bitboard, square::Square},
};

pub const fn get_rook_attacks_internal(sq: Square, occ: Bitboard) -> Bitboard {
    let mut total_attacks = Bitboard::EMPTY;
    let mut dir = 0;

    while dir < 4 {
        let current_attack = Bitboard(RAYS[dir][sq.as_usize()]);
        let blocker = current_attack.const_and(occ);

        let adjusted_attack = if blocker.is_not_empty() {
            if dir == 0 || dir == 3 {
                // up and right
                current_attack.const_xor(Bitboard(RAYS[dir][blocker.lsb() as usize]))
            } else {
                current_attack.const_xor(Bitboard(RAYS[dir][127 - blocker.msb() as usize]))
            }
        } else {
            current_attack
        };

        total_attacks = total_attacks.const_or(adjusted_attack);
        dir += 1;
    }
    total_attacks
}

pub const fn get_bishop_attacks_internal(sq: Square, occ: Bitboard) -> Bitboard {
    let mut total_attacks = Bitboard::EMPTY;
    let mut dir = 4;

    while dir < 8 {
        let current_attack = Bitboard(RAYS[dir][sq.as_usize()]);
        let blocker = current_attack.const_and(occ);

        let adjusted_attack = if blocker.is_not_empty() {
            if dir == 4 || dir == 6 {
                // up-right and up-left
                current_attack.const_xor(Bitboard(RAYS[dir][blocker.lsb() as usize]))
            } else {
                current_attack.const_xor(Bitboard(RAYS[dir][127 - blocker.msb() as usize]))
            }
        } else {
            current_attack
        };

        total_attacks = total_attacks.const_or(adjusted_attack);
        dir += 1;
    }
    total_attacks
}

pub const fn get_lance_attacks_internal(sq: Square, occ: Bitboard, stm: u8) -> Bitboard {
    let dir = if stm == 0 { 0 } else { 1 };
    let ray = &RAYS[dir];
    let current_attack = Bitboard(ray[sq.as_usize()]);
    let blocker = current_attack.const_and(occ);

    let adjusted_attack = if blocker.is_not_empty() {
        if dir == 1 {
            current_attack.const_xor(Bitboard(ray[127 - blocker.msb() as usize]))
        } else {
            current_attack.const_xor(Bitboard(ray[blocker.lsb() as usize]))
        }
    } else {
        current_attack
    };

    adjusted_attack
}
