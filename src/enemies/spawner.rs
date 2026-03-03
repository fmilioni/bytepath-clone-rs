use bevy::prelude::*;
use rand::Rng;

use crate::campaign::components::ActiveScenario;
use crate::combat::components::{ColliderRadius, Health};
use crate::constants::{HALF_H, HALF_W, Z_ENEMY};

use super::components::{
    BomberData, CircleOrbit, Enemy, EnemyKind, EnemyShield, EnemyShootTimer,
    EnemyStats, MinionSpawnData, SniperState, SplitterData, TeleportData,
};

/// Timer global que controla o spawn periódico de inimigos.
#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

/// Pesos padrão quando não há cenário ativo.
const DEFAULT_WEIGHTS: [u32; 10] = [4, 3, 3, 2, 1, 1, 1, 1, 1, 1];
const DEFAULT_MAX: u32 = 25;

/// Spawna inimigos periodicamente nas bordas da tela.
pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_count: Query<(), With<Enemy>>,
    active_scenario: Res<ActiveScenario>,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.finished() { return; }
    spawn_timer.0.reset();

    // Lê configurações do cenário ativo (ou defaults)
    let (weights, max_enemies, hp_scale, speed_scale) = if active_scenario.id != 0 {
        let def = active_scenario.def();
        (def.enemy_weights, def.max_enemies, def.enemy_hp_scale, def.enemy_speed_scale)
    } else {
        (DEFAULT_WEIGHTS, DEFAULT_MAX, 1.0_f32, 1.0_f32)
    };

    // Limita inimigos simultâneos
    if enemy_count.iter().count() >= max_enemies as usize { return; }

    let mut rng = rand::thread_rng();
    let kind = weighted_kind_pick(&weights, &mut rng);

    let spawn_pos = random_border_pos(&mut rng);
    spawn_enemy_scaled(&mut commands, &mut meshes, &mut materials, kind, spawn_pos, hp_scale, speed_scale);
}

fn weighted_kind_pick(weights: &[u32; 10], rng: &mut impl Rng) -> EnemyKind {
    const KINDS: [EnemyKind; 10] = [
        EnemyKind::Swarmer, EnemyKind::Charger, EnemyKind::Shooter,
        EnemyKind::Circler, EnemyKind::Bomber, EnemyKind::Sniper,
        EnemyKind::ShieldEnemy, EnemyKind::Splitter, EnemyKind::Teleporter,
        EnemyKind::MinionSpawner,
    ];
    let total: u32 = weights.iter().sum();
    if total == 0 { return EnemyKind::Swarmer; }
    let mut roll = rng.gen_range(0..total);
    for (i, &w) in weights.iter().enumerate() {
        if w > 0 && roll < w { return KINDS[i]; }
        roll = roll.saturating_sub(w);
    }
    EnemyKind::Swarmer
}

/// Posição aleatória em uma das 4 bordas da tela.
pub fn random_border_pos(rng: &mut impl Rng) -> Vec2 {
    match rng.gen_range(0..4) {
        0 => Vec2::new(rng.gen_range(-HALF_W..HALF_W), HALF_H + 40.0),
        1 => Vec2::new(rng.gen_range(-HALF_W..HALF_W), -HALF_H - 40.0),
        2 => Vec2::new(HALF_W + 40.0, rng.gen_range(-HALF_H..HALF_H)),
        _ => Vec2::new(-HALF_W - 40.0, rng.gen_range(-HALF_H..HALF_H)),
    }
}

