use solitaire::{SolitaireGame, game::Move};

fn main() {
    let mut game = SolitaireGame::new();
    println!("{:#?}", game);
    while game.attempt_move(Move::FromDeckToWaste) {
        
    }
    println!("{:#?}", game);
    game.flush_waste();
    println!("{:#?}",game);
}
