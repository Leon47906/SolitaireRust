use core::{fmt, panic};
use serde::Serialize;

use crate::cards::{Card, same_card_identity};
use crate::piles::*;

// For WASM
#[derive(Serialize)]
pub struct CardView {
    pub suit: String,
    pub value: u8,
    pub face_up: bool,
}

#[derive(Serialize)]
pub struct GameState {
    pub deck_size: usize,
    pub waste: Vec<CardView>,
    pub tableau: Vec<Vec<CardView>>,
    pub foundation: Vec<Vec<CardView>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PileKind {
    Deck,
    Waste,
    Tableau(usize),
    Foundation(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    FromDeckToWaste,
    FromWasteToPile {
        to: PileKind,
    },
    FromTableauToTableau {
        from: PileKind,
        to: PileKind,
        count: usize,
    },
    FromTableauToFoundation {
        from: PileKind,
        to: PileKind,
    },
}

#[derive(Debug,Clone)]
pub enum UndoRecord {
    MoveCards {
        from: PileKind,
        to: PileKind,
        cards: Vec<Card>,
        moved_cards_were_modified: bool,
        flipped_source_top: bool,
    },
    RecycleWaste {
        cards_moved: usize,
    }
}

#[derive(Debug, Clone)]
struct MoveError;

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid move")
    }
}

#[derive(Debug)]
pub struct SolitaireGame {
    deck: Pile,
    tableau: Vec<Pile>,
    foundation: Vec<Pile>,
    waste: Pile,
    history: Vec<UndoRecord>
}

impl Default for SolitaireGame {
    fn default() -> Self {
        Self::new()
    }
}

