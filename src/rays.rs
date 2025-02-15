use crate::{
    movegen::classical::{get_bishop_attacks_internal, get_rook_attacks_internal},
    types::{bitboard::Bitboard, square::Square},
};

pub const BETWEEN_RAYS: [[Bitboard; 81]; 81] = generate_between_rays();
pub const INTERSECTING_RAYS: [[Bitboard; 81]; 81] = generate_intersecting_rays();

const fn generate_between_rays() -> [[Bitboard; 81]; 81] {
    let mut table = [[Bitboard::EMPTY; 81]; 81];
    let mut src_idx = 0;
    while src_idx < 81 {
        let src = Square(src_idx as u8);
        let src_mask = Bitboard::from_square(src);
        let rook_attacks = get_rook_attacks_internal(src, Bitboard::EMPTY);
        let bishop_attacks = get_bishop_attacks_internal(src, Bitboard::EMPTY);

        let mut dst_idx = 0;
        while dst_idx < 81 {
            table[src_idx][dst_idx] = if src_idx == dst_idx {
                Bitboard::EMPTY
            } else {
                let dst = Square(dst_idx as u8);
                let dst_mask = Bitboard::from_square(dst);

                if rook_attacks.const_and(dst_mask).is_not_empty() {
                    get_rook_attacks_internal(src, dst_mask)
                        .const_and(get_rook_attacks_internal(dst, src_mask))
                } else if bishop_attacks.const_and(dst_mask).is_not_empty() {
                    get_bishop_attacks_internal(src, dst_mask)
                        .const_and(get_bishop_attacks_internal(dst, src_mask))
                } else {
                    Bitboard::EMPTY
                }
            };
            dst_idx += 1;
        }
        src_idx += 1;
    }
    table
}

const fn generate_intersecting_rays() -> [[Bitboard; 81]; 81] {
    let mut table = [[Bitboard::EMPTY; 81]; 81];
    let mut src_idx = 0;
    while src_idx < 81 {
        let src = Square(src_idx as u8);
        let src_mask = Bitboard::from_square(src);
        let rook_attacks = get_rook_attacks_internal(src, Bitboard::EMPTY);
        let bishop_attacks = get_bishop_attacks_internal(src, Bitboard::EMPTY);

        let mut dst_idx = 0;
        while dst_idx < 81 {
            table[src_idx][dst_idx] = if src_idx == dst_idx {
                Bitboard::EMPTY
            } else {
                let dst = Square(dst_idx as u8);
                let dst_mask = Bitboard::from_square(dst);

                if rook_attacks.const_and(dst_mask).is_not_empty() {
                    src_mask.const_or(get_rook_attacks_internal(src, Bitboard::EMPTY).const_and(
                        dst_mask.const_or(get_rook_attacks_internal(dst, Bitboard::EMPTY)),
                    ))
                } else if bishop_attacks.const_and(dst_mask).is_not_empty() {
                    src_mask.const_or(get_bishop_attacks_internal(src, Bitboard::EMPTY).const_and(
                        dst_mask.const_or(get_bishop_attacks_internal(dst, Bitboard::EMPTY)),
                    ))
                } else {
                    Bitboard::EMPTY
                }
            };
            dst_idx += 1;
        }
        src_idx += 1;
    }
    table
}

#[must_use]
pub const fn ray_between(a: Square, b: Square) -> Bitboard {
    BETWEEN_RAYS[a.as_usize()][b.as_usize()]
}

#[must_use]
pub const fn ray_intersecting(a: Square, b: Square) -> Bitboard {
    INTERSECTING_RAYS[a.as_usize()][b.as_usize()]
}
