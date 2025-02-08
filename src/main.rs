use board::Board;
use movegen::slideys::{get_bishop_attacks, get_lance_attacks, get_rook_attacks};
use std::env;
use types::{bitboard::Bitboard, square::Square};

pub mod board;
pub mod movegen;
pub mod types;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut board = Board::default();
    board
        .load_fen("8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1PS6/1KSG3+r1/LN2+p3L w Sbgn3p 124");
    board.print_state();
    let occ = Bitboard(1209284252916221216365843);
    let r_atk = get_rook_attacks(Square(16), occ);
    println!("{}", r_atk);
    let b_atk = get_bishop_attacks(Square(58), occ);
    println!("{}", b_atk);
    let l_atk = get_lance_attacks(Square(8), occ, 0);
    println!("{}", l_atk);
}
