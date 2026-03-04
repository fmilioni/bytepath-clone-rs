pub mod components;
pub mod data;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

pub use components::{
    ActiveScenario, CampaignProgress, ScenarioKillCount,
    ScenarioWinTimer, SelectedScenario,
};

fn pause_time(mut time: ResMut<Time<Virtual>>) { time.pause(); }
fn unpause_time(mut time: ResMut<Time<Virtual>>) { time.unpause(); }

pub struct CampaignPlugin;

impl Plugin for CampaignPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CampaignProgress::default())
            .insert_resource(ActiveScenario { id: 0 })
            .insert_resource(ScenarioKillCount::default())
            .insert_resource(SelectedScenario::default())
            .insert_resource(ScenarioWinTimer::default())
            // Carrega save completo ao iniciar
            .add_systems(Startup, systems::load_save)
            // Configura cenario ao entrar em Playing (guarda por scenario_id)
            .add_systems(OnEnter(GameState::Playing), systems::setup_scenario)
            // Pausa/despausa o tempo ao abrir/fechar menus
            .add_systems(OnEnter(GameState::Paused), pause_time)
            .add_systems(OnExit(GameState::Paused), unpause_time)
            // Limpa entidades so ao SAIR do jogo de verdade (não ao pausar)
            .add_systems(OnEnter(GameState::ScenarioSelect), systems::cleanup_gameplay)
            .add_systems(OnEnter(GameState::GameOver), systems::cleanup_gameplay)
            // Sistemas de campanha durante o jogo
            .add_systems(
                Update,
                (
                    systems::track_scenario_kills,
                    systems::auto_spawn_boss,
                    systems::check_scenario_win,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Autosave sempre que créditos/skills/inventory/progress mudam
            .add_systems(
                Update,
                systems::autosave.run_if(
                    in_state(GameState::Playing)
                        .or(in_state(GameState::Paused))
                        .or(in_state(GameState::ScenarioSelect)),
                ),
            );
    }
}