impl SolitaireGame {
    pub fn new() -> Self {
        let mut deck = new_deck();
        let mut tableau = Vec::with_capacity(7);
        let mut foundation = Vec::with_capacity(4);
        let waste = Pile::new();
        for column in 0..7 {
            let mut pile = Pile::new();
            for row in 0..=column {
                match deck.draw_from_top() {
                    Some(mut card) => {
                        if row == column {
                            card.flip();
                        }
                        pile.add_to_top(card);
                    }
                    None => {
                        panic!("Not enough cards in deck!")
                    }
                }
            }
            tableau.push(pile);
        }
        let deck_len = deck.get_cards().len();
        let deck_cards = deck.get_cards_mut();
        deck_cards[deck_len - 1].make_clickable();
        for _ in 0..4 {
            foundation.push(Pile::new());
        }
        Self {
            deck,
            tableau,
            foundation,
            waste,
            history: vec![]
        }
    }
    pub fn is_game_won(&self) -> bool {
        self.foundation.iter().all(|f| f.size() == 13)
    }
    fn is_on_top(&self, card: &Card) -> bool {
        for pile in &self.tableau {
            if let Some(top_card) = pile.top()
                && top_card == card {
                    return true;
                }
        }
        if let Some(top_card) = self.waste.top()
            && top_card == card {
                return true;
            }
        if let Some(top_card) = self.deck.top()
            && top_card == card {
                return true;
            }
        false
    }
    fn is_foundation_move_valid(&self, card: &Card, destination: &Pile) -> bool {
        if !self.is_on_top(card) {
            return false;
        }

        match destination.top() {
            None => card.get_value() == 1,
            Some(top_card) => {
                top_card.get_value() + 1 == card.get_value()
                    && top_card.get_suit() == card.get_suit()
            }
        }
    }
    fn is_tableau_move_valid(&self, card: &Card, destination: &Pile) -> bool {
        match destination.top() {
            None => {
                if card.get_value() == 13 {
                    return true;
                }
            }
            Some(top_card) => {
                if top_card.get_value() == card.get_value() + 1
                    && top_card.is_red() != card.is_red()
                {
                    return true;
                }
            }
        }
        false
    }
    fn try_get_top_card(&self, from: PileKind) -> Option<(Card, &Pile)> {
        let (card, pile) = match from {
            PileKind::Deck => {
                let card_ref = self.deck.top()?;
                (*card_ref, &self.deck)
            }
            PileKind::Waste => {
                let card_ref = self.waste.top()?;
                (*card_ref, &self.waste)
            }
            PileKind::Tableau(col) => {
                let pile_ref = self.tableau.get(col)?;
                let card_ref = pile_ref.top()?;
                (*card_ref, pile_ref)
            }
            PileKind::Foundation(_) => {
                return None;
            }
        };

        if !self.is_on_top(&card) {
            return None;
        }

        Some((card, pile))
    }
    fn pile_for_kind_mut(&mut self, kind: PileKind) -> Option<&mut Pile> {
        match kind {
            PileKind::Deck => Some(&mut self.deck),
            PileKind::Waste => Some(&mut self.waste),
            PileKind::Tableau(col) => self.tableau.get_mut(col),
            PileKind::Foundation(idx) => self.foundation.get_mut(idx),
        }
    }
    fn move_top_card(&mut self, from: PileKind, to: PileKind) -> Result<UndoRecord, MoveError> {
        let card = match self.try_get_top_card(from) {
            Some((card, _)) => card,
            None => return Err(MoveError),
        };
        let mut moved_cards_were_modified = false;

        let reveal_col = match (from, to) {
            (PileKind::Deck, PileKind::Waste) => {
                if !self.deck.reveal_top_card() {
                    return Err(MoveError);
                }
                moved_cards_were_modified = true;
                None
            }
            (PileKind::Waste, PileKind::Tableau(col)) => {
                let to_pile = self.tableau.get(col).ok_or(MoveError)?;
                if !self.is_tableau_move_valid(&card, to_pile) {
                    return Err(MoveError);
                }
                None
            }
            (PileKind::Waste, PileKind::Foundation(idx)) => {
                let to_pile = self.foundation.get(idx).ok_or(MoveError)?;
                if !self.is_foundation_move_valid(&card, to_pile) {
                    return Err(MoveError);
                }
                None
            }
            (PileKind::Tableau(col1), PileKind::Tableau(col2)) => {
                let to_pile = self.tableau.get(col2).ok_or(MoveError)?;
                if !self.is_tableau_move_valid(&card, to_pile) {
                    return Err(MoveError);
                }
                Some(col1)
            }
            (PileKind::Tableau(col), PileKind::Foundation(idx)) => {
                let to_pile = self.foundation.get(idx).ok_or(MoveError)?;
                if !self.is_foundation_move_valid(&card, to_pile) {
                    return Err(MoveError);
                }
                Some(col)
            }
            _ => {
                return Err(MoveError);
            }
        };
        
        let drawn_card = {
            let from_pile = self.pile_for_kind_mut(from).ok_or(MoveError)?;
            from_pile.draw_from_top().ok_or(MoveError)?
        };

        let flipped_source_top = match reveal_col {
            Some(col) => self.tableau[col].reveal_new_top_if_needed(),
            None => false,
        };

        {
            let to_pile = self.pile_for_kind_mut(to).ok_or(MoveError)?;
            to_pile.add_to_top(drawn_card);
        }

        Ok(UndoRecord::MoveCards { from, to, cards: vec![drawn_card], moved_cards_were_modified , flipped_source_top })
    }
    fn move_stack(&mut self, from: PileKind, count: usize, to: PileKind) -> Result<UndoRecord, MoveError> {
        match count {
            2.. => {
                let (from_col, to_col) = match (from, to) {
                    (PileKind::Tableau(f), PileKind::Tableau(g)) => (f, g),
                    _ => return Err(MoveError),
                };

                if from_col >= self.tableau.len() || to_col >= self.tableau.len() {
                    return Err(MoveError);
                }

                if from_col == to_col {
                    return Err(MoveError);
                }

                let pile_len = self.tableau[from_col].size();

                if count > pile_len {
                    return Err(MoveError);
                }

                let row = pile_len - count;

                let bottom_card = match self.tableau[from_col].get_cards().get(row) {
                    Some(c) => *c,
                    None => return Err(MoveError),
                };

                if !bottom_card.is_face_up() {
                    return Err(MoveError);
                }

                let to_pile = &self.tableau[to_col];
                if !self.is_tableau_move_valid(&bottom_card, to_pile) {
                    return Err(MoveError);
                }

                let stack: Vec<Card> = self.tableau[from_col]
                    .get_cards_mut()
                    .drain(row..)
                    .collect();

                let stack_record = stack.clone();

                let flipped_source_top = self.tableau[from_col].reveal_new_top_if_needed();

                for card in stack {
                    self.tableau[to_col].add_to_top(card);
                }

                Ok(UndoRecord::MoveCards { from, to, cards: stack_record, moved_cards_were_modified: false, flipped_source_top })
            }
            1 => self.move_top_card(from, to),
            _ => Err(MoveError),
        }
    }
    pub fn flush_waste(&mut self) -> bool {
        if self.deck.size() != 0 {
            return false;
        }

        match self.waste.recycle_to_deck(&mut self.deck) {
            Some(cards_moved) => {
                self.history.push(UndoRecord::RecycleWaste { cards_moved });
                true
            },
            None => false,
        }
    }
    pub fn attempt_move(&mut self, mv: Move) -> bool {
        let result = match mv {
            Move::FromDeckToWaste => self.move_stack(PileKind::Deck, 1, PileKind::Waste),
            Move::FromWasteToPile { to } => self.move_stack(PileKind::Waste, 1, to),
            Move::FromTableauToTableau { from, to, count } => self.move_stack(from, count, to),
            Move::FromTableauToFoundation { from, to } => self.move_stack(from, 1, to),
        };

        match result {
            Ok(record) => {
                self.history.push(record);
                true
            },
            Err(_) => false,
        }


    }
    pub fn get_state(&self) -> GameState {
        let card_view = |c: &Card| CardView {
            suit: format!("{:?}", c.get_suit()),
            value: c.get_value(),
            face_up: c.is_face_up(),
        };

        GameState {
            deck_size: self.deck.size(),
            waste: self.waste.get_cards().iter().map(card_view).collect(),
            tableau: self
                .tableau
                .iter()
                .map(|pile| pile.get_cards().iter().map(card_view).collect())
                .collect(),
            foundation: self
                .foundation
                .iter()
                .map(|pile| pile.get_cards().iter().map(card_view).collect())
                .collect(),
        }
    }

