use crate::types::{
    bitboard::Bitboard,
    piece::{Piece, NUM_PIECE_TYPES},
    square::{Square, NUM_SQUARES},
};

#[derive(Debug, Clone)]
pub struct Position {
    sides: [Bitboard; 2],
    pieces: [Bitboard; NUM_PIECE_TYPES as usize],
    mailbox: [Piece; NUM_SQUARES as usize],
}

impl Default for Position {
    fn default() -> Self {
        Self {
            sides: [Bitboard::EMPTY; 2],
            pieces: [Bitboard::EMPTY; NUM_PIECE_TYPES as usize],
            mailbox: [Piece::NONE; NUM_SQUARES as usize],
        }
    }
}

impl Position {
    pub fn add_piece(&mut self, sq: Square, piece: Piece) {
        let bitboard_square: Bitboard = Bitboard::from_square(sq);
        self.sides[piece.side() as usize] ^= bitboard_square;
        self.pieces[piece.piece() as usize] ^= bitboard_square;
        self.mailbox[sq.as_usize()] = piece;
    }

    pub fn remove_piece(&mut self, sq: Square, piece: Piece) {
        let bitboard_square: Bitboard = Bitboard::from_square(sq);
        self.sides[piece.side() as usize] ^= bitboard_square;
        self.pieces[piece.piece() as usize] ^= bitboard_square;
        self.mailbox[sq.as_usize()] = Piece::NONE;
    }

    pub fn move_piece(&mut self, from: Square, piece: Piece, to: Square, victim: Piece) {
        if victim != Piece::NONE {
            self.remove_piece(to, victim);
        }
        self.remove_piece(from, piece);
        self.add_piece(to, piece);
    }

    #[must_use]
    pub const fn piece_on_square(&self, sq: Square) -> Piece {
        self.mailbox[sq.as_usize()]
    }

    #[must_use]
    pub fn occupied(&self) -> Bitboard {
        self.sides[0] | self.sides[1]
    }

    #[must_use]
    pub fn sided_piece(&self, piece: u8, side: u8) -> Bitboard {
        self.sides[side as usize] & self.pieces[piece as usize]
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub states: Vec<Position>,
    pub stm: u8,
    pub ply: i16,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            states: vec![Position::default(); 256],
            stm: 0,
            ply: 0,
        }
    }
}

impl Board {
    fn current_state(&self) -> &Position {
        self.states.last().expect("No current state")
    }

    fn current_state_mut(&mut self) -> &mut Position {
        self.states.last_mut().expect("No current state")
    }

    pub fn print_state(&self) {
        let state = self.current_state();

        // top line
        print!("┌");
        for _i in 0..8 {
            print!("───┬")
        }
        println!("───┐");

        for i in (0..9).rev() {
            for j in 0..9 {
                if state.mailbox[i * 9 + j].to_string().len() == 2 {
                    print!("│{} ", state.mailbox[i * 9 + j]);
                } else {
                    print!("│ {} ", state.mailbox[i * 9 + j]);
                }
            }
            println!("│");
            // line
            if i != 0 {
                print!("├");
                for _k in 0..8 {
                    print!("───┼")
                }
                println!("───┤");
            }
        }

        // bottom line
        print!("└");
        for _i in 0..8 {
            print!("───┴")
        }
        println!("───┘");
    }

    fn load_fen(&mut self, fen: &str) {
        let mut state = Position::default();

        let mut fen_segments = fen.split_ascii_whitespace();

        // first token: position
        let mut token = fen_segments.next().expect("no position?");
        let mut ranks = token.rsplit('/');
        let mut i: Square = Square(0);
        for rank in ranks.by_ref() {
            let mut is_promoted = false;
            for c in rank.chars() {
                match c {
                    '+' => {
                        // promote next piece
                        is_promoted = true;
                    }
                    'p' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(Piece::PAWN.raw() + (8 * is_promoted as u8), Piece::GOTE.raw()),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'P' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(Piece::PAWN.raw() + (8 * is_promoted as u8), Piece::SENTE.raw()),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'r' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(Piece::ROOK.raw() + (8 * is_promoted as u8), Piece::GOTE.raw()),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'R' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(Piece::ROOK.raw() + (8 * is_promoted as u8), Piece::SENTE.raw()),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    _ => i += Square(c.to_digit(10).expect("invalid character in fen") as u8),
                }
            }
        }

        // second token: stm

        // third token: hand
        // nothing here yet

        // fourth token: move count (optional)
    }
}
