use bevy::prelude::*;

use crate::bosses::components::Boss;
use crate::combat::components::{DeathEvent, Energy, Health, Shield};
use crate::player::components::{Player, ShipStats};
use crate::player::stat_calc::apply_full_stats;
use crate::shop::components::PlayerInventory;
use crate::weapons::components::WeaponCooldown;

use super::components::{
    apply_effect_to_skills, PlayerSkills, SkillNodeUnlocked, SkillTreeUiState,
};
use super::data::all_nodes;

/// Concede pontos de skill ao matar inimigos (2 por kill, 50 por boss).
pub fn award_skill_points(
    mut death_events: EventReader<DeathEvent>,
    mut skills: ResMut<PlayerSkills>,
    boss_q: Query<(), With<Boss>>,
) {
    for event in death_events.read() {
        if !event.was_enemy { continue; }
        skills.skill_points += if boss_q.contains(event.entity) { 50 } else { 2 };
    }
}

/// Abre/fecha a skill tree com Tab — pausa o jogo enquanto aberta.
pub fn toggle_skill_tree(
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<SkillTreeUiState>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    current_state: Res<State<crate::states::GameState>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        ui_state.open = !ui_state.open;
        if ui_state.open {
            ui_state.selected_cluster = 0;
            ui_state.selected_node_idx = 0;
            ui_state.scroll_offset = 0;
            next_state.set(crate::states::GameState::Paused);
        } else if *current_state.get() == crate::states::GameState::Paused {
            next_state.set(crate::states::GameState::Playing);
        }
    }
}

/// Navegação e desbloqueio de nós quando a skill tree está aberta.
pub fn skill_tree_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<SkillTreeUiState>,
    mut skills: ResMut<PlayerSkills>,
    mut unlock_events: EventWriter<SkillNodeUnlocked>,
    selected: Res<crate::player::ship_classes::SelectedShipClass>,
    inventory: Res<PlayerInventory>,
    mut player_q: Query<
        (&mut ShipStats, &mut Health, &mut Shield, &mut Energy, &mut WeaponCooldown),
        With<Player>,
    >,
) {
    if !ui_state.open { return; }

    let nodes = all_nodes();
    let cluster = super::data::SkillCluster::ALL[ui_state.selected_cluster];
    let cluster_nodes: Vec<_> = nodes.iter().filter(|n| n.cluster == cluster).collect();

    // A/D — muda cluster
    let left  = keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft);
    let right = keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight);
    let up    = keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp);
    let down  = keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown);

    if left || right {
        let len = super::data::SkillCluster::ALL.len();
        if left {
            ui_state.selected_cluster = (ui_state.selected_cluster + len - 1) % len;
        } else {
            ui_state.selected_cluster = (ui_state.selected_cluster + 1) % len;
        }
        ui_state.selected_node_idx = 0;
        ui_state.scroll_offset = 0;
        return;
    }

    // W/S — navega nós
    if !cluster_nodes.is_empty() {
        const MAX_VISIBLE: usize = 9;
        if up && ui_state.selected_node_idx > 0 {
            ui_state.selected_node_idx -= 1;
            if ui_state.selected_node_idx < ui_state.scroll_offset {
                ui_state.scroll_offset = ui_state.selected_node_idx;
            }
        }
        if down && ui_state.selected_node_idx + 1 < cluster_nodes.len() {
            ui_state.selected_node_idx += 1;
            if ui_state.selected_node_idx >= ui_state.scroll_offset + MAX_VISIBLE {
                ui_state.scroll_offset = ui_state.selected_node_idx - MAX_VISIBLE + 1;
            }
        }
    }

    // Enter — desbloqueia nó selecionado
    if keys.just_pressed(KeyCode::Enter) {
        if let Some(node) = cluster_nodes.get(ui_state.selected_node_idx) {
            let can_unlock = !skills.unlocked.contains(&node.id)
                && skills.skill_points >= node.cost
                && node.prereq.map_or(true, |p| skills.unlocked.contains(&p));

            if can_unlock {
                skills.skill_points -= node.cost;
                skills.unlocked.insert(node.id);
                apply_effect_to_skills(&mut skills, &node.effect);
                unlock_events.send(SkillNodeUnlocked(node.id));

                // Aplica ao player imediatamente (se estiver em jogo)
                if let Ok((mut stats, mut health, mut shield, mut energy, mut cooldown)) =
                    player_q.get_single_mut()
                {
                    let base = selected.0.to_ship_stats();
                    apply_full_stats(
                        &skills, &inventory, &base, &mut stats, &mut health,
                        &mut shield, &mut energy, &mut cooldown,
                    );
                }
            }
        }
    }
}


/// Regeneração passiva de HP por segundo (skill tree + itens).
pub fn hp_regen_system(
    time: Res<Time>,
    skills: Res<PlayerSkills>,
    inventory: Res<PlayerInventory>,
    mut player_q: Query<&mut Health, With<Player>>,
) {
    let item_regen: f32 = inventory.items.iter().map(|&id| id.def().hp_regen).sum();
    let total = skills.hp_regen + item_regen;
    if total <= 0.0 { return; }
    let Ok(mut health) = player_q.get_single_mut() else { return; };
    if health.current < health.max {
        health.current = (health.current + total * time.delta_secs()).min(health.max);
    }
}
