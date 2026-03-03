use bevy::prelude::*;

use crate::player::ship_classes::{SelectedShipClass, ALL_CLASSES};
use crate::states::GameState;

// ── Marcadores ───────────────────────────────────────────────────────────────

#[derive(Component)] pub struct ShipSelectRoot;
#[derive(Component)] pub struct ShipPreviewMesh;
#[derive(Component)] pub struct ShipNameText;
#[derive(Component)] pub struct ShipDescText;
#[derive(Component)] pub struct ShipIndexText;

// Textos de stats individuais
#[derive(Component)] pub struct StatSpeed;
#[derive(Component)] pub struct StatHp;
#[derive(Component)] pub struct StatShield;
#[derive(Component)] pub struct StatEnergy;
#[derive(Component)] pub struct StatDamage;
#[derive(Component)] pub struct StatFireRate;
#[derive(Component)] pub struct StatCargo;

// ── Spawn ────────────────────────────────────────────────────────────────────

pub fn spawn_ship_select(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    selected: Res<SelectedShipClass>,
) {
    let class = selected.0;
    let data = class.data();

    // Preview da nave no centro da tela (em espaço de mundo, câmera em z=10)
    commands.spawn((
        ShipPreviewMesh,
        Mesh2d(class.build_mesh(&mut meshes)),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(data.color))),
        Transform::from_xyz(0.0, 30.0, 0.0).with_scale(Vec3::splat(2.5)),
    ));

    // UI overlay
    commands
        .spawn((
            ShipSelectRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
        ))
        .with_children(|root| {
            // ── Topo: título + índice ─────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            })
            .with_children(|top| {
                top.spawn((
                    Text::new("SELECT YOUR SHIP"),
                    TextFont { font_size: 22.0, ..default() },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                ));
                top.spawn((
                    ShipIndexText,
                    Text::new(format!("{} / {}", 1, ALL_CLASSES.len())),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.4, 0.4, 0.4)),
                ));
            });

            // ── Centro: nome + descrição (alinhado à esquerda) ─
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|mid| {
                mid.spawn((
                    ShipNameText,
                    Text::new(data.name),
                    TextFont { font_size: 40.0, ..default() },
                    TextColor(data.color),
                ));
                mid.spawn((
                    ShipDescText,
                    Text::new(data.description),
                    TextFont { font_size: 15.0, ..default() },
                    TextColor(Color::srgb(0.55, 0.55, 0.55)),
                ));
            });

            // ── Stats no canto inferior direito ──────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..default()
            })
            .with_children(|bottom| {
                // Instruções à esquerda
                bottom.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    ..default()
                })
                .with_children(|instr| {
                    for line in ["A / ← → / D  —  Mudar nave", "ENTER  —  Jogar", "ESC  —  Sair"] {
                        instr.spawn((
                            Text::new(line),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::srgb(0.4, 0.4, 0.4)),
                        ));
                    }
                });

                // Stats à direita
                bottom.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    align_items: AlignItems::FlexEnd,
                    ..default()
                })
                .with_children(|stats_col| {
                    spawn_stat_row(stats_col, "Velocidade", StatSpeed,
                        format!("{:.0}", data.speed), Color::srgb(0.0, 6.0, 2.0));
                    spawn_stat_row(stats_col, "HP", StatHp,
                        format!("{:.0}", data.max_hp), Color::srgb(0.0, 8.0, 2.0));
                    spawn_stat_row(stats_col, "Escudo", StatShield,
                        format!("{:.0}", data.max_shield), Color::srgb(0.0, 2.0, 8.0));
                    spawn_stat_row(stats_col, "Energia", StatEnergy,
                        format!("{:.0}", data.max_energy), Color::srgb(8.0, 6.0, 0.0));
                    spawn_stat_row(stats_col, "Dano ×", StatDamage,
                        format!("{:.1}×", data.damage_multiplier), Color::srgb(8.0, 0.5, 0.0));
                    spawn_stat_row(stats_col, "Cadência", StatFireRate,
                        format!("{:.1}/s", data.fire_rate), Color::srgb(6.0, 0.0, 6.0));
                    spawn_stat_row(stats_col, "Slots Cargo", StatCargo,
                        format!("{}", data.cargo_slots), Color::srgb(3.0, 3.0, 3.0));
                });
            });
        });
}

fn spawn_stat_row<M: Component>(
    parent: &mut ChildBuilder,
    label: &str,
    marker: M,
    value: String,
    color: Color,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(10.0),
        ..default()
    })
    .with_children(|row| {
        row.spawn((
            Text::new(label),
            TextFont { font_size: 13.0, ..default() },
            TextColor(Color::srgb(0.45, 0.45, 0.45)),
        ));
        row.spawn((
            marker,
            Text::new(value),
            TextFont { font_size: 13.0, ..default() },
            TextColor(color),
        ));
    });
}

// ── Cleanup ──────────────────────────────────────────────────────────────────

pub fn despawn_ship_select(
    mut commands: Commands,
    root_query: Query<Entity, With<ShipSelectRoot>>,
    preview_query: Query<Entity, With<ShipPreviewMesh>>,
) {
    for entity in root_query.iter().chain(preview_query.iter()) {
        commands.entity(entity).despawn_recursive();
    }
}

