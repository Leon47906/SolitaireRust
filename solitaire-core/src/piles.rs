use crate::cards::*;

use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Pile {
    cards: Vec<Card>,
}

impl Default for Pile {
    fn default() -> Self {
        Self::new()
    }
}

impl Pile {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
    pub fn new_with(cards: Vec<Card>) -> Self {
        Self { cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn get_cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn draw_from_top(&mut self) -> Option<Card> {
        if let 2.. = self.size() {
            let len = self.size();
            self.cards[len - 2].make_clickable();
        }
        self.cards.pop()
    }

    pub fn add_to_top(&mut self, card: Card) {
        match self.size() {
            0 => {}
            1.. => {
                let len = self.size();
                self.cards[len - 1].make_unclickable();
            }
        }
        self.cards.push(card);
    }

    pub fn top(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn top_mut(&mut self) -> Option<&mut Card> {
        self.cards.last_mut()
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }
    pub fn reveal_top_card(&mut self) -> bool {
        match self.size() {
            0 => false,
            1 => {
                self.cards[0].flip();
                true
            }
            2.. => {
                let len = self.size();
                self.cards[len - 1].flip();
                self.cards[len - 2].make_clickable();
                true
            }
        }
    }
    pub fn reveal_new_top_if_needed(&mut self) -> bool {
        match self.size() {
            0 => false,
            _ => {
                let len = self.size();
                if self.cards[len - 1].is_face_up() {
                    self.cards[len - 1].make_clickable();
                    false
                } else {
                    self.cards[len - 1].flip();
                    self.cards[len - 1].make_clickable();
                    true
                }
            }
        }
    }
    pub fn recycle_to_deck(&mut self, dest: &mut Pile) -> Option<usize> {
        let mut cards = self.cards.drain(..).collect::<Vec<_>>();
        if cards.is_empty() {
            return None;
        }
        cards.reverse();

        for card in &mut cards {
            card.make_unclickable();
            if card.is_face_up() {
                card.flip();
            }
        }

        let len = cards.len();

        cards[len - 1].make_clickable();

        dest.cards.extend(cards);

        Some(len)
    }
}

pub fn new_deck() -> Pile {
    let mut cards = Vec::with_capacity(52);
    for value in 1..=13 {
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs] {
            cards.push(Card::new(value, suit));
        }
    }
    let mut deck = Pile::new_with(cards);
    deck.shuffle();
    let len = deck.cards.len();
    deck.cards[len - 1].make_clickable();
    deck
}
