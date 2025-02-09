use core::panic;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece(pub u8);

//                            real  promoted
pub const NUM_PIECE_TYPES: u8 = 8 + 6;

impl Piece {
    pub const PAWN: Piece = Self(0);
    pub const LANCE: Piece = Self(1);
    pub const KNIGHT: Piece = Self(2);
    pub const SILVER: Piece = Self(3);
    pub const BISHOP: Piece = Self(4);
    pub const ROOK: Piece = Self(5);
    pub const GOLD: Piece = Self(6);
    pub const KING: Piece = Self(7);
    pub const PROMO_PAWN: Piece = Self(8);
    pub const PROMO_LANCE: Piece = Self(9);
    pub const PROMO_KNIGHT: Piece = Self(10);
    pub const PROMO_SILVER: Piece = Self(11);
    pub const PROMO_BISHOP: Piece = Self(12);
    pub const PROMO_ROOK: Piece = Self(13);
    pub const NONE: Piece = Self(14);

    pub const SENTE: Piece = Self(0);
    pub const GOTE: Piece = Self(1);

    #[must_use]
    pub const fn side(&self) -> u8 {
        self.0 >> 4
    }

    #[must_use]
    pub const fn piece(&self) -> Self {
        Self(self.0 & 0b01111)
    }

    #[must_use]
    pub fn new_unchecked(piece: u8, side: u8) -> Self {
        Self((side << 4) | piece)
    }

    pub fn promote(&mut self) {
        self.0 += 8;
    }

    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub const fn as_stm(&self, stm: u8) -> Self {
        Self(self.0 | (stm << 4))
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c: &str = match self.piece() {
            Self::PAWN => "p",
            Self::LANCE => "l",
            Self::KNIGHT => "n",
            Self::SILVER => "s",
            Self::GOLD => "g",
            Self::KING => "k",
            Self::BISHOP => "b",
            Self::ROOK => "r",
            Self::PROMO_PAWN => "+p",
            Self::PROMO_LANCE => "+l",
            Self::PROMO_KNIGHT => "+n",
            Self::PROMO_SILVER => "+p",
            Self::PROMO_BISHOP => "+b",
            Self::PROMO_ROOK => "+r",
            Self::NONE => " ",
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
