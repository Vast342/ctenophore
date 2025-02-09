use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square(pub u8);

pub const NUM_SQUARES: u8 = 81;
pub const BOARD_LEN: u8 = 9;

impl Square {
    pub const INVALID: Self = Self(NUM_SQUARES);

    #[must_use]
    pub const fn rank(&self) -> u8 /* i refuse to write a rank wrapper */ {
        self.0 / BOARD_LEN
    }

    #[must_use]
    pub const fn file(&self) -> u8 /* i refuse to write a file wrapper */ {
        self.0 % BOARD_LEN
    }

    #[must_use]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub const fn as_u16(&self) -> u16 {
        self.0 as u16
    }

    #[must_use]
    pub const fn from_rf(rank: u8, file: u8) -> Self {
        Self(rank * BOARD_LEN + file)
    }
}

impl AddAssign for Square {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
