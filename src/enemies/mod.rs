pub mod components;
pub mod spawner;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use spawner::{spawn_enemies, EnemySpawnTimer};
use systems::{
    bomber_system, enemy_ai, enemy_player_collision, minion_spawner_system, sniper_system,
    splitter_system, teleporter_system, update_sniper_warnings,
};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            2.2,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (
                spawn_enemies,
                enemy_ai,
                enemy_player_collision,
                bomber_system,
                sniper_system,
                update_sniper_warnings,
                splitter_system,
                teleporter_system,
                minion_spawner_system,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
