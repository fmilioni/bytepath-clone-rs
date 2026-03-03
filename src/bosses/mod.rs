pub mod components;
pub mod spawner;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use components::{
    ActiveBoss, BossPhaseTransition,
};
use spawner::spawn_boss_on_key;
use systems::{
    apply_boss_phase, boss_death_cleanup, boss_phase_check,
    dreadnought_system, phantom_system, sentinel_system, shield_panel_absorb,
    singularity_system, swarmmother_system, track_active_boss, update_shield_panels,
};

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ActiveBoss::default())
            .add_event::<BossPhaseTransition>()
            .add_systems(
                Update,
                (
                    // Controle e estado
                    spawn_boss_on_key,
                    track_active_boss,
                    boss_phase_check,
                    apply_boss_phase,
                    // IA dos bosses
                    sentinel_system,
                    update_shield_panels,
                    swarmmother_system,
                    dreadnought_system,
                    phantom_system,
                    singularity_system,
                    // Defesa e limpeza
                    shield_panel_absorb,
                    boss_death_cleanup,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
