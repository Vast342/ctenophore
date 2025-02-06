use core::panic;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece(pub u8);

//                            real  promoted
pub const NUM_PIECE_TYPES: u8 = 8 + 6;

impl Piece {
    pub const PAWN: Piece = Self(0);
    pub const ROOK: Piece = Self(1);
    pub const BISHOP: Piece = Self(2);
    pub const LANCE: Piece = Self(3);
    pub const KNIGHT: Piece = Self(4);
    pub const SILVER: Piece = Self(5);
    pub const KING: Piece = Self(6);
    pub const GOLD: Piece = Self(7);
    pub const PROMO_PAWN: Piece = Self(8);
    pub const PROMO_ROOK: Piece = Self(9);
    pub const PROMO_BISHOP: Piece = Self(10);
    pub const PROMO_LANCE: Piece = Self(11);
    pub const PROMO_KNIGHT: Piece = Self(12);
    pub const PROMO_SILVER: Piece = Self(13);
    pub const NONE: Piece = Self(14);

    #[must_use]
    pub const fn side(&self) -> u8 {
        self.0 >> 4
    }

    #[must_use]
    pub const fn piece(&self) -> u8 {
        self.0 & 0b01111
    }

    #[must_use]
    pub fn new_unchecked(piece: u8, color: u8) -> Self {
        Self((color << 4) | piece)
    }

    pub fn promote(&mut self) {
        self.0 += 8;
    }
}

// this is for chess, redo later
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c: &str = match self {
            &Self::PAWN => "p",
            &Self::ROOK => "r",
            &Self::BISHOP => "b",
            &Self::LANCE => "l",
            &Self::KNIGHT => "n",
            &Self::SILVER => "s",
            &Self::GOLD => "g",
            &Self::KING => "k",
            &Self::PROMO_PAWN => "+p",
            &Self::PROMO_ROOK => "+r",
            &Self::PROMO_BISHOP => "+b",
            &Self::PROMO_LANCE => "+l",
            &Self::PROMO_KNIGHT => "+n",
            &Self::PROMO_SILVER => "+p",
            &Self::NONE => " ",
            _ => panic!("invalid piece"),
        };
        let output: String = if self.side() == 0 {
            c.to_ascii_uppercase()
        } else {
            c.to_owned()
        };
        write!(f, "{output}")
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self::NONE
    }
}
