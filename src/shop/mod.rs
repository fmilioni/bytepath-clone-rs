pub mod components;
pub mod systems;
pub mod ui;

use bevy::prelude::*;
use crate::states::GameState;

use components::{Credits, PlayerInventory, ShopUiState};
use systems::{award_credits, open_close_shop, shop_buy, shop_navigate, shop_reroll};
use ui::update_shop_ui;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Credits>()
            .init_resource::<PlayerInventory>()
            .init_resource::<ShopUiState>()
            // Créditos só ganhos em Playing
            .add_systems(Update, award_credits.run_if(in_state(GameState::Playing)))
            // Toggle loja roda em Playing e Paused
            .add_systems(
                Update,
                open_close_shop
                    .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            )
            // Navegação e compra — só quando pausado (loja aberta)
            .add_systems(
                Update,
                (shop_navigate, shop_buy, shop_reroll).run_if(in_state(GameState::Paused)),
            )
            // UI atualiza em Playing e Paused
            .add_systems(
                Update,
                update_shop_ui
                    .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            );
    }
}
