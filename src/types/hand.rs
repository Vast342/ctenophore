use std::fmt;

use super::piece::Piece;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hand(pub u32);

impl Default for Hand {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Hand {
    pub const EMPTY: Self = Self(0);

    // bits
    const PAWN_BITS: u32 = 5;
    const LANCE_BITS: u32 = 3;
    const KNIGHT_BITS: u32 = 3;
    const SILVER_BITS: u32 = 3;
    const GOLD_BITS: u32 = 2;
    const BISHOP_BITS: u32 = 2;
    const ROOK_BITS: u32 = 2;

    // offsets
    const PAWN_OFFSET: u32 = 0;
    const LANCE_OFFSET: u32 = Self::BISHOP_OFFSET + Self::BISHOP_BITS;
    const KNIGHT_OFFSET: u32 = Self::LANCE_OFFSET + Self::LANCE_BITS;
    const SILVER_OFFSET: u32 = Self::KNIGHT_OFFSET + Self::KNIGHT_BITS;
    const GOLD_OFFSET: u32 = Self::SILVER_OFFSET + Self::SILVER_BITS;
    const BISHOP_OFFSET: u32 = Self::ROOK_OFFSET + Self::ROOK_BITS;
    const ROOK_OFFSET: u32 = Self::PAWN_OFFSET + Self::PAWN_BITS;

    // arrays
    const OFFSETS: [u32; 7] = [
        Self::PAWN_OFFSET,
        Self::LANCE_OFFSET,
        Self::KNIGHT_OFFSET,
        Self::SILVER_OFFSET,
        Self::GOLD_OFFSET,
        Self::BISHOP_OFFSET,
        Self::ROOK_OFFSET,
    ];
    const BITS: [u32; 7] = [
        Self::PAWN_BITS,
        Self::LANCE_BITS,
        Self::KNIGHT_BITS,
        Self::SILVER_BITS,
        Self::GOLD_BITS,
        Self::BISHOP_BITS,
        Self::ROOK_BITS,
    ];
    const MASKS: [u32; 7] = {
        let mut result = [0; 7];
        let mut i = 0;
        while i < 7 {
            result[i] = ((1 << Self::BITS[i]) - 1) << Self::OFFSETS[i];
            i += 1;
        }
        result
    };

    // current number
    pub fn num(&self, piece: Piece) -> u8 {
        let piece_type = piece.piece() as usize;
        ((self.0 & Self::MASKS[piece_type]) >> Self::OFFSETS[piece_type]) as u8
    }

    // increase value by 1
    pub fn inc(&mut self, piece: Piece) {
        self.set(piece, self.num(piece) as u32 + 1);
    }

    // decrease value by 1
    pub fn dec(&mut self, piece: Piece) {
        self.set(piece, self.num(piece) as u32 - 1);
    }

    // set value to something
    pub fn set(&mut self, piece: Piece, new_count: u32) {
        let piece_type = piece.piece() as usize;
        self.0 = (self.0 & !Self::MASKS[piece_type]) | (new_count << Self::OFFSETS[piece_type])
    }
}

// ignores capitalisation for now
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = "".to_owned();

        for i in 0..7 {
            output += format!("{}{} ", self.num(Piece(i)), Piece(i)).as_str()
        }

        write!(f, "{output}")
    }
}
