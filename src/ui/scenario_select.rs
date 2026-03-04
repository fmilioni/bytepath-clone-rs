use bevy::prelude::*;

use crate::campaign::components::{CampaignProgress, Region, SelectedScenario};
use crate::campaign::{ActiveScenario, ScenarioWinTimer};
use crate::campaign::data::all_scenarios;
use crate::states::GameState;
use crate::vfx::components::Star;

// ── Marcadores ────────────────────────────────────────────────────────────────

/// Raiz de todas as entidades Mesh2d do mapa (despawn_recursive limpa tudo).
#[derive(Component)]
pub struct ScenarioMapRoot;

/// Raiz da UI overlay (painel de info no rodapé).
#[derive(Component)]
pub struct ScenarioSelectRoot;

/// Nó de cenário no mapa.
#[derive(Component)]
pub struct ScenarioNode(pub u32);

/// Cursor pulsante que indica o cenário selecionado.
#[derive(Component)]
pub struct ScenarioCursor;

#[derive(Component)]
pub struct ScenarioCountHint;

#[derive(Component)]
pub struct ScenarioTitleHint;

#[derive(Component)]
pub struct ScenarioDescHint;

#[derive(Component)]
pub struct ScenarioWinOverlay;

// ── Posições dos nós no mapa (espaço de mundo 1280×720) ───────────────────────

pub fn scenario_map_pos(id: u32) -> Vec2 {
    match id {
        // FRONTIER — baixo-esquerdo, curva suave
        1  => Vec2::new(-520.0, -200.0),
        2  => Vec2::new(-430.0, -245.0),
        3  => Vec2::new(-330.0, -225.0),
        4  => Vec2::new(-415.0, -145.0),
        5  => Vec2::new(-305.0, -145.0),
        // NEBULA — superior-esquerdo
        6  => Vec2::new(-475.0,  60.0),
        7  => Vec2::new(-375.0, 115.0),
        8  => Vec2::new(-265.0,  95.0),
        9  => Vec2::new(-360.0,  15.0),
        10 => Vec2::new(-250.0,  25.0),
        11 => Vec2::new(-155.0,  65.0),
        // ASTEROID BELT — centro
        12 => Vec2::new( -65.0, -115.0),
        13 => Vec2::new(  30.0, -165.0),
        14 => Vec2::new( 120.0, -115.0),
        15 => Vec2::new(  30.0,  -40.0),
        16 => Vec2::new( 120.0,  -40.0),
        17 => Vec2::new( 210.0,  -70.0),
        // VOID — direita-centro
        18 => Vec2::new( 290.0,  60.0),
        19 => Vec2::new( 370.0, 125.0),
        20 => Vec2::new( 460.0,  70.0),
        21 => Vec2::new( 375.0,  10.0),
        22 => Vec2::new( 465.0,  20.0),
        // CORE — extremo direito
        23 => Vec2::new( 535.0, -140.0),
        24 => Vec2::new( 560.0,  -55.0),
        25 => Vec2::new( 555.0,   30.0),
        _  => Vec2::ZERO,
    }
}

// (centro, raio, cor do halo)
fn region_halo(region: Region) -> (Vec2, f32, Color) {
    match region {
        Region::Frontier     => (Vec2::new(-415.0, -190.0), 170.0, Color::srgba(0.0, 0.7, 0.2,  0.06)),
        Region::Nebula       => (Vec2::new(-338.0,  58.0),  195.0, Color::srgba(0.6, 0.0, 0.9,  0.07)),
        Region::AsteroidBelt => (Vec2::new(  58.0, -90.0),  185.0, Color::srgba(0.8, 0.35, 0.0, 0.06)),
        Region::Void         => (Vec2::new( 388.0,  52.0),  175.0, Color::srgba(0.0, 0.2,  0.9, 0.07)),
        Region::Core         => (Vec2::new( 550.0, -55.0),  145.0, Color::srgba(1.0, 0.12, 0.0, 0.08)),
    }
}

