use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Card{
    value: u8,
    suit: Suit,
    face_up : bool,
    is_clickable : bool
}

impl Card {
    pub fn new(value_ : u8, suit_ : Suit) -> Self {
        let value = value_;
        let suit = suit_;
        let face_up = false;
        let is_clickable = false;
        Self {
            value,
            suit,
            face_up,
            is_clickable
        }
    }
    pub fn flip(&mut self) { self.face_up = ! self.face_up; }
    pub fn is_face_up(&self) -> bool { return self.face_up; }
    pub fn make_clickable(&mut self) { self.is_clickable = true; }
    pub fn make_unclickable(&mut self) { self.is_clickable = false; }
    pub fn get_value(&self) -> u8 { return self.value; }
    pub fn get_suit(&self) -> Suit { return self.suit; }
    pub fn is_red(&self) -> bool {
        match self.get_suit() {
            Suit::Hearts => { return true; }
            Suit::Diamonds => { return true; }
            Suit::Spades => { return false; }
            Suit::Clubs => {return false; }
        }
    }
}
