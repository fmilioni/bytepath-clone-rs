use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::bosses::components::Boss;
use crate::combat::components::{DeathEvent, Energy, Health, Shield};
use crate::player::components::{Player, ShipStats};
use crate::player::ship_classes::SelectedShipClass;
use crate::player::stat_calc::apply_full_stats;
use crate::skill_tree::components::{PlayerSkills, SkillTreeUiState};
use crate::states::GameState;
use crate::weapons::components::WeaponCooldown;

use super::components::{ALL_ITEMS, Credits, ItemId, PlayerInventory, ShopUiState};

/// Concede créditos ao matar inimigos: 5 por regular, 100 por boss.
pub fn award_credits(
    mut death_events: EventReader<DeathEvent>,
    mut credits: ResMut<Credits>,
    boss_q: Query<(), With<Boss>>,
) {
    for event in death_events.read() {
        if !event.was_enemy { continue; }
        credits.0 += if boss_q.contains(event.entity) { 40 } else { 3 };
    }
}

/// Abre (E em Playing) e fecha (E em Paused com loja aberta) a loja.
pub fn open_close_shop(
    keys: Res<ButtonInput<KeyCode>>,
    mut shop_state: ResMut<ShopUiState>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    skill_ui: Res<SkillTreeUiState>,
    inventory: Res<PlayerInventory>,
) {
    if !keys.just_pressed(KeyCode::KeyE) { return; }

    match current_state.get() {
        GameState::Playing if !skill_ui.open => {
            shop_state.open = true;
            shop_state.selected = 0;
            shop_state.offered = generate_offer(&inventory);
            next_state.set(GameState::Paused);
        }
        GameState::Paused if shop_state.open => {
            shop_state.open = false;
            next_state.set(GameState::Playing);
        }
        _ => {}
    }
}

fn generate_offer(inventory: &PlayerInventory) -> Vec<ItemId> {
    let mut rng = rand::thread_rng();
    let mut available: Vec<ItemId> = ALL_ITEMS.iter()
        .filter(|&&id| !inventory.items.contains(&id))
        .copied()
        .collect();
    available.shuffle(&mut rng);
    available.into_iter().take(4).collect()
}

/// Navega os itens da oferta com A/D.
pub fn shop_navigate(
    keys: Res<ButtonInput<KeyCode>>,
    mut shop_state: ResMut<ShopUiState>,
) {
    if !shop_state.open || shop_state.offered.is_empty() { return; }

    let len = shop_state.offered.len();
    if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        shop_state.selected = (shop_state.selected + len - 1) % len;
    }
    if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        shop_state.selected = (shop_state.selected + 1) % len;
    }
}

/// Compra o item selecionado (Enter).
pub fn shop_buy(
    keys: Res<ButtonInput<KeyCode>>,
    mut shop_state: ResMut<ShopUiState>,
    mut credits: ResMut<Credits>,
    mut inventory: ResMut<PlayerInventory>,
    skills: Res<PlayerSkills>,
    selected_class: Res<SelectedShipClass>,
    mut player_q: Query<
        (&mut ShipStats, &mut Health, &mut Shield, &mut Energy, &mut WeaponCooldown),
        With<Player>,
    >,
) {
    if !shop_state.open || !keys.just_pressed(KeyCode::Enter) { return; }
    let Some(&item_id) = shop_state.offered.get(shop_state.selected) else { return; };
    let def = item_id.def();
    if credits.0 < def.cost { return; }

    credits.0 -= def.cost;
    inventory.items.push(item_id);

    let idx = shop_state.selected;
    shop_state.offered.remove(idx);
    if !shop_state.offered.is_empty() {
        shop_state.selected = idx.min(shop_state.offered.len() - 1);
    }

    // Recalcula stats com skills + todos os itens
    if let Ok((mut stats, mut health, mut shield, mut energy, mut cooldown)) =
        player_q.get_single_mut()
    {
        let base = selected_class.0.to_ship_stats();
        apply_full_stats(&skills, &inventory, &base, &mut stats, &mut health, &mut shield, &mut energy, &mut cooldown);
    }
}

/// Renova a oferta (R) por 30 créditos.
pub fn shop_reroll(
    keys: Res<ButtonInput<KeyCode>>,
    mut shop_state: ResMut<ShopUiState>,
    mut credits: ResMut<Credits>,
    inventory: Res<PlayerInventory>,
) {
    if !shop_state.open || !keys.just_pressed(KeyCode::KeyR) { return; }
    const REROLL_COST: u32 = 30;
    if credits.0 < REROLL_COST { return; }
    credits.0 -= REROLL_COST;
    shop_state.offered = generate_offer(&inventory);
    shop_state.selected = 0;
}
