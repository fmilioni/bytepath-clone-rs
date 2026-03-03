pub mod components;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use systems::{enemy_shoot, move_projectiles, player_shoot, projectile_collision};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_shoot,
                enemy_shoot,
                move_projectiles,
                projectile_collision,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
