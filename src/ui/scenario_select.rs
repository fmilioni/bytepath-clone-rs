use bevy::prelude::*;

use crate::campaign::components::{CampaignProgress, SelectedScenario};
use crate::campaign::{ActiveScenario, ScenarioWinTimer};
use crate::campaign::data::all_scenarios;
use crate::states::GameState;

// ── Marcadores ────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct ScenarioSelectRoot;

#[derive(Component)]
pub struct ScenarioRow(pub u32); // scenario id

/// Contador "X/25 completados" no header
#[derive(Component)]
pub struct ScenarioCountHint;

/// Título/status do cenário selecionado no painel de detalhe
#[derive(Component)]
pub struct ScenarioTitleHint;

/// Descrição do cenário selecionado
#[derive(Component)]
pub struct ScenarioDescHint;


#[derive(Component)]
pub struct ScenarioWinOverlay;

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_scenario_select(
    mut commands: Commands,
    progress: Res<CampaignProgress>,
    mut selected: ResMut<SelectedScenario>,
) {
    // Seleciona o primeiro cenário disponível
    if selected.id == 0 {
        selected.id = 1;
        selected.scroll_offset = 0;
    }

    commands
        .spawn((
            ScenarioSelectRoot,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.05, 0.95)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(780.0),
                height: Val::Px(560.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(6.0),
                ..default()
            })
            .with_children(|panel| {
                // ── Cabeçalho ──────────────────────────────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("CAMPANHA  [↑↓ selecionar | ENTER jogar | ESC voltar]"),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.5, 0.6)),
                    ));
                    let done = progress.completed.len();
                    let total = all_scenarios().len();
                    header.spawn((
                        ScenarioCountHint,
                        Text::new(format!("{}/{} completados", done, total)),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgb(0.4, 0.7, 0.4)),
                    ));
                });

                // Divisor
                panel.spawn((
                    Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ));

                // ── Lista de cenários ──────────────────────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.0),
                    width: Val::Percent(100.0),
                    overflow: Overflow::clip(),
                    ..default()
                })
                .with_children(|list| {
                    for scenario in all_scenarios() {
                        let unlocked = progress.is_unlocked(scenario.id);
                        let completed = progress.is_completed(scenario.id);

                        let is_boss = scenario.boss.is_some();
                        let (label, color) = if !unlocked {
                            let txt = format!(
                                "  ·  {:>2}. {:<32} [BLOQUEADO]",
                                scenario.id, scenario.name
                            );
                            (txt, Color::srgb(0.25, 0.25, 0.3))
                        } else if completed {
                            let txt = format!(
                                "  ✓  {:>2}. {:<32} [{}]",
                                scenario.id, scenario.name, scenario.region.name()
                            );
                            (txt, Color::srgb(0.0, 4.0, 1.0))
                        } else {
                            let star = if is_boss { "★" } else { "◉" };
                            let txt = format!(
                                "  {}  {:>2}. {:<32} [{}]",
                                star, scenario.id, scenario.name, scenario.region.name()
                            );
                            (txt, Color::srgb(0.7, 0.7, 0.7))
                        };

                        list.spawn((
                            ScenarioRow(scenario.id),
                            Text::new(label),
                            TextFont { font_size: 13.0, ..default() },
                            TextColor(color),
                        ));
                    }
                });

                // Divisor
                panel.spawn((
                    Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ));

                // ── Descrição do cenário selecionado ──────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|detail| {
                    detail.spawn((
                        ScenarioTitleHint,
                        Text::new(""),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(8.0, 8.0, 4.0)),
                    ));
                    detail.spawn((
                        ScenarioDescHint,
                        Text::new(""),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
            });
        });
}

pub fn despawn_scenario_select(
    mut commands: Commands,
    root_q: Query<Entity, With<ScenarioSelectRoot>>,
) {
    for e in root_q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// ── Input ─────────────────────────────────────────────────────────────────────

pub fn scenario_select_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedScenario>,
    progress: Res<CampaignProgress>,
    mut active: ResMut<ActiveScenario>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let scenarios = all_scenarios();
    let count = scenarios.len();

    let up   = keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp);
    let down = keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown);

    const MAX_VIS: usize = 18;
    if up && selected.id > 1 {
        selected.id -= 1;
        if (selected.id as usize).saturating_sub(1) < selected.scroll_offset {
            selected.scroll_offset = (selected.id as usize).saturating_sub(1);
        }
    }
    if down && selected.id < count as u32 {
        selected.id += 1;
        if selected.id as usize > selected.scroll_offset + MAX_VIS {
            selected.scroll_offset = selected.id as usize - MAX_VIS;
        }
    }

    if keys.just_pressed(KeyCode::Enter) {
        if progress.is_unlocked(selected.id) {
            active.id = selected.id;
            next_state.set(GameState::Playing);
        }
    }
}