fn region_label_pos(region: Region) -> Vec2 {
    match region {
        // Y mínimo seguro: -270 (abaixo ficaria sob o painel UI de 84px)
        Region::Frontier     => Vec2::new(-458.0, -268.0),
        Region::Nebula       => Vec2::new(-400.0,  166.0),
        Region::AsteroidBelt => Vec2::new(  48.0, -224.0),
        Region::Void         => Vec2::new( 368.0,  176.0),
        Region::Core         => Vec2::new( 528.0, -200.0),
    }
}

/// Cor dimida de um nó desbloqueado mas não completado.
fn dim_color(region: Region) -> Color {
    match region {
        Region::Frontier     => Color::srgb(0.0, 1.8, 0.6),
        Region::Nebula       => Color::srgb(1.2, 0.0, 2.4),
        Region::AsteroidBelt => Color::srgb(1.8, 0.9, 0.0),
        Region::Void         => Color::srgb(0.0, 0.6, 1.8),
        Region::Core         => Color::srgb(2.4, 0.2, 0.0),
    }
}

/// Cor do cursor (semi-transparente) para cada região.
fn cursor_color(region: Region) -> Color {
    match region {
        Region::Frontier     => Color::srgba(0.0, 3.0, 1.0, 0.35),
        Region::Nebula       => Color::srgba(2.0, 0.0, 4.0, 0.35),
        Region::AsteroidBelt => Color::srgba(3.0, 1.5, 0.0, 0.35),
        Region::Void         => Color::srgba(0.0, 1.0, 3.0, 0.35),
        Region::Core         => Color::srgba(4.0, 0.25, 0.0, 0.35),
    }
}

// ── Helper: linha entre dois pontos ──────────────────────────────────────────

