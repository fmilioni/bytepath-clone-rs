use bevy::prelude::*;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;

use crate::combat::components::{ColliderRadius, Energy, Health, Shield};
use crate::shop::components::PlayerInventory;
use crate::skill_tree::components::PlayerSkills;
use crate::vfx::components::GameCamera;
use crate::weapons::components::{SpecialAmmo, WeaponCooldown};
use super::abilities::attach_class_ability;
use super::components::{Player, PlayerThrottle};
use super::ship_classes::SelectedShipClass;

/// Spawna a câmera de jogo (chamada apenas uma vez no Startup).
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        GameCamera,
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Bloom {
            intensity: 0.4,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            ..default()
        },
        Tonemapping::TonyMcMapface,
    ));
}

/// Spawna a nave do jogador usando a classe selecionada na tela de escolha.
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    selected: Res<SelectedShipClass>,
    skills: Res<PlayerSkills>,
    inventory: Res<PlayerInventory>,
) {
    let class = selected.0;
    let data = class.data();
    // Aplica bônus de skill tree + itens sobre os stats base da classe
    let items = inventory.combined_bonus();
    let stats = super::stat_calc::compute_full_stats(&class.to_ship_stats(), &skills, &items);

    let fire_rate = stats.fire_rate;
    let max_hp = stats.max_hp;
    let max_shield = stats.max_shield;
    let max_energy = stats.max_energy;
    let energy_regen = stats.energy_regen;
    let shield_regen = 5.0 * skills.shield_regen_mul * items.shield_regen_mul;
    let collider_r = data.collider_radius;
    let special_kind = data.special_ammo_kind;
    let special_max = data.special_ammo_max;

    let mesh = class.build_mesh(&mut meshes);
    let material = materials.add(ColorMaterial::from_color(data.color));

    let player_entity = commands
        .spawn((
            Player,
            class,
            stats,
            PlayerThrottle::default(),
            Health::new(max_hp),
            Shield { regen_rate: shield_regen, ..Shield::new(max_shield) },
            Energy { regen_rate: energy_regen, ..Energy::new(max_energy) },
            ColliderRadius(collider_r),
            WeaponCooldown(Timer::from_seconds(1.0 / fire_rate, TimerMode::Once)),
            SpecialAmmo {
                kind: special_kind,
                count: special_max,
                max: special_max,
            },
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_xyz(0.0, 0.0, crate::constants::Z_PLAYER),
        ))
        .id();

    // Adiciona componente de habilidade específica da classe
    attach_class_ability(&mut commands, player_entity, class);
}

/// Remove a entidade do jogador ao sair do estado Playing (ex: Game Over → menu).
pub fn despawn_player(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
