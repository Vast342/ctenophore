use core::fmt;

use super::{piece::Piece, square::Square};
use arrayvec::ArrayVec;

pub type Actionlist = ArrayVec<Action, 600>;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Action(pub u16);

const PROMO_MASK: u16 = 0b10000000_00000000;
const DROP_MASK: u16 = 0b01000000_00000000;

const FROM_OFFSET: u16 = 7;
const DROP_OFFSET: u16 = 14;
const PROMO_OFFSET: u16 = 15;
const SQUARE_MASK: u16 = 0b1111111;

impl Action {
    pub fn new_move(from: Square, to: Square, is_promo: bool) -> Self {
        Self(to.as_u16() | (from.as_u16() << FROM_OFFSET) | ((is_promo as u16) << PROMO_OFFSET))
    }

    pub fn new_drop(piece: Piece, to: Square) -> Self {
        Self(to.as_u16() | ((piece.raw() as u16) << FROM_OFFSET) | (1 << DROP_OFFSET))
    }

    pub const fn to(&self) -> Square {
        Square((self.0 & SQUARE_MASK) as u8)
    }

    pub const fn from(&self) -> Square {
        debug_assert!(!self.is_drop());
        Square(((self.0 >> FROM_OFFSET) & SQUARE_MASK) as u8)
    }

    pub const fn piece(&self) -> Piece {
        Piece((self.0 >> FROM_OFFSET) as u8)
    }

    pub const fn is_promo(&self) -> bool {
        (self.0 & PROMO_MASK) != 0
    }

    pub const fn is_drop(&self) -> bool {
        (self.0 & DROP_MASK) != 0
    }

    pub fn to_usi(&self) -> String {
        let to_square = self.to();
        let to_file = 9 - to_square.file();
        let to_rank = (b'i' - to_square.rank()) as char;

        if self.is_drop() {
            format!("{}*{}{}", self.piece(), to_file, to_rank)
        } else {
            let from_square = self.from();
            let from_file = 9 - from_square.file();
            let from_rank = (b'i' - from_square.rank()) as char;
            if self.is_promo() {
                format!("{}{}{}{}+", from_file, from_rank, to_file, to_rank)
            } else {
                format!("{}{}{}{}", from_file, from_rank, to_file, to_rank)
            }
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_square = self.to();
        let to_file = 9 - to_square.file();
        let to_rank = (b'i' - to_square.rank()) as char;

        let output = if self.is_drop() {
            format!("{}*{}{}", self.piece(), to_file, to_rank)
        } else {
            let from_square = self.from();
            let from_file = 9 - from_square.file();
            let from_rank = (b'i' - from_square.rank()) as char;
            if self.is_promo() {
                format!("{}{}{}{}+", from_file, from_rank, to_file, to_rank)
            } else {
                format!("{}{}{}{}", from_file, from_rank, to_file, to_rank)
            }
        };

        write!(f, "{output}")
    }
}