fn spawn_line(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    a: Vec2,
    b: Vec2,
    color: Color,
    parent: Entity,
) {
    let diff = b - a;
    let length = diff.length();
    let angle = diff.y.atan2(diff.x);
    let mid = (a + b) * 0.5;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(length, 1.5))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
        Transform::from_xyz(mid.x, mid.y, 1.0)
            .with_rotation(Quat::from_rotation_z(angle)),
    ))
    .set_parent(parent);
}

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_scenario_select(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    progress: Res<CampaignProgress>,
    mut selected: ResMut<SelectedScenario>,
) {
    if selected.id == 0 {
        selected.id = 1;
    }

    // ── Raiz das entidades Mesh2d ──────────────────────────────────────────
    let map_root = commands.spawn((
        ScenarioMapRoot,
        Transform::default(),
        Visibility::default(),
    )).id();

    // ── Halos nebulares por região ─────────────────────────────────────────
    for region in [Region::Frontier, Region::Nebula, Region::AsteroidBelt, Region::Void, Region::Core] {
        let (center, radius, color) = region_halo(region);
        // Halo externo amplo
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_xyz(center.x, center.y, 0.15),
        )).set_parent(map_root);
        // Núcleo mais concentrado
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(radius * 0.45))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color({
                let c = color.to_srgba();
                Color::srgba(c.red, c.green, c.blue, c.alpha * 0.9)
            }))),
            Transform::from_xyz(center.x, center.y, 0.16),
        )).set_parent(map_root);
    }

    // ── Linhas de conexão ─────────────────────────────────────────────────
    for id in 1u32..25 {
        let a = scenario_map_pos(id);
        let b = scenario_map_pos(id + 1);
        let both_unlocked = progress.is_unlocked(id) && progress.is_unlocked(id + 1);
        let color = if both_unlocked {
            Color::srgba(0.4, 0.4, 0.55, 0.35)
        } else {
            Color::srgba(0.1, 0.1, 0.15, 0.18)
        };
        spawn_line(&mut commands, &mut meshes, &mut materials, a, b, color, map_root);
    }

    // ── Nós de cenário ────────────────────────────────────────────────────
    for scenario in all_scenarios() {
        let pos = scenario_map_pos(scenario.id);
        let unlocked = progress.is_unlocked(scenario.id);
        let completed = progress.is_completed(scenario.id);
        let is_boss = scenario.boss.is_some();

        let radius = if is_boss { 11.0_f32 } else { 8.0_f32 };
        let color = if !unlocked {
            Color::srgba(0.15, 0.15, 0.2, 0.4)
        } else if completed {
            scenario.region.color()
        } else {
            dim_color(scenario.region)
        };

        commands.spawn((
            ScenarioNode(scenario.id),
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_xyz(pos.x, pos.y, 2.0),
        )).set_parent(map_root);
    }

    // ── Cursor de seleção ─────────────────────────────────────────────────
    let sel_region = all_scenarios()
        .iter()
        .find(|s| s.id == selected.id)
        .map(|s| s.region)
        .unwrap_or(Region::Frontier);

    let cursor_pos = scenario_map_pos(selected.id);
    commands.spawn((
        ScenarioCursor,
        Mesh2d(meshes.add(Circle::new(16.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(cursor_color(sel_region)))),
        Transform::from_xyz(cursor_pos.x, cursor_pos.y, 2.5),
    )).set_parent(map_root);

    // ── Labels de região (Text2d world-space) ─────────────────────────────
    for region in [Region::Frontier, Region::Nebula, Region::AsteroidBelt, Region::Void, Region::Core] {
        let pos = region_label_pos(region);
        let c = region.color().to_srgba();
        commands.spawn((
            Text2d::new(region.name()),
            TextFont { font_size: 10.0, ..default() },
            TextColor(Color::srgba(c.red * 0.5, c.green * 0.5, c.blue * 0.5, 0.6)),
            Transform::from_xyz(pos.x, pos.y, 1.8),
        )).set_parent(map_root);
    }

    // ── Painel de info UI (Node) ──────────────────────────────────────────
    let done = progress.completed.len();
    let total = all_scenarios().len();

    commands.spawn((
        ScenarioSelectRoot,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(84.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                left: Val::Px(22.0),
                right: Val::Px(22.0),
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            row_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.04, 0.90)),
    ))
    .with_children(|panel| {
        // Linha superior: título + contador
        panel.spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                ScenarioTitleHint,
                Text::new(""),
                TextFont { font_size: 15.0, ..default() },
                TextColor(Color::srgb(8.0, 8.0, 4.0)),
            ));
            row.spawn((
                ScenarioCountHint,
                Text::new(format!("{}/{} completados", done, total)),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.4, 0.7, 0.4)),
            ));
        });
        // Linha inferior: descrição + controles
        panel.spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                ScenarioDescHint,
                Text::new(""),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.55, 0.55, 0.65)),
            ));
            row.spawn((
                Text::new("← → navegar   ENTER jogar   ESC voltar"),
                TextFont { font_size: 11.0, ..default() },
                TextColor(Color::srgb(0.28, 0.28, 0.38)),
            ));
        });
    });
}

pub fn despawn_scenario_select(
    mut commands: Commands,
    map_q: Query<Entity, With<ScenarioMapRoot>>,
    ui_q: Query<Entity, With<ScenarioSelectRoot>>,
) {
    for e in map_q.iter().chain(ui_q.iter()) {
        if let Some(cmd) = commands.get_entity(e) {
            cmd.despawn_recursive();
        }
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
    let count = all_scenarios().len() as u32;
    let prev = keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft);
    let next = keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight);

    if prev && selected.id > 1 {
        selected.id -= 1;
    }
    if next && selected.id < count {
        selected.id += 1;
    }

    if keys.just_pressed(KeyCode::Enter) && progress.is_unlocked(selected.id) {
        active.id = selected.id;
        next_state.set(GameState::Playing);
    }
}

// ── Atualização do mapa ───────────────────────────────────────────────────────

