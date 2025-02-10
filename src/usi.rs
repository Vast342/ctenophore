use std::io;

use crate::{
    board::Board,
    perft::{perft, split_perft},
};

#[derive(Default)]
pub struct UsiManager {
    board: Board,
}

impl UsiManager {
    pub fn get_command(&mut self) -> bool {
        let mut buffer = String::new();

        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let (command, _new_line) = buffer.split_at(buffer.len() - 1);
        self.interpret_command(command)
    }
    fn interpret_command(&mut self, command_msg: &str) -> bool {
        let mut command_split = command_msg.split_ascii_whitespace();
        let Some(command) = command_split.next() else {
            return false;
        };

        match command {
            "perft" => perft(
                &mut self.board,
                command_split
                    .next()
                    .expect("No Depth")
                    .parse()
                    .expect("Invalid Depth"),
            ),
            "splitperft" => split_perft(
                &mut self.board,
                command_split
                    .next()
                    .expect("No Depth")
                    .parse()
                    .expect("Invalid Depth"),
            ),
            "position" => self.position(command_msg),
            "print" => self.board.print_state(),
            "quit" => return false,
            "makemove" => self.make_move(command_msg),
            _ => println!("Invalid Command: {}", command),
        }
        true
    }
    fn position(&mut self, command_msg: &str) {
        let mut command_split = command_msg.split_ascii_whitespace();
        let _first_token = command_split.next().expect("not enough tokens");
        let second_token = command_split.next().expect("not enough tokens");
        let mut fen: String;
        if second_token == "startpos" {
            fen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1".to_string();
        } else {
            let third_token = command_split.next().expect("not enough tokens");
            fen = third_token.to_owned()
                + " "
                + command_split.next().expect("not enough tokens")
                + " "
                + command_split.next().expect("not enough tokens");
            let next_token = command_split.next();
            if let Some(string) = next_token {
                if string != "moves" {
                    fen += string;
                }
            }
        }
        self.board = Board::default();
        self.board.load_fen(&fen);
    }
    fn make_move(&mut self, command_msg: &str) {
        let mut command_split = command_msg.split_ascii_whitespace();
        let _first_token = command_split.next().expect("not enough tokens");
        let second_token = command_split.next().expect("not enough tokens");
        let index: usize = second_token.parse::<usize>().expect("invalid index");
        let list = self.board.get_actions();
        self.board.perform_action(list[index]);
    }
}