// ── Input + Atualização de UI ────────────────────────────────────────────────

pub fn ship_select_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedShipClass>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        selected.0 = selected.0.prev();
    }
    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        selected.0 = selected.0.next();
    }
    if keys.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::ScenarioSelect);
    }
}

pub fn update_ship_select_ui(
    selected: Res<SelectedShipClass>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut preview_query: Query<
        (&mut Mesh2d, &mut MeshMaterial2d<ColorMaterial>, &mut Transform),
        With<ShipPreviewMesh>,
    >,
    mut name_query: Query<&mut Text, (With<ShipNameText>, Without<ShipDescText>, Without<ShipIndexText>, Without<StatSpeed>, Without<StatHp>, Without<StatShield>, Without<StatEnergy>, Without<StatDamage>, Without<StatFireRate>, Without<StatCargo>)>,
    mut desc_query: Query<&mut Text, (With<ShipDescText>, Without<ShipNameText>, Without<ShipIndexText>, Without<StatSpeed>, Without<StatHp>, Without<StatShield>, Without<StatEnergy>, Without<StatDamage>, Without<StatFireRate>, Without<StatCargo>)>,
    mut index_query: Query<&mut Text, (With<ShipIndexText>, Without<ShipNameText>, Without<ShipDescText>, Without<StatSpeed>, Without<StatHp>, Without<StatShield>, Without<StatEnergy>, Without<StatDamage>, Without<StatFireRate>, Without<StatCargo>)>,
    mut speed_q: Query<(&mut Text, &mut TextColor), (With<StatSpeed>, Without<StatHp>, Without<StatShield>, Without<StatEnergy>, Without<StatDamage>, Without<StatFireRate>, Without<StatCargo>)>,
    mut hp_q: Query<(&mut Text, &mut TextColor), (With<StatHp>, Without<StatSpeed>, Without<StatShield>, Without<StatEnergy>, Without<StatDamage>, Without<StatFireRate>, Without<StatCargo>)>,
    mut shield_q: Query<(&mut Text, &mut TextColor), (With<StatShield>, Without<StatSpeed>, Without<StatHp>, Without<StatEnergy>, Without<StatDamage>, Without<StatFireRate>, Without<StatCargo>)>,
    mut energy_q: Query<(&mut Text, &mut TextColor), (With<StatEnergy>, Without<StatSpeed>, Without<StatHp>, Without<StatShield>, Without<StatDamage>, Without<StatFireRate>, Without<StatCargo>)>,
    mut damage_q: Query<(&mut Text, &mut TextColor), (With<StatDamage>, Without<StatSpeed>, Without<StatHp>, Without<StatShield>, Without<StatEnergy>, Without<StatFireRate>, Without<StatCargo>)>,
    mut fire_q: Query<(&mut Text, &mut TextColor), (With<StatFireRate>, Without<StatSpeed>, Without<StatHp>, Without<StatShield>, Without<StatEnergy>, Without<StatDamage>, Without<StatCargo>)>,
    mut cargo_q: Query<(&mut Text, &mut TextColor), (With<StatCargo>, Without<StatSpeed>, Without<StatHp>, Without<StatShield>, Without<StatEnergy>, Without<StatDamage>, Without<StatFireRate>)>,
) {
    if !selected.is_changed() {
        return;
    }

    let class = selected.0;
    let d = class.data();
    let idx = ALL_CLASSES.iter().position(|c| *c == class).unwrap_or(0);

    // Atualiza preview de mesh
    if let Ok((mut mesh2d, mut mat, mut transform)) = preview_query.get_single_mut() {
        mesh2d.0 = class.build_mesh(&mut meshes);
        mat.0 = materials.add(ColorMaterial::from_color(d.color));
        transform.scale = Vec3::splat(2.5);
    }

    // Textos de info
    if let Ok(mut t) = name_query.get_single_mut() { **t = d.name.to_string(); }
    if let Ok(mut t) = desc_query.get_single_mut() { **t = d.description.to_string(); }
    if let Ok(mut t) = index_query.get_single_mut() {
        **t = format!("{} / {}", idx + 1, ALL_CLASSES.len());
    }

    // Stats
    if let Ok((mut t, _)) = speed_q.get_single_mut() { **t = format!("{:.0}", d.speed); }
    if let Ok((mut t, _)) = hp_q.get_single_mut()    { **t = format!("{:.0}", d.max_hp); }
    if let Ok((mut t, _)) = shield_q.get_single_mut() { **t = format!("{:.0}", d.max_shield); }
    if let Ok((mut t, _)) = energy_q.get_single_mut() { **t = format!("{:.0}", d.max_energy); }
    if let Ok((mut t, _)) = damage_q.get_single_mut() { **t = format!("{:.1}×", d.damage_multiplier); }
    if let Ok((mut t, _)) = fire_q.get_single_mut()  { **t = format!("{:.1}/s", d.fire_rate); }
    if let Ok((mut t, _)) = cargo_q.get_single_mut() { **t = format!("{}", d.cargo_slots); }
}

/// Rotaciona lentamente o preview da nave na tela de seleção.
pub fn rotate_preview(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<ShipPreviewMesh>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_secs() * 0.8);
    }
}
