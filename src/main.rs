use board::Board;
use std::env;

pub mod board;
pub mod types;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut board = Board::default();
    board
        .load_fen("8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1PS6/1KSG3+r1/LN2+p3L w Sbgn3p 124");
    board.print_state();
}
