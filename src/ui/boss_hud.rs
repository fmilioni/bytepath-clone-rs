use bevy::prelude::*;

use crate::bosses::components::{ActiveBoss, Boss, BossKind};
use crate::combat::components::Health;

/// Container da barra de HP do boss (para mostrar/ocultar).
#[derive(Component)]
pub struct BossHudRoot;

/// Texto do nome do boss.
#[derive(Component)]
pub struct BossNameText;

/// Barra de progresso (nó que encolhe horizontalmente).
#[derive(Component)]
pub struct BossHpFill;

/// Texto de HP numérico.
#[derive(Component)]
pub struct BossHpText;

pub fn spawn_boss_hud(mut commands: Commands) {
    // Container centrado no topo da tela
    commands
        .spawn((
            BossHudRoot,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(14.0),
                left: Val::Percent(20.0),
                width: Val::Percent(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                ..default()
            },
            Visibility::Hidden, // oculto até um boss aparecer
        ))
        .with_children(|p| {
            // Nome do boss
            p.spawn((
                BossNameText,
                Text::new(""),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(8.0, 4.0, 0.0)),
            ));

            // Fundo da barra
            p.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(12.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.05, 0.05)),
            ))
            .with_children(|bar| {
                // Preenchimento da barra (encolhe pela direita)
                bar.spawn((
                    BossHpFill,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(6.0, 0.5, 0.0)),
                ));
            });

            // HP numérico
            p.spawn((
                BossHpText,
                Text::new(""),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.8, 0.5, 0.3)),
            ));
        });
}

pub fn update_boss_hud(
    active: Res<ActiveBoss>,
    boss_q: Query<(&Health, &BossKind), With<Boss>>,
    mut root_q: Query<&mut Visibility, With<BossHudRoot>>,
    mut name_q: Query<&mut Text, (With<BossNameText>, Without<BossHpText>)>,
    mut hp_text_q: Query<&mut Text, (With<BossHpText>, Without<BossNameText>)>,
    mut fill_q: Query<&mut Node, With<BossHpFill>>,
) {
    let Ok(mut vis) = root_q.get_single_mut() else { return; };

    let Some(boss_entity) = active.0 else {
        *vis = Visibility::Hidden;
        return;
    };

    let Ok((health, kind)) = boss_q.get(boss_entity) else {
        *vis = Visibility::Hidden;
        return;
    };

    *vis = Visibility::Visible;

    let frac = (health.current / health.max).clamp(0.0, 1.0);

    if let Ok(mut name) = name_q.get_single_mut() {
        let boss_name = match kind {
            BossKind::Sentinel => "SENTINEL",
            BossKind::Swarmmother => "SWARMMOTHER",
            BossKind::Dreadnought => "DREADNOUGHT",
            BossKind::Phantom => "PHANTOM",
            BossKind::Singularity => "SINGULARITY",
        };
        **name = boss_name.to_string();
    }
    if let Ok(mut fill) = fill_q.get_single_mut() {
        fill.width = Val::Percent(frac * 100.0);
    }
    if let Ok(mut txt) = hp_text_q.get_single_mut() {
        **txt = format!("{:.0} / {:.0}", health.current.max(0.0), health.max);
    }
}