// ── Atualização da UI ─────────────────────────────────────────────────────────

pub fn update_scenario_select_ui(
    selected: Res<SelectedScenario>,
    progress: Res<CampaignProgress>,
    mut row_q: Query<(&ScenarioRow, &mut Text, &mut TextColor)>,
    mut count_q: Query<&mut Text, (With<ScenarioCountHint>, Without<ScenarioRow>, Without<ScenarioTitleHint>, Without<ScenarioDescHint>)>,
    mut title_q: Query<(&mut Text, &mut TextColor), (With<ScenarioTitleHint>, Without<ScenarioRow>, Without<ScenarioCountHint>, Without<ScenarioDescHint>)>,
    mut desc_q: Query<&mut Text, (With<ScenarioDescHint>, Without<ScenarioRow>, Without<ScenarioCountHint>, Without<ScenarioTitleHint>)>,
) {
    // Atualiza rows
    for (row, mut text, mut color) in row_q.iter_mut() {
        let id = row.0;
        let scenario = all_scenarios().iter().find(|s| s.id == id).unwrap();
        let unlocked = progress.is_unlocked(id);
        let completed = progress.is_completed(id);
        let is_selected = id == selected.id;
        let is_boss = scenario.boss.is_some();

        let (label, col) = if !unlocked {
            let cursor = if is_selected { "▶" } else { " " };
            (format!("  {} ·  {:>2}. {:<32} [BLOQUEADO]", cursor, id, scenario.name),
             if is_selected { Color::srgb(0.5, 0.5, 0.6) } else { Color::srgb(0.25, 0.25, 0.3) })
        } else if completed {
            let cursor = if is_selected { "▶" } else { " " };
            (format!("  {} ✓ {:>2}. {:<32} [{}]", cursor, id, scenario.name, scenario.region.name()),
             if is_selected { Color::srgb(0.0, 8.0, 3.0) } else { Color::srgb(0.0, 4.0, 1.0) })
        } else {
            let cursor = if is_selected { "▶" } else { " " };
            let star = if is_boss { "★" } else { "◉" };
            (format!("  {} {} {:>2}. {:<32} [{}]", cursor, star, id, scenario.name, scenario.region.name()),
             if is_selected { Color::srgb(8.0, 8.0, 4.0) } else { Color::srgb(0.7, 0.7, 0.7) })
        };

        **text = label;
        *color = TextColor(col);
    }

    // Contador no header
    if let Ok(mut txt) = count_q.get_single_mut() {
        **txt = format!("{}/{} completados", progress.completed.len(), all_scenarios().len());
    }

    // Título + status do cenário selecionado
    if let Some(def) = all_scenarios().iter().find(|s| s.id == selected.id) {
        if let Ok((mut txt, mut col)) = title_q.get_single_mut() {
            let status = if !progress.is_unlocked(selected.id) {
                "BLOQUEADO".to_string()
            } else if progress.is_completed(selected.id) {
                "Completado ✓".to_string()
            } else {
                format!("Meta: {} kills{}", def.kill_goal,
                    if def.boss.is_some() { " + BOSS" } else { "" })
            };
            **txt = format!("[{}]  {}  —  {}", def.region.name(), def.name, status);
            *col = TextColor(def.region.color());
        }

        if let Ok(mut txt) = desc_q.get_single_mut() {
            **txt = def.description.to_string();
        }
    }
}

// ── Overlay de vitória ────────────────────────────────────────────────────────

pub fn spawn_win_overlay(mut commands: Commands) {
    commands.spawn((
        ScenarioWinOverlay,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(35.0),
            left: Val::Percent(0.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("CENÁRIO COMPLETO!"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(0.0, 8.0, 2.0)),
        ));
    });
}

pub fn despawn_win_overlay(
    mut commands: Commands,
    q: Query<Entity, With<ScenarioWinOverlay>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn update_win_overlay(
    win_timer: Res<ScenarioWinTimer>,
    mut commands: Commands,
    overlay_q: Query<Entity, With<ScenarioWinOverlay>>,
) {
    let has_overlay = !overlay_q.is_empty();
    let win_active = win_timer.0.is_some();

    if win_active && !has_overlay {
        spawn_win_overlay(commands);
    } else if !win_active && has_overlay {
        for e in overlay_q.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}