    pub fn auto_move_to_foundation(&mut self, from: PileKind) -> bool {
        for i in 0..4 {
            if self.attempt_move(match from {
                PileKind::Waste => Move::FromWasteToPile {
                    to: PileKind::Foundation(i),
                },
                PileKind::Tableau(_) => Move::FromTableauToFoundation {
                    from,
                    to: PileKind::Foundation(i),
                },
                _ => return false,
            }) {
                return true;
            }
        }
        false
    }

    pub fn auto_move_to_tableau(&mut self, from: PileKind, count: usize) -> bool {
        for i in 0..7 {
            if self.attempt_move(match from {
                PileKind::Tableau(_) => Move::FromTableauToTableau {
                    from,
                    to: PileKind::Tableau(i),
                    count,
                },
                PileKind::Waste => Move::FromWasteToPile {
                    to: PileKind::Tableau(i),
                },
                _ => return false,
            }) {
                return true;
            }
        }
        false
    }
    
    fn undo_move_cards(&mut self, from: PileKind, to: PileKind, cards: Vec<Card>, moved_cards_were_modified: bool, flipped_source_top: bool) -> bool {
        let moved_count = cards.len();
        let mut popped = Vec::with_capacity(moved_count);

        {
            let to_pile = match self.pile_for_kind_mut(to) {
                Some(p) => p,
                None => return false,
            };

            for _ in 0..moved_count {
                let drawn = match to_pile.draw_from_top() {
                    Some(c) => c,
                    None => return false,
                };
                popped.push(drawn);
            }

        }

        let expected: Vec<Card> = cards.iter().rev().copied().collect();

        if popped.len() != expected.len() {
            return false;
        }

        for (a, b) in popped.iter().zip(expected.iter()) {
            if !same_card_identity(a, b) {
                return false;
            }
        }

        {
            let from_pile = match self.pile_for_kind_mut(from) {
                Some(p) => p,
                None => return false,
            };

            for card in popped.into_iter().rev() {
                from_pile.add_to_top(card);
            }
        }

        if flipped_source_top {
            match from {
                PileKind::Tableau(col) => {
                    let from_pile = match self.tableau.get_mut(col) {
                        Some(p) => p,
                        None => return false,
                    };

                    let idx = match from_pile.size().checked_sub(moved_count + 1) {
                        Some(i) => i,
                        None => return false,
                    };

                    let card = match from_pile.get_cards_mut().get_mut(idx) {
                        Some(c) => c,
                        None => return false,
                    };

                    card.flip();
                    card.make_unclickable();
                }
                _ => return false,
            }
        }

        if moved_cards_were_modified {
            match from {
                PileKind::Deck => {
                    let from_pile = match self.pile_for_kind_mut(from) {
                        Some(p) => p,
                        None => return false,
                    };

                    for i in 0..moved_count {
                        let idx = match from_pile.size().checked_sub(i + 1) {
                            Some(v) => v,
                            None => return false,
                        };

                        let card = match from_pile.get_cards_mut().get_mut(idx) {
                            Some(c) => c,
                            None => return false,
                        };

                        card.make_unclickable();
                        if card.is_face_up() {
                            card.flip();
                        }
                    }
                    
                    if let Some(top) = from_pile.top_mut() {
                        top.make_clickable();
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        true
    }

    fn undo_recycle_waste(&mut self, cards_moved: usize) -> bool {
        if self.deck.size() < cards_moved {
            return false;
        }

        let mut cards = Vec::with_capacity(cards_moved);

        for _ in 0..cards_moved {
            let card = match self.deck.draw_from_top() {
                Some(c) => c,
                None => return false,
            };
            cards.push(card);
        }

        for card in cards {
            self.waste.add_to_top(card);
        }

        true
    }

    pub fn undo(&mut self) -> bool {
        let record = match self.history.pop() {
            Some(r) => r,
            None => return false,
        };

        match record {
            UndoRecord::MoveCards { from, to, cards, moved_cards_were_modified, flipped_source_top } => self.undo_move_cards(from, to, cards, moved_cards_were_modified, flipped_source_top),
            UndoRecord::RecycleWaste { cards_moved } => self.undo_recycle_waste(cards_moved),
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }
}
