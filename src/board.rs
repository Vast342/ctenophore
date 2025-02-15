use crate::{
    movegen::{
        get_bishop_attacks, get_gold_attacks, get_king_attacks, get_knight_attacks,
        get_lance_attacks, get_rook_attacks, get_silver_attacks, setwise_pawns,
    },
    types::{
        action::{Action, Actionlist},
        bitboard::Bitboard,
        hand::Hand,
        piece::{Piece, NUM_PIECE_TYPES},
        square::{Square, NUM_SQUARES},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    sides: [Bitboard; 2],
    pieces: [Bitboard; NUM_PIECE_TYPES as usize],
    mailbox: [Piece; NUM_SQUARES as usize],
    hands: [Hand; 2],
    checkers: Bitboard,
    diag_pin_mask: Bitboard,
    orth_pin_mask: Bitboard,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            sides: [Bitboard::default(); 2],
            pieces: [Bitboard::default(); NUM_PIECE_TYPES as usize],
            mailbox: [Piece::default(); NUM_SQUARES as usize],
            hands: [Hand::default(); 2],
            checkers: Bitboard::EMPTY,
            diag_pin_mask: Bitboard::EMPTY,
            orth_pin_mask: Bitboard::EMPTY,
        }
    }
}

impl Position {
    pub fn add_piece(&mut self, sq: Square, piece: Piece) {
        let bitboard_square: Bitboard = Bitboard::from_square(sq);
        self.sides[piece.side() as usize] ^= bitboard_square;
        self.pieces[piece.piece().as_usize()] ^= bitboard_square;
        self.mailbox[sq.as_usize()] = piece;
    }

