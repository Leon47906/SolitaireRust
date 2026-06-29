use core::panic;
use serde::Serialize;

use crate::cards::Card;
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
    Foundation(usize)
}

#[derive(Debug, Clone, Copy)]
pub enum Move{
    FromDeckToWaste,
    FromWasteToPile(PileKind),
    FromTableauToPile(usize, PileKind)
}

#[derive(Debug)]
pub struct SolitaireGame {
    deck: Pile,
    tableau: Vec<Pile>,
    foundation: Vec<Pile>,
    waste: Pile 
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
                        None => {panic!("Not enough cards in deck!")}
                    }
                }
            tableau.push(pile);
        }
        let deck_len = deck.get_cards().len();
        let deck_cards = deck.get_cards_mut();
        deck_cards[deck_len - 1].make_clickable();
        for _ in 0..4 { foundation.push(Pile::new()); }
        Self {
            deck,
            tableau,
            foundation,
            waste
        }
    }
    pub fn is_game_won(&self) -> bool {
        self.foundation.iter().all(|f| f.size() == 13)
    }
    fn is_on_top(&self, card : &Card) -> bool {
        for pile in &self.tableau {
            match pile.top() {
                Some(top_card) => {
                    if top_card == card { return true; }
                }
                None => {}
            }
        }
        match self.waste.top() {
            Some(top_card) => {
                if top_card == card { return true; }
            }
            None => {}
        }
        match self.deck.top() {
            Some(top_card) => {
                if top_card == card { return true; }
            }
            None => {}
        }
        false
    }
    fn is_foundation_move_valid(&self ,card : &Card, destination : &Pile) -> bool {
        if !self.is_on_top(card) {
            return false;
        }

        match destination.top() {
            None => { card.get_value() == 1 }
            Some(top_card) => {
                top_card.get_value() + 1 == card.get_value() && top_card.get_suit() == card.get_suit()
            }
        }
    }
    fn is_tableau_move_valid(&self, card : &Card, destination : &Pile) -> bool {
        match destination.top() {
            None => {
                if card.get_value() == 13 { return true; }
            }
            Some(top_card) => {
                if top_card.get_value() == card.get_value() + 1 && top_card.is_red() == !card.is_red() { return true; }
            }
        }
        false
    }
    fn try_get_top_card(&self, from : PileKind) -> Option<(Card,&Pile)> {
        let (card, pile) = match from {
            PileKind::Deck => {
                let card_ref = self.deck.top()?;
                (card_ref.clone(), &self.deck)
            }
            PileKind::Waste => {
                let card_ref = self.waste.top()?;
                (card_ref.clone(), &self.waste)
            }
            PileKind::Tableau(col) => {
                let pile_ref = self.tableau.get(col)?;
                let card_ref = pile_ref.top()?;
                (card_ref.clone(), pile_ref)
            }
            PileKind::Foundation(_) => {
                return None;
            }
        };

        if !self.is_on_top(&card) { return None; }

        Some((card, pile))
    }
    fn pile_for_kind_mut(&mut self, kind : PileKind) -> Option<&mut Pile> {
        match kind {
            PileKind::Deck => Some(&mut self.deck),
            PileKind::Waste => Some(&mut self.waste),
            PileKind::Tableau(col) => self.tableau.get_mut(col),
            PileKind::Foundation(idx) => self.foundation.get_mut(idx),
        }
    }
    fn move_top_card(&mut self, from : PileKind, to : PileKind) -> bool {
        let (card, _source) = match self.try_get_top_card(from) {
            Some(tuple) => tuple,
            None => return false,
        };
        let is_valid = match (from, to) {
            (PileKind::Deck, PileKind::Waste) => {
                let cards = &mut self.deck;
                cards.reveal_top_card()
            }
            (PileKind::Waste, PileKind::Tableau(col)) => {
                let to_pile = match self.tableau.get(col) {
                    Some(pile) => pile,
                    None => return false,
                };
                self.is_tableau_move_valid(&card, to_pile)
            }
            (PileKind::Waste, PileKind::Foundation(number)) => {
                let to_pile = match self.foundation.get(number) {
                    Some(pile) => pile,
                    None => return false,
                };
                self.is_foundation_move_valid(&card, to_pile)
            }
            (PileKind::Tableau(col1), PileKind::Tableau(col2)) => {
                let to_pile = match self.tableau.get(col2) {
                    Some(pile) => pile,
                    None => return false,
                };
                if self.is_tableau_move_valid(&card, to_pile) {
                    self.tableau[col1].tableau_reveal()
                }
                else { false }
            }
            (PileKind::Tableau(col), PileKind::Foundation(number)) => {
                let to_pile = match self.foundation.get(number) {
                    Some(pile) => pile,
                    None => return false,
                };
                if self.is_foundation_move_valid(&card, to_pile) {
                    self.tableau[col].tableau_reveal()
                }
                else { false }
            }
            _ => { return false; }
        };
        if !is_valid { return false; }

        let from_pile = match self.pile_for_kind_mut(from) {
            Some(p) => p,
            None => return false,
        };

        let card = match from_pile.draw_from_top() {
                Some(card) => card,
                None => return false,
        };

        let to_pile = match self.pile_for_kind_mut(to) {
                Some(p) => p,
                None => return false,
        };

        to_pile.add_to_top(card);
    
        true
    }
    pub fn move_stack (&mut self, from : PileKind, row : usize, to : PileKind) -> bool {
        let (from_col, to_col) = match (from, to) {
            (PileKind::Tableau(f), PileKind::Tableau(g)) => (f, g),
            _ => return false,
        };
        
        if from_col >= self.tableau.len() || to_col >= self.tableau.len() { return false; }

        if from_col == to_col { return false; }

        let pile_len = self.tableau[from_col].size();

        if row >= pile_len { return false; }

        let bottom_card = match self.tableau[from_col].get_cards().get(row) {
            Some(c) => c.clone(),
            None => return false,
        };

        if !bottom_card.is_face_up() { return false; }

        let to_pile = &self.tableau[to_col];
        if !self.is_tableau_move_valid(&bottom_card, to_pile) { return false; }

        let stack : Vec<Card> = self.tableau[from_col]
            .get_cards_mut()
            .drain(row..)
            .collect();

        for card in stack {
            self.tableau[to_col].add_to_top(card);
        }

        self.tableau[from_col].tableau_reveal_last();

        true
    }
    pub fn flush_waste(&mut self) -> bool {
        match self.deck.size() {
            0 => {
                self.waste.recycle_to_deck(&mut self.deck);
                return true;
            }
            _ => { return false;}
        }
    }
    pub fn attempt_move(&mut self, mv : Move) -> bool {
        match mv {
            Move::FromDeckToWaste => {
                self.move_top_card(PileKind::Deck, PileKind::Waste)
            }
            Move::FromWasteToPile(to) => {
                self.move_top_card(PileKind::Waste, to)
            }
            Move::FromTableauToPile(from_col, to) => {
                self.move_top_card(PileKind::Tableau(from_col), to)
            }
        }
    }
    pub fn get_state(&self) -> GameState {
        let card_view = |c : &Card| CardView {
            suit:format!("{:?}",c.get_suit()),
            value:c.get_value(),
            face_up:c.is_face_up(),
        };

        GameState {
            deck_size:self.deck.size(),
            waste:self.waste.get_cards().iter().map(card_view).collect(),
            tableau:self.tableau.iter().map(|pile| pile.get_cards().iter().map(card_view).collect()).collect(),
            foundation:self.foundation.iter().map(|pile| pile.get_cards().iter().map(card_view).collect()).collect(),
        }
    }
    
    pub fn auto_move_to_foundation(&mut self, from: PileKind) -> bool {
        for i in 0..4 {
            if self.attempt_move(match from {
                PileKind::Waste => Move::FromWasteToPile(PileKind::Foundation(i)),
                PileKind::Tableau(col) => Move::FromTableauToPile(col, PileKind::Foundation(i)),
                _ => return false,
            }) {
                return true;
            }
        }
        false
    }

    pub fn auto_move_to_tableau(&mut self, from: PileKind) -> bool {
        for i in 0..7 {
            if self.attempt_move(match from {
                PileKind::Tableau(j) => Move::FromTableauToPile(j, PileKind::Tableau(i)),
                PileKind::Waste => Move::FromWasteToPile(PileKind::Tableau(i)),
                _ => return false,
            }) {
                return true;
            }
        }
        false
    }
}
