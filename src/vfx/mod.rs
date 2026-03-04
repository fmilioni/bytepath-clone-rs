pub mod components;
pub mod particles;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use components::ScreenShake;
use systems::{
    apply_screen_shake, drift_stars_menu, on_death_particles, spawn_stars, spawn_trail,
    update_letterbox, update_particles, update_shield_ring, update_trail,
};

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenShake>()
            // Estrelas existem em todos os estados (spawnam uma vez no início)
            .add_systems(Startup, spawn_stars)
            // Letterbox: roda em todos os estados para manter aspect ratio
            .add_systems(Update, update_letterbox)
            // Drift de estrelas nos menus
            .add_systems(
                Update,
                drift_stars_menu.run_if(
                    in_state(GameState::MainMenu)
                        .or(in_state(GameState::ShipSelect))
                        .or(in_state(GameState::ScenarioSelect))
                        .or(in_state(GameState::GameOver)),
                ),
            )
            .add_systems(
                Update,
                (
                    spawn_trail,
                    update_trail,
                    update_particles,
                    apply_screen_shake,
                    on_death_particles,
                    update_shield_ring,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
