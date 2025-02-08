use crate::types::bitboard::Bitboard;

pub fn setwise_pawns(our_pawns: Bitboard, us: Bitboard, stm: u8) -> Bitboard {
    if stm == 0 {
        (our_pawns << 9) & !us
    } else {
        (our_pawns >> 9) & !us
    }
}