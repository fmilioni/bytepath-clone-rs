pub mod components;
pub mod particles;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use components::ScreenShake;
use systems::{
    apply_screen_shake, despawn_region_background, drift_stars_menu, on_death_particles,
    reset_stars_default, spawn_region_background, spawn_stars, spawn_trail, update_letterbox,
    update_particles, update_shield_ring, update_trail,
};

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenShake>()
            .add_systems(Startup, spawn_stars)
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
            // Background temático: spawn ao entrar em Playing
            .add_systems(OnEnter(GameState::Playing), spawn_region_background)
            // Background temático: cleanup ao sair de Playing de verdade
            .add_systems(
                OnEnter(GameState::ScenarioSelect),
                despawn_region_background,
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                (despawn_region_background, reset_stars_default),
            )
            // Sistemas de gameplay (Playing + Paused para trail continuar)
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
