use crate::cards::*;
use crate::stack::CardStack;

use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for value in 1..=13 {
            for suit in [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs] {
                cards.push(Card::new(value, suit));
            }
        }
        Self { cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }
}

impl CardStack for Deck {
    fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
    fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }
}
