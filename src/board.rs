use crate::types::{
    bitboard::Bitboard,
    hand::Hand,
    piece::{Piece, NUM_PIECE_TYPES},
    square::{Square, NUM_SQUARES},
};

#[derive(Debug, Clone)]
pub struct Position {
    sides: [Bitboard; 2],
    pieces: [Bitboard; NUM_PIECE_TYPES as usize],
    mailbox: [Piece; NUM_SQUARES as usize],
    hands: [Hand; 2],
}

impl Default for Position {
    fn default() -> Self {
        Self {
            sides: [Bitboard::default(); 2],
            pieces: [Bitboard::default(); NUM_PIECE_TYPES as usize],
            mailbox: [Piece::default(); NUM_SQUARES as usize],
            hands: [Hand::default(); 2],
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
    states: Vec<Position>,
    stm: u8,
    ply: i16,
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

    #[allow(dead_code)]
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

        println!();

        println!("stm: {}", if self.stm == 0 { "sente" } else { "gote" });
        println!("sente hand: {}", state.hands[0]);
        println!(
            "gote hand: {}",
            state.hands[1].to_string().to_ascii_lowercase()
        );
        println!("ply count: {}", self.ply);
    }

    pub fn load_fen(&mut self, fen: &str) {
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
                            Piece::new_unchecked(
                                Piece::PAWN.raw() + (8 * is_promoted as u8),
                                Piece::GOTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'P' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::PAWN.raw() + (8 * is_promoted as u8),
                                Piece::SENTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'l' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::LANCE.raw() + (8 * is_promoted as u8),
                                Piece::GOTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'L' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::LANCE.raw() + (8 * is_promoted as u8),
                                Piece::SENTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'n' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::KNIGHT.raw() + (8 * is_promoted as u8),
                                Piece::GOTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'N' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::KNIGHT.raw() + (8 * is_promoted as u8),
                                Piece::SENTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    's' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::SILVER.raw() + (8 * is_promoted as u8),
                                Piece::GOTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'S' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::SILVER.raw() + (8 * is_promoted as u8),
                                Piece::SENTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'g' => {
                        debug_assert!(!is_promoted);
                        state.add_piece(
                            i,
                            Piece::new_unchecked(Piece::GOLD.raw(), Piece::GOTE.raw()),
                        );
                        i += Square(1);
                    }
                    'G' => {
                        debug_assert!(!is_promoted);
                        state.add_piece(
                            i,
                            Piece::new_unchecked(Piece::GOLD.raw(), Piece::SENTE.raw()),
                        );
                        i += Square(1);
                    }
                    'b' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::BISHOP.raw() + (8 * is_promoted as u8),
                                Piece::GOTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'B' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::BISHOP.raw() + (8 * is_promoted as u8),
                                Piece::SENTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'r' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::ROOK.raw() + (8 * is_promoted as u8),
                                Piece::GOTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'R' => {
                        state.add_piece(
                            i,
                            Piece::new_unchecked(
                                Piece::ROOK.raw() + (8 * is_promoted as u8),
                                Piece::SENTE.raw(),
                            ),
                        );
                        is_promoted = false;
                        i += Square(1);
                    }
                    'k' => {
                        debug_assert!(!is_promoted);
                        state.add_piece(
                            i,
                            Piece::new_unchecked(Piece::KING.raw(), Piece::GOTE.raw()),
                        );
                        i += Square(1);
                    }
                    'K' => {
                        debug_assert!(!is_promoted);
                        state.add_piece(
                            i,
                            Piece::new_unchecked(Piece::KING.raw(), Piece::SENTE.raw()),
                        );
                        i += Square(1);
                    }
                    _ => {
                        i += Square(
                            c.to_digit(10)
                                .unwrap_or_else(|| panic!("invalid character in fen: {c}"))
                                as u8,
                        )
                    }
                }
            }
        }

        // second token: stm
        token = fen_segments.next().expect("no ctm?");
        self.stm = u8::from(token == "w");

        // third token: hand
        token = fen_segments.next().expect("no hand");
        if token != "-" {
            let mut count = 1;
            for c in token.chars() {
                match c {
                    'p' => {
                        state.hands[1].set(
                            Piece::new_unchecked(Piece::PAWN.raw(), Piece::GOTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'P' => {
                        state.hands[0].set(
                            Piece::new_unchecked(Piece::PAWN.raw(), Piece::SENTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'l' => {
                        state.hands[1].set(
                            Piece::new_unchecked(Piece::LANCE.raw(), Piece::GOTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'L' => {
                        state.hands[0].set(
                            Piece::new_unchecked(Piece::LANCE.raw(), Piece::SENTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'n' => {
                        state.hands[1].set(
                            Piece::new_unchecked(Piece::KNIGHT.raw(), Piece::GOTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'N' => {
                        state.hands[0].set(
                            Piece::new_unchecked(Piece::KNIGHT.raw(), Piece::SENTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    's' => {
                        state.hands[1].set(
                            Piece::new_unchecked(Piece::SILVER.raw(), Piece::GOTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'S' => {
                        state.hands[0].set(
                            Piece::new_unchecked(Piece::SILVER.raw(), Piece::SENTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'g' => {
                        state.hands[1].set(
                            Piece::new_unchecked(Piece::GOLD.raw(), Piece::GOTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'G' => {
                        state.hands[0].set(
                            Piece::new_unchecked(Piece::GOLD.raw(), Piece::SENTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'b' => {
                        state.hands[1].set(
                            Piece::new_unchecked(Piece::BISHOP.raw(), Piece::GOTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'B' => {
                        state.hands[0].set(
                            Piece::new_unchecked(Piece::BISHOP.raw(), Piece::SENTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'r' => {
                        state.hands[1].set(
                            Piece::new_unchecked(Piece::ROOK.raw(), Piece::GOTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'R' => {
                        state.hands[0].set(
                            Piece::new_unchecked(Piece::ROOK.raw(), Piece::SENTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'k' => {
                        state.hands[1].set(
                            Piece::new_unchecked(Piece::KING.raw(), Piece::GOTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    'K' => {
                        state.hands[0].set(
                            Piece::new_unchecked(Piece::KING.raw(), Piece::SENTE.raw()),
                            count,
                        );
                        count = 1;
                    }
                    // sets the count to use for next time
                    _ => {
                        count = c
                            .to_digit(10)
                            .unwrap_or_else(|| panic!("invalid character in fen: {c}"))
                    }
                }
            }
        }

        // fourth token: move count (optional)
        let token_option = fen_segments.next();
        if token_option.is_some() {
            self.ply = token_option.unwrap().parse().unwrap();
        }

        self.states.push(state);
    }
}
