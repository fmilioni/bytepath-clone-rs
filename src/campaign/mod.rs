pub mod components;
pub mod data;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

pub use components::{
    ActiveScenario, CampaignProgress, ScenarioKillCount,
    ScenarioWinTimer, SelectedScenario,
};

pub struct CampaignPlugin;

impl Plugin for CampaignPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CampaignProgress::default())
            .insert_resource(ActiveScenario { id: 0 })
            .insert_resource(ScenarioKillCount::default())
            .insert_resource(SelectedScenario::default())
            .insert_resource(ScenarioWinTimer::default())
            // Carrega progresso ao iniciar
            .add_systems(Startup, systems::load_campaign_progress)
            // Configura cenário ao entrar em Playing
            .add_systems(OnEnter(GameState::Playing), systems::setup_scenario)
            // Limpa entidades de gameplay ao sair de Playing
            .add_systems(OnExit(GameState::Playing), systems::cleanup_gameplay)
            // Sistemas de campanha durante o jogo
            .add_systems(
                Update,
                (
                    systems::track_scenario_kills,
                    systems::auto_spawn_boss,
                    systems::check_scenario_win,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