    pub fn remove_piece(&mut self, sq: Square, piece: Piece) {
        let bitboard_square: Bitboard = Bitboard::from_square(sq);
        self.sides[piece.side() as usize] ^= bitboard_square;
        self.pieces[piece.piece().as_usize()] ^= bitboard_square;
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

    fn get_gold_movers(&self, stm: u8) -> Bitboard {
        self.sided_piece(Piece::GOLD.raw(), stm)
            | self.sided_piece(Piece::PROMO_PAWN.raw(), stm)
            | self.sided_piece(Piece::PROMO_LANCE.raw(), stm)
            | self.sided_piece(Piece::PROMO_KNIGHT.raw(), stm)
            | self.sided_piece(Piece::PROMO_SILVER.raw(), stm)
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
                        state.hands[1].set(Piece::PAWN, count);
                        count = 1;
                    }
                    'P' => {
                        state.hands[0].set(Piece::PAWN, count);
                        count = 1;
                    }
                    'l' => {
                        state.hands[1].set(Piece::LANCE, count);
                        count = 1;
                    }
                    'L' => {
                        state.hands[0].set(Piece::LANCE, count);
                        count = 1;
                    }
                    'n' => {
                        state.hands[1].set(Piece::KNIGHT, count);
                        count = 1;
                    }
                    'N' => {
                        state.hands[0].set(Piece::KNIGHT, count);
                        count = 1;
                    }
                    's' => {
                        state.hands[1].set(Piece::SILVER, count);
                        count = 1;
                    }
                    'S' => {
                        state.hands[0].set(Piece::SILVER, count);
                        count = 1;
                    }
                    'g' => {
                        state.hands[1].set(Piece::GOLD, count);
                        count = 1;
                    }
                    'G' => {
                        state.hands[0].set(Piece::GOLD, count);
                        count = 1;
                    }
                    'b' => {
                        state.hands[1].set(Piece::BISHOP, count);
                        count = 1;
                    }
                    'B' => {
                        state.hands[0].set(Piece::BISHOP, count);
                        count = 1;
                    }
                    'r' => {
                        state.hands[1].set(Piece::ROOK, count);
                        count = 1;
                    }
                    'R' => {
                        state.hands[0].set(Piece::ROOK, count);
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
        self.update_checkers();
    }
    fn is_promotable(&self, piece: Piece, sq: Square, bit: Square) -> bool {
        piece < Piece::GOLD && (self.is_in_promotion_zone(sq) || self.is_in_promotion_zone(bit))
    }

    fn is_in_promotion_zone(&self, sq: Square) -> bool {
        match self.stm {
            0 => sq >= Square(54), // Player 0's promotion zone
            1 => sq < Square(27),  // Player 1's promotion zone
            _ => false,
        }
    }

    fn is_out_of_bounds(&self, piece: Piece, bit: Square) -> bool {
        (piece == Piece::LANCE && self.is_out_of_lance_bounds(bit))
            || (piece == Piece::KNIGHT && self.is_out_of_knight_bounds(bit))
    }

    fn is_out_of_lance_bounds(&self, bit: Square) -> bool {
        match self.stm {
            0 => bit >= Square(72),
            1 => bit < Square(9),
            _ => false,
        }
    }

    fn is_out_of_knight_bounds(&self, bit: Square) -> bool {
        match self.stm {
            0 => bit >= Square(63),
            1 => bit < Square(18),
            _ => false,
        }
    }

    pub fn get_actions(&self) -> Actionlist {
        let state = self.current_state();
        let mut actions = Actionlist::default();
        let occ = state.occupied();
        let us = state.sides[self.stm as usize];

        for sq in us {
            let piece = state.piece_on_square(sq);
            let mut attacks = match piece.piece() {
                Piece::PAWN => Bitboard::EMPTY,
                Piece::LANCE => get_lance_attacks(sq, occ, self.stm),
                Piece::KNIGHT => get_knight_attacks(sq, self.stm),
                Piece::SILVER => get_silver_attacks(sq, self.stm),
                Piece::BISHOP => get_bishop_attacks(sq, occ),
                Piece::ROOK => get_rook_attacks(sq, occ),
                Piece::GOLD
                | Piece::PROMO_PAWN
                | Piece::PROMO_LANCE
                | Piece::PROMO_KNIGHT
                | Piece::PROMO_SILVER => get_gold_attacks(sq, self.stm),
                Piece::KING => get_king_attacks(sq),
                Piece::PROMO_BISHOP => get_bishop_attacks(sq, occ) | get_king_attacks(sq),
                Piece::PROMO_ROOK => get_rook_attacks(sq, occ) | get_king_attacks(sq),
                _ => panic!("invalid piece"),
            };

            // no taking our own pieces
            attacks &= !us;

            // parse to actions
            for bit in attacks {
                if self.is_promotable(piece.piece(), sq, bit) {
                    actions.push(Action::new_move(sq, bit, true));
                }
                if !self.is_out_of_bounds(piece.piece(), bit) {
                    actions.push(Action::new_move(sq, bit, false));
                }
            }
        }

        // setwise pawns
        let our_pawns = state.sided_piece(Piece::PAWN.as_usize() as u8, self.stm);
        let mut pawn_attacks = setwise_pawns(our_pawns, self.stm);

        // no taking our own pieces
        pawn_attacks &= !us;

        // parse to actions
        for bit in pawn_attacks {
            let og = Square((bit.as_u16() as i16 + if self.stm == 0 { -9 } else { 9 }) as u8);
            if (self.stm == 0 && bit >= Square(54)) || (self.stm == 1 && bit < Square(27)) {
                actions.push(Action::new_move(og, bit, true));
            }
            // don't generate pawn promos if it's last row
            if !((self.stm == 0 && bit >= Square(72)) || (self.stm == 1 && bit < Square(9))) {
                actions.push(Action::new_move(og, bit, false));
            }
        }

        // drops
        let hand = state.hands[self.stm as usize];
        let empty = !occ & Bitboard::FULL;
        for (piece, _count) in hand {
            let open_squares = if piece.piece() == Piece::PAWN {
                // no back ranks, no overlapping files, no checkmates (not handled yet)
                let free_files = !our_pawns.file_fill() & Bitboard::FULL;
                let free_squares = if self.stm == 0 {
                    free_files >> 9
                } else {
                    free_files << 9
                };
                empty & free_squares
            } else if piece.piece() == Piece::KNIGHT {
                // no back 2 ranks
                let free_squares = if self.stm == 0 {
                    Bitboard::FULL >> 18
                } else {
                    Bitboard::FULL << 18
                };
                empty & free_squares
            } else if piece.piece() == Piece::LANCE {
                // no back ranks
                let free_squares = if self.stm == 0 {
                    Bitboard::FULL >> 9
                } else {
                    Bitboard::FULL << 9
                };
                empty & free_squares
            } else {
                empty
            };

            for sq in open_squares {
                actions.push(Action::new_drop(piece.as_stm(self.stm), sq));
            }
        }

        actions
    }
    pub fn piece_on_square(&self, sq: Square) -> Piece {
        self.current_state().piece_on_square(sq)
    }

    pub fn get_attackers(&self, sq: Square) -> Bitboard {
        let opps = 1 - self.stm;
        let state = self.current_state();
        let occ = state.occupied();

        let pawn_atk_bb = Bitboard::from_square(Square(
            (sq.as_u16() as i32 + if self.stm == 0 { 9 } else { -9 }) as u8,
        ));

        let gold_movers = state.get_gold_movers(opps);
        let bishopy_movers = state.sided_piece(Piece::BISHOP.raw(), opps)
            | state.sided_piece(Piece::PROMO_BISHOP.raw(), opps);
        let rooky_movers = state.sided_piece(Piece::ROOK.raw(), opps)
            | state.sided_piece(Piece::PROMO_ROOK.raw(), opps);
        let kingy_movers = state.sided_piece(Piece::KING.raw(), opps)
            | state.sided_piece(Piece::PROMO_BISHOP.raw(), opps)
            | state.sided_piece(Piece::PROMO_ROOK.raw(), opps);

        (pawn_atk_bb & state.sided_piece(Piece::PAWN.raw(), opps))
            | (get_lance_attacks(sq, occ, self.stm) & state.sided_piece(Piece::LANCE.raw(), opps))
            | (get_knight_attacks(sq, self.stm) & state.sided_piece(Piece::KNIGHT.raw(), opps))
            | (get_silver_attacks(sq, self.stm) & state.sided_piece(Piece::SILVER.raw(), opps))
            | (get_bishop_attacks(sq, occ) & bishopy_movers)
            | (get_rook_attacks(sq, occ) & rooky_movers)
            | (get_king_attacks(sq) & kingy_movers)
            | (get_gold_attacks(sq, self.stm) & gold_movers)
    }

    pub fn in_check(&self) -> bool {
        !self.current_state().checkers.is_empty()
    }

    pub fn king_sq(&self) -> Square {
        let state = self.current_state();
        let our_king = state.sided_piece(Piece::KING.raw(), self.stm);
        Square(our_king.lsb())
    }

    pub fn update_checkers(&mut self) {
        let king_sq = self.king_sq();
        let king_atkers = self.get_attackers(king_sq);
        let state = self.current_state_mut();
        state.checkers = king_atkers;
    }

    pub fn square_attacked(&self, sq: Square, occ: Bitboard) -> bool {
        let opp = 1 - self.stm;
        let state = self.current_state();

        let opp_bishopy = state.sided_piece(Piece::BISHOP.raw(), opp)
            | state.sided_piece(Piece::PROMO_BISHOP.raw(), opp);
        let opp_rooky = state.sided_piece(Piece::ROOK.raw(), opp)
            | state.sided_piece(Piece::PROMO_ROOK.raw(), opp);
        let opp_kingy = state.sided_piece(Piece::KING.raw(), opp)
            | state.sided_piece(Piece::PROMO_BISHOP.raw(), opp)
            | state.sided_piece(Piece::PROMO_ROOK.raw(), opp);
        let gold_movers = state.get_gold_movers(opp);

        let pawn_atk_bb = Bitboard::from_square(Square(
            (sq.as_u16() as i32 + if self.stm == 0 { 9 } else { -9 }) as u8,
        ));
        if (pawn_atk_bb & state.sided_piece(Piece::PAWN.raw(), opp)).is_not_empty() {
            return true;
        }

        if (get_gold_attacks(sq, self.stm) & gold_movers).is_not_empty() {
            return true;
        }

        if (get_silver_attacks(sq, self.stm) & state.sided_piece(Piece::SILVER.raw(), opp))
            .is_not_empty()
        {
            return true;
        }

        if (get_knight_attacks(sq, self.stm) & state.sided_piece(Piece::KNIGHT.raw(), opp))
            .is_not_empty()
        {
            return true;
        }

        if (get_king_attacks(sq) & opp_kingy).is_not_empty() {
            return true;
        }

        if (get_lance_attacks(sq, occ, self.stm) & state.sided_piece(Piece::LANCE.raw(), opp))
            .is_not_empty()
        {
            return true;
        }

        if (get_bishop_attacks(sq, occ) & opp_bishopy).is_not_empty() {
            return true;
        }

        if (get_rook_attacks(sq, occ) & opp_rooky).is_not_empty() {
            return true;
        }

        false
    }

    pub fn update_pins_and_checkers(&mut self) {}

    pub fn perform_action(&mut self, action: Action) {
        self.states.push(*self.current_state());
        // just like in anura, not using self.current_state_mut() because of borrowing shenanigans
        let state = self.states.last_mut().expect("no position");
        if action.is_drop() {
            let to = action.to();
            let piece = action.piece();
            state.add_piece(to, piece);
            state.hands[self.stm as usize].dec(piece.unpromote());
        } else {
            let from = action.from();
            let to = action.to();
            let piece = state.piece_on_square(from);
            let victim = state.piece_on_square(to);
            state.remove_piece(from, piece);
            if victim != Piece::NONE {
                state.remove_piece(to, victim);
                state.hands[self.stm as usize].inc(victim.unpromote())
            }
            if action.is_promo() {
                state.add_piece(to, piece.promote());
            } else {
                state.add_piece(to, piece);
            }
        }

        self.ply += 1;
        self.stm = 1 - self.stm;
    }

    pub fn undo_action(&mut self) {
        self.states.pop();
        self.ply -= 1;
        self.stm = 1 - self.stm;
    }
}
