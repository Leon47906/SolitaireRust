use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug, Clone, Copy, Eq, Serialize)]
pub struct Card {
    value: u8,
    suit: Suit,
    face_up: bool,
    is_clickable: bool,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suit == other.suit
    }
}

impl Card {
    pub fn new(value_: u8, suit_: Suit) -> Self {
        let value = value_;
        let suit = suit_;
        let face_up = false;
        let is_clickable = false;
        Self {
            value,
            suit,
            face_up,
            is_clickable,
        }
    }
    pub fn flip(&mut self) {
        self.face_up = !self.face_up;
    }
    pub fn is_face_up(&self) -> bool {
        self.face_up
    }
    pub fn make_clickable(&mut self) {
        self.is_clickable = true;
    }
    pub fn make_unclickable(&mut self) {
        self.is_clickable = false;
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn get_suit(&self) -> Suit {
        self.suit
    }
    pub fn is_red(&self) -> bool {
        match self.get_suit() {
            Suit::Hearts => true,
            Suit::Diamonds => true,
            Suit::Spades => false,
            Suit::Clubs => false,
        }
    }
}

pub fn same_card_identity(a: &Card, b: &Card) -> bool {
    a.get_value() == b.get_value() && a.get_suit() == b.get_suit()
}
