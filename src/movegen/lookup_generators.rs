use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

fn main() {
    println!("u d l r ur dr ul dl");
    let offsets = [9, -9, -1, 1, 10, -8, 8, -10];
    let prevention_masks = [
        Bitboard(0),
        Bitboard(0),
        Bitboard::from_file(8),
        Bitboard::from_file(0),
        Bitboard::from_file(0),
        Bitboard::from_file(0),
        Bitboard::from_file(8),
        Bitboard::from_file(8),
    ];
    let _edges = Bitboard::from_rank(0)
        | Bitboard::from_rank(8)
        | Bitboard::from_file(0)
        | Bitboard::from_file(8);

    // king attacks
    print!("pub const KING_ATTACKS: [u128; 81] = [");
    for king in 0..81 {
        let mut board: Bitboard = Bitboard(0);
        for i in 0..8 {
            if king + offsets[i] >= 0 && king + offsets[i] < 81 {
                board |= Bitboard::from_idx(king + offsets[i]) & !prevention_masks[i];
            }
        }
        print!("{}, ", board.raw());
    }
    println!("];");

    // knight (up 2 left one or up 2 right one, needs to be divided by stm)
    print!("pub const KNIGHT_ATTACKS: [[u128; 81]; 2] = [");
    // sente
    print!("[");
    for sq in 0..81 {
        let mut mask = Bitboard::EMPTY;
        if sq < 63 {
            let idx = sq + 9;
            for idxdir in [4, 6] {
                if idx + offsets[idxdir] >= 0 && idx + offsets[idxdir] < 81 {
                    mask |= Bitboard::from_idx(idx + offsets[idxdir]) & !prevention_masks[idxdir];
                }
            }
        }
        //println!("mask: \n{}\n", mask);
        print!("{}, ", mask.raw());
    }
    print!("], ");
    print!("[");
    // gote
    for sq in 0..81 {
        let mut mask = Bitboard::EMPTY;
        if sq > 17 {
            let idx = sq - 9;
            for idxdir in [5, 7] {
                if idx + offsets[idxdir] >= 0 && idx + offsets[idxdir] < 81 {
                    mask |= Bitboard::from_idx(idx + offsets[idxdir]) & !prevention_masks[idxdir];
                }
            }
        }
        //println!("mask: \n{}\n", mask);
        print!("{}, ", mask.raw());
    }
    println!("]];");

    // silver (up, and the diagonals, needs to be divided by stm)
    print!("pub const SILVER_ATTACKS: [[u128; 81]; 2] = [");
    // sente
    print!("[");
    for sq in 0..81 {
        let mut mask = Bitboard::EMPTY;
        for dir in [0, 4, 5, 6, 7] {
            let idx = sq + offsets[dir];
            if idx >= 0 && idx < 81 {
                mask |= Bitboard::from_idx(idx) & !prevention_masks[dir];
            }
        }
        //println!("{}\n", mask);
        print!("{}, ", mask.raw());
    }
    print!("], ");
    // gote
    print!("[");
    for sq in 0..81 {
        let mut mask = Bitboard::EMPTY;
        for dir in [1, 4, 5, 6, 7] {
            let idx = sq + offsets[dir];
            if idx >= 0 && idx < 81 {
                mask |= Bitboard::from_idx(idx) & !prevention_masks[dir];
            }
        }
        //println!("\n{}\n", mask);
        print!("{}, ", mask.raw());
    }
    println!("]];");

    // gold / promos, (up diagonals and orthogonals, needs to be divided by stm)
    print!("pub const GOLD_ATTACKS: [[u128; 81]; 2] = [");
    // sente
    print!("[");
    for sq in 0..81 {
        let mut mask = Bitboard::EMPTY;
        for dir in [0, 1, 2, 3, 4, 6] {
            let idx = sq + offsets[dir];
            if idx >= 0 && idx < 81 {
                mask |= Bitboard::from_idx(idx) & !prevention_masks[dir];
            }
        }
        //println!("\n{}\n", mask);
        print!("{}, ", mask.raw());
    }
    print!("], ");
    // gote
    print!("[");
    for sq in 0..81 {
        let mut mask = Bitboard::EMPTY;
        for dir in [0, 1, 2, 3, 5, 7] {
            let idx = sq + offsets[dir];
            if idx >= 0 && idx < 81 {
                mask |= Bitboard::from_idx(idx) & !prevention_masks[dir];
            }
        }
        //println!("\n{}\n", mask);
        print!("{}, ", mask.raw());
    }
    println!("]];");

    // rays (all directions)
    print!("pub const RAYS: [[u128; 81]; 8] = [");
    for dir in 0..8 {
        print!("[");
        for sq in 0..81 {
            let mut ray = Bitboard::EMPTY;
            let mut current_sq = sq;
            let mut last_sq = sq;
            loop {
                let next_sq = current_sq + offsets[dir];
                if next_sq < 0 || next_sq >= 81 {
                    break;
                }
                let current_file = current_sq % 9;
                let next_file = next_sq % 9;
                let file_diff = (next_file as i32 - current_file as i32).abs();
                if (dir >= 2) && file_diff > 1 {
                    break;
                }
                ray |= Bitboard::from_idx(next_sq);
                last_sq = next_sq;
                current_sq = next_sq;
            }
            
            print!("{}, ", ray.raw());
        }
        if dir < 7 {
            print!("], ");
        } else {
            print!("]");
        }
    }
    println!("];");
    
    print!("pub const BLOCKER_MASKS: [[u128; 81]; 4] = [");
    // up (dir 0)
    print!("[");
    for sq in 0..81 {
        let mut ray = Bitboard::EMPTY;
        let mut current_sq = sq;
        let mut last_sq = sq;
        loop {
            let next_sq = current_sq + offsets[0];
            if next_sq < 0 || next_sq >= 81 {
                break;
            }
            last_sq = next_sq;
            current_sq = next_sq;
            ray |= Bitboard::from_idx(next_sq);
        }
        if last_sq != sq {
            ray ^= Bitboard::from_idx(last_sq);
        }
        print!("{}, ", ray.raw());
    }
    print!("], ");

    print!("[");
    for sq in 0..81 {
        let mut ray = Bitboard::EMPTY;
        let mut current_sq = sq;
        let mut last_sq = sq;
        loop {
            let next_sq = current_sq + offsets[1];
            if next_sq < 0 || next_sq >= 81 {
                break;
            }
            last_sq = next_sq;
            current_sq = next_sq;
            ray |= Bitboard::from_idx(next_sq);
        }
        if last_sq != sq {
            ray ^= Bitboard::from_idx(last_sq);
        }
        print!("{}, ", ray.raw());
    }
    print!("], ");
    
    // diagonal (combining dirs 4,5,6,7)
    print!("[");
    for sq in 0..81 {
        let mut combined_ray = Bitboard::EMPTY;

        for dir in [4, 5, 6, 7] {
            let mut ray = Bitboard::EMPTY;
            let mut current_sq = sq;
            let mut last_sq = sq;
            
            loop {
                let next_sq = current_sq + offsets[dir];
                if next_sq < 0 || next_sq >= 81 {
                    break;
                }
                let current_file = current_sq % 9;
                let next_file = next_sq % 9;
                let file_diff = (next_file as i32 - current_file as i32).abs();
                if file_diff > 1 {
                    break;
                }
                last_sq = next_sq;
                current_sq = next_sq;
                ray |= Bitboard::from_idx(next_sq);
            }
            if last_sq != sq {
                ray ^= Bitboard::from_idx(last_sq);
            }
            combined_ray |= ray;
        }
        print!("{}, ", combined_ray.raw());
    }
    print!("], ");
    
    // orthogonal (combining dirs 0,1,2,3)
    print!("[");
    for sq in 0..81 {
        let mut combined_ray = Bitboard::EMPTY;
        for dir in [0, 1, 2, 3] {
            let mut ray = Bitboard::EMPTY;
            let mut current_sq = sq;
            let mut last_sq = sq;
            loop {
                let next_sq = current_sq + offsets[dir];
                if next_sq < 0 || next_sq >= 81 {
                    break;
                }
                if dir >= 2 {
                    let current_file = current_sq % 9;
                    let next_file = next_sq % 9;
                    let file_diff = (next_file as i32 - current_file as i32).abs();
                    if file_diff > 1 {
                        break;
                    }
                }
                last_sq = next_sq;
                current_sq = next_sq;
                ray |= Bitboard::from_idx(next_sq);
            }
            if last_sq != sq {
                ray ^= Bitboard::from_idx(last_sq);
            }
            combined_ray |= ray;
        }
        print!("{}, ", combined_ray.raw());
    }
    print!("]");
    println!("];");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(u128);

// a mask for a single file on the board
pub const FILEMASK: u128 =
    0b1_000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001;
// a mask for a single rank on the board
pub const RANKMASK: u128 = 0b111111111;

impl Bitboard {
    pub const EMPTY: Self = Self(0);

    pub const fn from_idx(idx: i32) -> Self {
        Self(1 << idx as i128)
    }

    #[must_use]
    pub const fn from_rank(rank: u8) -> Self {
        Self(RANKMASK << (9 * rank))
    }

    #[must_use]
    pub const fn from_file(file: u8) -> Self {
        Self(FILEMASK << file)
    }

    #[must_use]
    pub const fn popcount(&self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn has_bits(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn raw(&self) -> u128 {
        self.0
    }

    #[must_use]
    pub const fn contains_multiple(self) -> bool {
        (self.0 & self.0.wrapping_sub(1)) != 0
    }

    #[must_use]
    pub const fn contains_one(self) -> bool {
        !self.is_empty() && !self.contains_multiple()
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    } 
}

// i gotta add this to anura and ctenophore fr fr
impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0;
        let mut res = "".to_owned();
        for rank in (0..9).rev() {
            for file in 0..9 {
                let idx = rank * 9 + file;
                if value & (1 << idx) != 0 {
                    res += "1";
                } else {
                    res += "0";
                }
            }
            res += "\n";
        }
        write!(f, "{}", res)
    }
}
