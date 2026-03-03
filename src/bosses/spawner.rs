use bevy::prelude::*;
use rand::Rng;

use crate::combat::components::{ColliderRadius, Health};
use crate::constants::{HALF_H, HALF_W, Z_ENEMY};
use crate::enemies::components::{Enemy, EnemyShootTimer, EnemyStats};

use super::components::{
    ActiveBoss, Boss, BossKind, BossPhase, DreadnoughtData, PhantomData,
    SentinelData, SentinelShieldPanel, SingularityData, SwarmmotherData,
};

// Constante de Z para boss (fica à frente dos inimigos normais)
const Z_BOSS: f32 = Z_ENEMY + 0.5;

// ── Tecla de teste: B spawna o próximo boss ───────────────────────────────────

pub fn spawn_boss_on_key(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    active: Res<ActiveBoss>,
    boss_query: Query<(), With<Boss>>,
) {
    // Só spawna se não há boss ativo e se pressionar B
    if !keys.just_pressed(KeyCode::KeyB) || active.0.is_some() {
        return;
    }
    if !boss_query.is_empty() {
        return;
    }

    // Spawna Sentinel para teste (pode ser alternado depois)
    let mut rng = rand::thread_rng();
    let kinds = [
        BossKind::Sentinel,
        BossKind::Swarmmother,
        BossKind::Dreadnought,
        BossKind::Phantom,
        BossKind::Singularity,
    ];
    let kind = kinds[rng.gen_range(0..kinds.len())];
    spawn_boss(&mut commands, &mut meshes, &mut materials, kind);
}

/// Spawna um boss no centro da tela.
pub fn spawn_boss(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    kind: BossKind,
) {
    let pos = Vec2::new(
        rand::thread_rng().gen_range(-HALF_W * 0.3..HALF_W * 0.3),
        rand::thread_rng().gen_range(-HALF_H * 0.3..HALF_H * 0.3),
    );

    match kind {
        BossKind::Sentinel => spawn_sentinel(commands, meshes, materials, pos),
        BossKind::Swarmmother => spawn_swarmmother(commands, meshes, materials, pos),
        BossKind::Dreadnought => spawn_dreadnought(commands, meshes, materials, pos),
        BossKind::Phantom => spawn_phantom(commands, meshes, materials, pos),
        BossKind::Singularity => spawn_singularity(commands, meshes, materials, pos),
    }
}

// ── Sentinel ──────────────────────────────────────────────────────────────────

fn spawn_sentinel(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) {
    let boss_entity = commands.spawn((
        Boss,
        BossKind::Sentinel,
        BossPhase::new(),
        Health::new(800.0),
        ColliderRadius(28.0),
        SentinelData {
            shield_angle: 0.0,
            rotate_speed: 1.0,
            attack_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
            panel_count: 2,
        },
        EnemyShootTimer(Timer::from_seconds(2.5, TimerMode::Repeating)),
        Enemy,
        EnemyStats { speed: 60.0, damage: 12.0, fire_rate: 0.4, max_hp: 800.0, score: 1000 },
        Mesh2d(meshes.add(RegularPolygon::new(28.0, 6))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.0, 4.0, 6.0)))),
        Transform::from_translation(pos.extend(Z_BOSS)),
    )).id();

    // Spawna 2 painéis de escudo iniciais
    spawn_shield_panels(commands, meshes, materials, boss_entity, 2);
}

pub fn spawn_shield_panels(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    boss_entity: Entity,
    count: u32,
) {
    for i in 0..count {
        let angle_offset = (i as f32 / count as f32) * std::f32::consts::TAU;
        commands.spawn((
            SentinelShieldPanel { angle_offset, boss_entity },
            ColliderRadius(20.0),
            Enemy, // Para o sistema de colisão saber que existe
            Mesh2d(meshes.add(Rectangle::new(8.0, 55.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.0, 6.0, 8.0)))),
            Transform::from_translation(Vec3::ZERO),
        ));
    }
}

