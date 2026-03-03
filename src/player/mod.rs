pub mod abilities;
pub mod components;
pub mod ship_classes;
pub mod spawn;
pub mod stat_calc;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use abilities::{brawler_melee_system, stealth_cloak_system};
use ship_classes::SelectedShipClass;
use spawn::{despawn_player, spawn_camera, spawn_player};
use systems::player_movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedShipClass>()
            // Câmera spawna uma vez, persiste em todos os estados
            .add_systems(Startup, spawn_camera)
            // Jogador spawna ao entrar em Playing, some ao sair
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(OnExit(GameState::Playing), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    stealth_cloak_system,
                    brawler_melee_system,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
