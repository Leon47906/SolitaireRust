use crate::cards::Card;

pub trait CardStack {
    fn cards(&self) -> &Vec<Card>;
    fn cards_mut(&mut self) -> &mut Vec<Card>;

    fn draw_from_top(&mut self) -> Option<Card> {
        self.cards_mut().pop()
    }

    fn add_to_top(&mut self, card: Card) {
        self.cards_mut().push(card);
    }

    fn top(&self) -> Option<&Card> {
        self.cards().last()
    }

    fn size(&self) -> usize {
        self.cards().len()
    }
}