// ── Swarmmother ───────────────────────────────────────────────────────────────

fn spawn_swarmmother(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) {
    commands.spawn((
        Boss,
        BossKind::Swarmmother,
        BossPhase::new(),
        Health::new(600.0),
        ColliderRadius(32.0),
        SwarmmotherData {
            wave_timer: Timer::from_seconds(3.5, TimerMode::Repeating),
            minions_per_wave: 3,
            shoot_timer: Timer::from_seconds(4.0, TimerMode::Repeating),
        },
        EnemyShootTimer(Timer::from_seconds(4.0, TimerMode::Repeating)),
        Enemy,
        EnemyStats { speed: 50.0, damage: 10.0, fire_rate: 0.25, max_hp: 600.0, score: 900 },
        Mesh2d(meshes.add(RegularPolygon::new(32.0, 8))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(5.0, 0.0, 6.0)))),
        Transform::from_translation(pos.extend(Z_BOSS)),
    ));
}

// ── Dreadnought ───────────────────────────────────────────────────────────────

fn spawn_dreadnought(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) {
    commands.spawn((
        Boss,
        BossKind::Dreadnought,
        BossPhase::new(),
        Health::new(1000.0),
        ColliderRadius(36.0),
        DreadnoughtData {
            barrage_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
            barrage_count: 5,
            nova_timer: Timer::from_seconds(6.0, TimerMode::Repeating),
        },
        EnemyShootTimer(Timer::from_seconds(2.5, TimerMode::Repeating)),
        Enemy,
        EnemyStats { speed: 30.0, damage: 14.0, fire_rate: 0.4, max_hp: 1000.0, score: 1200 },
        Mesh2d(meshes.add(RegularPolygon::new(36.0, 4))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(6.0, 3.0, 0.0)))),
        Transform::from_translation(pos.extend(Z_BOSS)),
    ));
}

// ── Phantom ───────────────────────────────────────────────────────────────────

fn spawn_phantom(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) {
    commands.spawn((
        Boss,
        BossKind::Phantom,
        BossPhase::new(),
        Health::new(700.0),
        ColliderRadius(22.0),
        PhantomData {
            teleport_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            clone_count: 1,
        },
        EnemyShootTimer(Timer::from_seconds(3.5, TimerMode::Repeating)),
        Enemy,
        EnemyStats { speed: 90.0, damage: 11.0, fire_rate: 0.3, max_hp: 700.0, score: 1000 },
        Mesh2d(meshes.add(RegularPolygon::new(22.0, 3))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(4.0, 0.0, 8.0)))),
        Transform::from_translation(pos.extend(Z_BOSS)),
    ));
}

// ── Singularity ───────────────────────────────────────────────────────────────

fn spawn_singularity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) {
    commands.spawn((
        Boss,
        BossKind::Singularity,
        BossPhase::new(),
        Health::new(900.0),
        ColliderRadius(30.0),
        SingularityData {
            pull_force: 100.0,
            pulse_timer: Timer::from_seconds(1.2, TimerMode::Repeating),
            nova_timer: Timer::from_seconds(4.0, TimerMode::Repeating),
            spawn_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        },
        Enemy,
        EnemyStats { speed: 0.0, damage: 15.0, fire_rate: 0.0, max_hp: 900.0, score: 1500 },
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.0, 8.0)))),
        Transform::from_translation(pos.extend(Z_BOSS)),
    ));
}

/// Spawna um clone decoy do Phantom em posição offset.
pub fn spawn_phantom_clone(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) {
    commands.spawn((
        super::components::PhantomClone,
        Health::new(1.0), // 1 hit kill
        ColliderRadius(22.0),
        Enemy,
        EnemyStats { speed: 80.0, damage: 8.0, fire_rate: 0.0, max_hp: 1.0, score: 0 },
        Mesh2d(meshes.add(RegularPolygon::new(22.0, 3))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(2.0, 0.0, 4.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY + 0.3)),
    ));
}
