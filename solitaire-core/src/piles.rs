use crate::cards::*;

use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
    pub fn new_with(cards: Vec<Card>) -> Self {
        Self { cards: cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        return &self.cards;
    }

    pub fn get_cards_mut(&mut self) -> &mut Vec<Card> {
        return &mut self.cards;
    }

    pub fn draw_from_top(&mut self) -> Option<Card> {
        match self.size() {
            2.. => {
                let len = self.size();
                self.cards[len - 2].make_clickable();
            }
            _ => {}
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
    pub fn tableau_reveal(&mut self) -> bool {
        match self.size() {
            0 => false,
            1 => true,
            2.. => {
                let len = self.size();
                if self.cards[len - 2].is_face_up() {
                    self.cards[len - 2].make_clickable();
                    return true;
                } else {
                    self.cards[len - 2].flip();
                    self.cards[len - 2].make_clickable();
                    true
                }
            }
        }
    }
    pub fn tableau_reveal_last(&mut self) -> bool {
        match self.size() {
            0 => false,
            1.. => {
                let len = self.size();
                if self.cards[len - 1].is_face_up() {
                    self.cards[len - 1].make_clickable();
                    return true;
                } else {
                    self.cards[len - 1].flip();
                    self.cards[len - 1].make_clickable();
                    true
                }
            }
        }
    }
    pub fn recycle_to_deck(&mut self, dest: &mut Pile) {
        let mut cards = self.cards.drain(..).collect::<Vec<_>>();
        cards.reverse();

        for card in &mut cards {
            card.make_unclickable();
            if card.is_face_up() {
                card.flip();
            }
        }

        match cards.len() {
            0 => {}
            _ => {
                let len = cards.len();
                cards[len - 1].make_clickable();
            }
        }

        dest.cards.extend(cards);
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
