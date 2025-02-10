use std::time::Instant;

use crate::board::Board;

pub fn split_perft(board: &mut Board, depth: u8) {
    let actions = board.get_actions();
    let mut count = 0;
    let start = Instant::now();
    for action in &actions {
        if board.perform_action(*action) {
            let result = perft_internal(board, depth - 1);
            println!("{} : {}", action, result);
            board.undo_action();
            count += result;
        }
    }
    println!(
        "{} nodes {} nps",
        count,
        count as f32 / start.elapsed().as_secs_f32()
    );
}

pub fn perft(board: &mut Board, depth: u8) {
    let start = Instant::now();
    let result = perft_internal(board, depth);
    println!(
        "{} nodes {} nps",
        result,
        result as f32 / start.elapsed().as_secs_f32()
    );
}

fn perft_internal(board: &mut Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let actions = board.get_actions();
    let mut count = 0;
    for action in &actions {
        if board.perform_action(*action) {
            count += perft_internal(board, depth - 1);
            board.undo_action();
        }
    }
    count
}
