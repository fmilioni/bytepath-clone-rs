use bevy::prelude::*;

use crate::campaign::components::{ActiveScenario, ScenarioKillCount};
use crate::combat::components::{Energy, Health, Shield};
use crate::player::components::Player;
use crate::shop::components::Credits;
use crate::skill_tree::components::PlayerSkills;

/// Marcadores para os textos do HUD.
#[derive(Component)] pub struct HudHp;
#[derive(Component)] pub struct HudShield;
#[derive(Component)] pub struct HudEnergy;

/// Painel de progresso de cenário (parte inferior).
#[derive(Component)] pub struct HudScenarioBar;

/// Créditos e pontos de skill (canto superior direito).
#[derive(Component)] pub struct HudEconomy;

/// Root do HUD (para cleanup ao sair de Playing).
#[derive(Component)]
pub struct HudRoot;

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_hud(mut commands: Commands) {
    // Container raiz (full screen)
    commands
        .spawn((HudRoot, Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(12.0)),
            ..default()
        }))
        .with_children(|root| {
            // ── Topo: stats do player ─────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                ..default()
            })
            .with_children(|top| {
                // Stats do player (esquerda)
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|stats| {
                    spawn_hud_row(stats, "HP:", HudHp, Color::srgb(0.0, 8.0, 2.0));
                    spawn_hud_row(stats, "SHIELD:", HudShield, Color::srgb(0.0, 2.0, 8.0));
                    spawn_hud_row(stats, "ENERGY:", HudEnergy, Color::srgb(8.0, 6.0, 0.0));
                });

                // Créditos + skill points (direita)
                top.spawn((
                    HudEconomy,
                    Text::new("CR:0  SP:0"),
                    TextFont { font_size: 13.0, ..default() },
                    TextColor(Color::srgb(0.5, 0.5, 0.6)),
                ));
            });

            // ── Base: progresso do cenário ─────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                ..default()
            })
            .with_children(|bottom| {
                bottom.spawn((
                    HudScenarioBar,
                    Text::new(""),
                    TextFont { font_size: 13.0, ..default() },
                    TextColor(Color::srgb(0.5, 0.5, 0.6)),
                ));
            });
        });
}

fn spawn_hud_row<M: Component>(parent: &mut ChildBuilder, label: &str, marker: M, color: Color) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
            row.spawn((
                marker,
                Text::new("100"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(color),
            ));
        });
}

// ── Atualização ───────────────────────────────────────────────────────────────

pub fn update_hud(
    player_query: Query<(&Health, &Shield, &Energy), With<Player>>,
    mut hp_text: Query<&mut Text, (With<HudHp>, Without<HudShield>, Without<HudEnergy>, Without<HudScenarioBar>, Without<HudEconomy>)>,
    mut shield_text: Query<&mut Text, (With<HudShield>, Without<HudHp>, Without<HudEnergy>, Without<HudScenarioBar>, Without<HudEconomy>)>,
    mut energy_text: Query<&mut Text, (With<HudEnergy>, Without<HudHp>, Without<HudShield>, Without<HudScenarioBar>, Without<HudEconomy>)>,
    mut scenario_text: Query<&mut Text, (With<HudScenarioBar>, Without<HudHp>, Without<HudShield>, Without<HudEnergy>, Without<HudEconomy>)>,
    mut economy_text: Query<&mut Text, (With<HudEconomy>, Without<HudHp>, Without<HudShield>, Without<HudEnergy>, Without<HudScenarioBar>)>,
    kill_count: Res<ScenarioKillCount>,
    active: Res<ActiveScenario>,
    credits: Res<Credits>,
    skills: Res<PlayerSkills>,
) {
    // Stats do player
    let Ok((health, shield, energy)) = player_query.get_single() else { return; };

    if let Ok(mut text) = hp_text.get_single_mut() {
        **text = format!("{:.0}/{:.0}", health.current, health.max);
    }
    if let Ok(mut text) = shield_text.get_single_mut() {
        **text = format!("{:.0}/{:.0}", shield.current, shield.max);
    }
    if let Ok(mut text) = energy_text.get_single_mut() {
        **text = format!("{:.0}/{:.0}", energy.current, energy.max);
    }

    // Economia (top-right)
    if let Ok(mut text) = economy_text.get_single_mut() {
        **text = format!("CR:{}  SP:{}", credits.0, skills.skill_points);
    }

    // Barra de progresso do cenário (bottom)
    if let Ok(mut text) = scenario_text.get_single_mut() {
        if active.id == 0 {
            **text = String::new();
        } else {
            let def = active.def();
            **text = format_scenario_bar(def, &kill_count);
        }
    }
}

fn format_scenario_bar(
    def: &crate::campaign::components::ScenarioDef,
    kc: &ScenarioKillCount,
) -> String {
    const BAR_LEN: usize = 14;

    if def.boss.is_some() {
        // Cenário de boss
        if kc.boss_killed {
            format!(
                "[{}]  {}  ({}/{})  ✓ BOSS ELIMINADO",
                def.region.name(), def.name, def.id, 25
            )
        } else if kc.boss_spawned {
            format!(
                "[{}]  {}  ({}/{})  ★ BOSS ATIVO",
                def.region.name(), def.name, def.id, 25
            )
        } else {
            let filled = ((kc.kills as f32 / kc.goal as f32) * BAR_LEN as f32).round() as usize;
            let filled = filled.min(BAR_LEN);
            let bar: String = (0..BAR_LEN).map(|i| if i < filled { '█' } else { '░' }).collect();
            format!(
                "[{}]  {}  ({}/{})  {}  {}/{}  → boss",
                def.region.name(), def.name, def.id, 25, bar, kc.kills, kc.goal
            )
        }
    } else {
        let filled = ((kc.kills as f32 / kc.goal as f32) * BAR_LEN as f32).round() as usize;
        let filled = filled.min(BAR_LEN);
        let bar: String = (0..BAR_LEN).map(|i| if i < filled { '█' } else { '░' }).collect();
        format!(
            "[{}]  {}  ({}/{})  {}  {}/{}",
            def.region.name(), def.name, def.id, 25, bar, kc.kills, kc.goal
        )
    }
}
