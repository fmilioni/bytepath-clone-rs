pub mod components;
pub mod data;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use components::{PlayerSkills, SkillNodeUnlocked, SkillTreeUiState};
use systems::{award_skill_points, hp_regen_system, skill_tree_input, toggle_skill_tree};

pub struct SkillTreePlugin;

impl Plugin for SkillTreePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerSkills>()
            .init_resource::<SkillTreeUiState>()
            .add_event::<SkillNodeUnlocked>()
            // Sistemas de jogo — só em Playing
            .add_systems(
                Update,
                (award_skill_points, hp_regen_system)
                    .run_if(in_state(GameState::Playing)),
            )
            // Toggle Tab — roda em Playing E Paused (para poder fechar enquanto pausado)
            .add_systems(
                Update,
                toggle_skill_tree
                    .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            )
            // Navegação da skill tree — só quando pausado (tree aberta)
            .add_systems(
                Update,
                skill_tree_input.run_if(in_state(GameState::Paused)),
            );
    }
}