pub fn update_scenario_map(
    time: Res<Time>,
    selected: Res<SelectedScenario>,
    progress: Res<CampaignProgress>,
    mut cursor_q: Query<(&mut Transform, &MeshMaterial2d<ColorMaterial>), With<ScenarioCursor>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut title_q: Query<
        (&mut Text, &mut TextColor),
        (With<ScenarioTitleHint>, Without<ScenarioDescHint>, Without<ScenarioCountHint>),
    >,
    mut desc_q: Query<
        &mut Text,
        (With<ScenarioDescHint>, Without<ScenarioTitleHint>, Without<ScenarioCountHint>),
    >,
    mut count_q: Query<
        &mut Text,
        (With<ScenarioCountHint>, Without<ScenarioTitleHint>, Without<ScenarioDescHint>),
    >,
) {
    let t = time.elapsed_secs();

    // Cursor: posição + pulso de escala + cor por região
    if let Ok((mut tf, mat_handle)) = cursor_q.get_single_mut() {
        let pos = scenario_map_pos(selected.id);
        let pulse = 1.0 + (t * 3.5).sin() * 0.22;
        tf.translation = Vec3::new(pos.x, pos.y, 2.5);
        tf.scale = Vec3::splat(pulse);

        let region = all_scenarios()
            .iter()
            .find(|s| s.id == selected.id)
            .map(|s| s.region)
            .unwrap_or(Region::Frontier);

        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            let alpha_base = if progress.is_unlocked(selected.id) { 0.30 } else { 0.15 };
            let alpha = alpha_base + (t * 3.5).sin() * 0.08;
            mat.color = if progress.is_unlocked(selected.id) {
                let c = cursor_color(region).to_srgba();
                Color::srgba(c.red, c.green, c.blue, alpha.max(0.10))
            } else {
                Color::srgba(0.4, 0.4, 0.5, alpha.max(0.10))
            };
        }
    }

    // Painel de info
    if let Some(def) = all_scenarios().iter().find(|s| s.id == selected.id) {
        if let Ok((mut txt, mut col)) = title_q.get_single_mut() {
            let status = if !progress.is_unlocked(selected.id) {
                "BLOQUEADO".to_string()
            } else if progress.is_completed(selected.id) {
                "Completado ✓".to_string()
            } else {
                format!(
                    "Meta: {} kills{}",
                    def.kill_goal,
                    if def.boss.is_some() { " + BOSS" } else { "" }
                )
            };
            **txt = format!("[{}]  {}  —  {}", def.region.name(), def.name, status);
            *col = TextColor(if progress.is_unlocked(selected.id) {
                def.region.color()
            } else {
                Color::srgb(0.4, 0.4, 0.5)
            });
        }
        if let Ok(mut txt) = desc_q.get_single_mut() {
            **txt = def.description.to_string();
        }
    }

    if let Ok(mut txt) = count_q.get_single_mut() {
        **txt = format!("{}/{} completados", progress.completed.len(), all_scenarios().len());
    }
}

// ── Tint das estrelas conforme região navegada ────────────────────────────────

/// Aplica tint imediato ao entrar em ScenarioSelect (ignora cache).
pub fn init_star_tint(
    selected: Res<SelectedScenario>,
    star_q: Query<&MeshMaterial2d<ColorMaterial>, With<Star>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let region = all_scenarios()
        .iter()
        .find(|s| s.id == selected.id)
        .map(|s| s.region)
        .unwrap_or(Region::Frontier);
    let tint = region.star_tint();
    for mat_handle in star_q.iter() {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.color = tint;
        }
    }
}

pub fn tint_stars_scenario_select(
    selected: Res<SelectedScenario>,
    star_q: Query<&MeshMaterial2d<ColorMaterial>, With<Star>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut last_id: Local<u32>,
) {
    if selected.id == *last_id {
        return;
    }
    *last_id = selected.id;

    let region = all_scenarios()
        .iter()
        .find(|s| s.id == selected.id)
        .map(|s| s.region)
        .unwrap_or(Region::Frontier);

    let tint = region.star_tint();
    for mat_handle in star_q.iter() {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.color = tint;
        }
    }
}

// ── Overlay de vitória ────────────────────────────────────────────────────────

pub fn spawn_win_overlay(mut commands: Commands) {
    commands
        .spawn((
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
