use wasm_bindgen::prelude::*;
use solitaire::{SolitaireGame, Move, game::PileKind};
use serde_wasm_bindgen;

#[wasm_bindgen]
pub struct WasmGame {
    game: SolitaireGame,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        WasmGame {
            game: SolitaireGame::new(),
        }
    }
    
    pub fn from_deck_to_waste(&mut self) -> bool {
        self.game.attempt_move(Move::FromDeckToWaste)
    }

    pub fn get_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.game.get_state()).unwrap()
    }

    pub fn attempt_move_waste_to_tableau(&mut self, col: usize) -> bool {
        self.game.attempt_move(Move::FromWasteToPile(PileKind::Tableau(col)))
    }

    pub fn attempt_move_waste_to_foundation(&mut self, idx: usize) -> bool {
        self.game.attempt_move(Move::FromWasteToPile(PileKind::Foundation(idx)))
    }

    pub fn attempt_move_tableau_to_tableau(&mut self, from: usize, to: usize) -> bool {
        self.game.attempt_move(Move::FromTableauToPile(from, PileKind::Tableau(to)))
    }

    pub fn attempt_move_tableau_to_foundation(&mut self, from: usize, idx: usize) -> bool {
        self.game.attempt_move(Move::FromTableauToPile(from, PileKind::Foundation(idx)))
    }

    pub fn flush_waste(&mut self) -> bool {
        self.game.flush_waste()
    }

    pub fn move_stack(&mut self, from_col : usize, row : usize, to_col : usize) -> bool {
        self.game.move_stack(PileKind::Tableau(from_col), row, PileKind::Tableau(to_col))
    }

    pub fn auto_move_waste_to_foundation(&mut self) -> bool {
        self.game.auto_move_to_foundation(PileKind::Waste)
    }

    pub fn auto_move_tableau_to_foundation(&mut self, col: usize) -> bool {
        self.game.auto_move_to_foundation(PileKind::Tableau(col))
    }

    pub fn auto_move_waste_to_tableau(&mut self) -> bool {
        self.game.auto_move_to_tableau(PileKind::Waste)
    }

    pub fn auto_move_tableau_to_tableau(&mut self, col: usize) -> bool {
        self.game.auto_move_to_tableau(PileKind::Tableau(col))
    }

    pub fn is_game_won(&self) -> bool {
        self.game.is_game_won()
    }
}

