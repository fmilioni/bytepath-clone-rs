pub mod components;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use components::{DamageEvent, DeathEvent};
use systems::{apply_damage, handle_deaths, shield_energy_regen};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(
                Update,
                (
                    shield_energy_regen,
                    apply_damage,
                    handle_deaths,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