/// Spawna um inimigo com escalas de dificuldade.
pub fn spawn_enemy_scaled(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    kind: EnemyKind,
    position: Vec2,
    hp_scale: f32,
    speed_scale: f32,
) {
    match kind {
        EnemyKind::Swarmer     => spawn_swarmer(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::Charger     => spawn_charger(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::Shooter     => spawn_shooter(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::Circler     => spawn_circler(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::Bomber      => spawn_bomber(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::Sniper      => spawn_sniper(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::ShieldEnemy => spawn_shield_enemy(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::Splitter    => spawn_splitter(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::Teleporter  => spawn_teleporter(commands, meshes, materials, position, hp_scale, speed_scale),
        EnemyKind::MinionSpawner => spawn_minion_spawner(commands, meshes, materials, position, hp_scale, speed_scale),
    }
}

/// Spawna um inimigo sem escala (para compatibilidade com outros sistemas).
pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    kind: EnemyKind,
    position: Vec2,
) {
    spawn_enemy_scaled(commands, meshes, materials, kind, position, 1.0, 1.0);
}

// ── Funções de spawn individuais ──────────────────────────────────────────────

fn spawn_swarmer(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 30.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::Swarmer,
        EnemyStats { speed: 110.0 * sp_s, damage: 15.0, fire_rate: 0.0, max_hp: hp, score: 50 },
        Health::new(hp), ColliderRadius(10.0),
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 9.0), Vec2::new(-6.0, -7.0), Vec2::new(6.0, -7.0),
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(8.0, 0.5, 0.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_charger(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 60.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::Charger,
        EnemyStats { speed: 180.0 * sp_s, damage: 35.0, fire_rate: 0.0, max_hp: hp, score: 100 },
        Health::new(hp), ColliderRadius(13.0),
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 16.0), Vec2::new(-7.0, -11.0), Vec2::new(7.0, -11.0),
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(8.0, 1.5, 0.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_shooter(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 80.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::Shooter,
        EnemyStats { speed: 65.0 * sp_s, damage: 10.0, fire_rate: 0.8, max_hp: hp, score: 150 },
        Health::new(hp), ColliderRadius(13.0),
        EnemyShootTimer(Timer::from_seconds(1.25, TimerMode::Repeating)),
        Mesh2d(meshes.add(RegularPolygon::new(12.0, 6))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(6.0, 0.0, 6.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_circler(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let dir = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
    let hp = 70.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::Circler,
        EnemyStats { speed: 100.0 * sp_s, damage: 18.0, fire_rate: 0.5, max_hp: hp, score: 120 },
        Health::new(hp), ColliderRadius(11.0),
        CircleOrbit { angle, radius: 200.0, angular_speed: 0.9 * dir },
        EnemyShootTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
        Mesh2d(meshes.add(RegularPolygon::new(11.0, 4))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.0, 4.0, 8.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_bomber(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 50.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::Bomber,
        EnemyStats { speed: 65.0 * sp_s, damage: 0.0, fire_rate: 0.0, max_hp: hp, score: 200 },
        Health::new(hp), ColliderRadius(15.0),
        BomberData { explosion_radius: 70.0, explosion_damage: 60.0, triggered: false },
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(8.0, 4.0, 0.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_sniper(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 60.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::Sniper,
        EnemyStats { speed: 55.0 * sp_s, damage: 45.0, fire_rate: 0.0, max_hp: hp, score: 200 },
        Health::new(hp), ColliderRadius(10.0),
        SniperState {
            charging: false,
            charge_timer: Timer::from_seconds(0.7, TimerMode::Once),
            cooldown_timer: Timer::from_seconds(3.5, TimerMode::Once),
            aim_pos: Vec2::ZERO,
        },
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 18.0), Vec2::new(-4.0, -10.0), Vec2::new(4.0, -10.0),
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(3.0, 8.0, 3.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_shield_enemy(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 120.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::ShieldEnemy,
        EnemyStats { speed: 80.0 * sp_s, damage: 20.0, fire_rate: 0.4, max_hp: hp, score: 180 },
        Health::new(hp), ColliderRadius(14.0), EnemyShield,
        EnemyShootTimer(Timer::from_seconds(2.5, TimerMode::Repeating)),
        Mesh2d(meshes.add(RegularPolygon::new(14.0, 5))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 4.0, 8.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_splitter(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 100.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::Splitter,
        EnemyStats { speed: 85.0 * sp_s, damage: 20.0, fire_rate: 0.0, max_hp: hp, score: 250 },
        Health::new(hp), ColliderRadius(16.0),
        SplitterData { has_split: false, split_threshold: 0.5 },
        Mesh2d(meshes.add(RegularPolygon::new(16.0, 3))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(6.0, 6.0, 0.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_teleporter(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 70.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::Teleporter,
        EnemyStats { speed: 90.0 * sp_s, damage: 25.0, fire_rate: 0.0, max_hp: hp, score: 220 },
        Health::new(hp), ColliderRadius(12.0),
        TeleportData { timer: Timer::from_seconds(2.8, TimerMode::Repeating), teleport_radius: 200.0 },
        Mesh2d(meshes.add(RegularPolygon::new(12.0, 8))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(4.0, 0.0, 8.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_minion_spawner(
    commands: &mut Commands, meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>, pos: Vec2,
    hp_s: f32, sp_s: f32,
) {
    let hp = 180.0 * hp_s;
    commands.spawn((
        Enemy, EnemyKind::MinionSpawner,
        EnemyStats { speed: 40.0 * sp_s, damage: 15.0, fire_rate: 0.0, max_hp: hp, score: 300 },
        Health::new(hp), ColliderRadius(18.0),
        MinionSpawnData { timer: Timer::from_seconds(3.5, TimerMode::Repeating), max_minions: 6, current_minions: 0 },
        Mesh2d(meshes.add(RegularPolygon::new(18.0, 7))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(5.0, 0.0, 3.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}
