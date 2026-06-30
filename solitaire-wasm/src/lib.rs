use solitaire::{game::PileKind, Move, SolitaireGame};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmGame {
    game: SolitaireGame,
}

#[wasm_bindgen]
impl WasmGame {
    #[allow(clippy::new_without_default)]
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
        self.game.attempt_move(Move::FromWasteToPile {
            to: PileKind::Tableau(col),
        })
    }

    pub fn attempt_move_waste_to_foundation(&mut self, idx: usize) -> bool {
        self.game.attempt_move(Move::FromWasteToPile {
            to: PileKind::Foundation(idx),
        })
    }

    pub fn attempt_move_tableau_to_tableau(
        &mut self,
        from: usize,
        to: usize,
        count: usize,
    ) -> bool {
        self.game.attempt_move(Move::FromTableauToTableau {
            from: PileKind::Tableau(from),
            to: PileKind::Tableau(to),
            count,
        })
    }

    pub fn attempt_move_tableau_to_foundation(&mut self, from: usize, idx: usize) -> bool {
        self.game.attempt_move(Move::FromTableauToFoundation {
            from: PileKind::Tableau(from),
            to: PileKind::Foundation(idx),
        })
    }

    pub fn flush_waste(&mut self) -> bool {
        self.game.flush_waste()
    }

    pub fn auto_move_waste_to_foundation(&mut self) -> bool {
        self.game.auto_move_to_foundation(PileKind::Waste)
    }

    pub fn auto_move_tableau_to_foundation(&mut self, col: usize) -> bool {
        self.game.auto_move_to_foundation(PileKind::Tableau(col))
    }

    pub fn auto_move_waste_to_tableau(&mut self) -> bool {
        self.game.auto_move_to_tableau(PileKind::Waste, 1)
    }

    pub fn auto_move_tableau_to_tableau(&mut self, col: usize, count: usize) -> bool {
        self.game
            .auto_move_to_tableau(PileKind::Tableau(col), count)
    }

    pub fn is_game_won(&self) -> bool {
        self.game.is_game_won()
    }
}
