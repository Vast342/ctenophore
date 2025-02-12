use std::env;

use usi::UsiManager;

pub mod board;
pub mod movegen;
pub mod perft;
pub mod types;
pub mod usi;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let mut manager = UsiManager::default();
    loop {
        if !manager.get_command() {
            break;
        }
    }
}
