use board::Board;

pub mod board;
pub mod types;

fn main() {
    let board = Board::default();
    board.print_state();
}
