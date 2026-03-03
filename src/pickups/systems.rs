use bevy::prelude::*;
use rand::Rng;

use crate::combat::components::{ColliderRadius, DeathEvent, Energy, Health, Shield};
use crate::constants::Z_PICKUP;
use crate::player::components::Player;
use crate::shop::components::PlayerInventory;
use crate::skill_tree::components::PlayerSkills;
use crate::weapons::components::SpecialAmmo;

use super::{Pickup, PickupBobbing, PickupKind, PickupLifetime};

// ── Spawn no death de inimigos ────────────────────────────────────────────────

pub fn spawn_pickup_on_death(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for event in death_events.read() {
        if !event.was_enemy { continue; }
        // 40% de chance de dropar algo
        if !rng.gen_bool(0.40) { continue; }
        let kind = random_kind(&mut rng);
        spawn_pickup(&mut commands, &mut meshes, &mut materials, event.position, kind);
    }
}

fn random_kind(rng: &mut impl Rng) -> PickupKind {
    match rng.gen_range(0..100) {
        0..=29  => PickupKind::HealthPack,
        30..=49 => PickupKind::EnergyPack,
        50..=64 => PickupKind::ShieldPack,
        65..=74 => PickupKind::Scrap,
        75..=84 => PickupKind::SpecialAmmoPack,
        85..=93 => PickupKind::Fuel,
        _       => PickupKind::MysteryCrate,
    }
}

pub fn spawn_pickup(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
    kind: PickupKind,
) {
    let (mesh, color, radius) = pickup_visuals(kind, meshes);
    let mat = materials.add(ColorMaterial::from_color(color));

    commands.spawn((
        Pickup,
        kind,
        PickupLifetime(Timer::from_seconds(12.0, TimerMode::Once)),
        PickupBobbing { base_y: pos.y, time: 0.0 },
        ColliderRadius(radius),
        Mesh2d(mesh),
        MeshMaterial2d(mat),
        Transform::from_translation(pos.extend(Z_PICKUP)),
    ));
}

fn pickup_visuals(kind: PickupKind, meshes: &mut Assets<Mesh>) -> (Handle<Mesh>, Color, f32) {
    match kind {
        PickupKind::HealthPack    => (meshes.add(RegularPolygon::new(8.0, 4)), Color::srgb(0.0, 8.0, 2.0), 8.0),
        PickupKind::ShieldPack    => (meshes.add(Circle::new(7.0)),            Color::srgb(0.0, 2.0, 8.0), 7.0),
        PickupKind::EnergyPack    => (meshes.add(RegularPolygon::new(7.0, 6)), Color::srgb(8.0, 6.0, 0.0), 7.0),
        PickupKind::SpecialAmmoPack=>(meshes.add(RegularPolygon::new(7.0, 3)), Color::srgb(8.0, 0.0, 6.0), 7.0),
        PickupKind::Scrap         => (meshes.add(RegularPolygon::new(6.0, 5)), Color::srgb(4.0, 4.0, 4.0), 6.0),
        PickupKind::Fuel          => (meshes.add(Circle::new(6.0)),            Color::srgb(0.0, 6.0, 8.0), 6.0),
        PickupKind::MysteryCrate  => (meshes.add(RegularPolygon::new(9.0, 4)), Color::srgb(8.0, 2.0, 8.0), 9.0),
    }
}

// ── Atração ao player ─────────────────────────────────────────────────────────

pub fn attract_pickups(
    time: Res<Time>,
    skills: Res<PlayerSkills>,
    inventory: Res<PlayerInventory>,
    player_q: Query<&Transform, With<Player>>,
    mut pickup_q: Query<(&mut Transform, &PickupBobbing), (With<Pickup>, Without<Player>)>,
) {
    let Ok(player_tf) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();
    let radius = skills.pickup_radius + inventory.combined_bonus().pickup_radius;
    let dt = time.delta_secs();

    for (mut tf, bob) in pickup_q.iter_mut() {
        let pos = tf.translation.truncate();
        let to_player = player_pos - pos;
        let dist = to_player.length();

        if dist < radius && dist > 2.0 {
            let pull = (1.0 - dist / radius).max(0.0) * 250.0 + 80.0;
            let new_xy = pos + to_player.normalize() * pull * dt;
            tf.translation.x = new_xy.x;
            tf.translation.y = new_xy.y + bob.base_y - pos.y; // preserva bobbing offset
        }
    }
}

// ── Coleta ────────────────────────────────────────────────────────────────────

pub fn collect_pickups(
    mut commands: Commands,
    player_q: Query<(Entity, &Transform, &ColliderRadius), With<Player>>,
    pickup_q: Query<(Entity, &Transform, &ColliderRadius, &PickupKind), With<Pickup>>,
    mut health_q: Query<&mut Health, With<Player>>,
    mut shield_q: Query<&mut Shield, With<Player>>,
    mut energy_q: Query<&mut Energy, With<Player>>,
    mut ammo_q: Query<&mut SpecialAmmo, With<Player>>,
    mut skills: ResMut<PlayerSkills>,
) {
    let Ok((_player_entity, player_tf, player_col)) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();

    for (pickup_entity, pickup_tf, pickup_col, kind) in pickup_q.iter() {
        let dist = player_pos.distance(pickup_tf.translation.truncate());
        if dist >= player_col.0 + pickup_col.0 { continue; }

        // Aplica efeito
        match kind {
            PickupKind::HealthPack => {
                if let Ok(mut h) = health_q.get_single_mut() {
                    h.current = (h.current + 30.0).min(h.max);
                }
            }
            PickupKind::ShieldPack => {
                if let Ok(mut s) = shield_q.get_single_mut() {
                    s.current = (s.current + 20.0).min(s.max);
                }
            }
            PickupKind::EnergyPack => {
                if let Ok(mut e) = energy_q.get_single_mut() {
                    e.current = (e.current + 25.0).min(e.max);
                }
            }
            PickupKind::SpecialAmmoPack => {
                if let Ok(mut a) = ammo_q.get_single_mut() {
                    a.count = (a.count + 15).min(a.max);
                }
            }
            PickupKind::Scrap => {
                // Scrap dá 1 ponto de skill (stand-in para economia)
                skills.skill_points += 1;
            }
            PickupKind::Fuel => {
                if let Ok(mut e) = energy_q.get_single_mut() {
                    e.current = (e.current + e.max * 0.3).min(e.max);
                }
            }
            PickupKind::MysteryCrate => {
                // Efeito aleatório
                let mut rng = rand::thread_rng();
                match rng.gen_range(0..4) {
                    0 => { if let Ok(mut h) = health_q.get_single_mut() {
                        h.current = (h.current + 50.0).min(h.max);
                    }}
                    1 => { if let Ok(mut s) = shield_q.get_single_mut() {
                        s.current = (s.current + 30.0).min(s.max);
                    }}
                    2 => { if let Ok(mut e) = energy_q.get_single_mut() {
                        e.current = (e.current + 50.0).min(e.max);
                    }}
                    _ => { skills.skill_points += 3; }
                }
            }
        }

        if let Some(mut cmd) = commands.get_entity(pickup_entity) {
            cmd.despawn();
        }
    }
}

// ── Animação de bobbing ───────────────────────────────────────────────────────

pub fn pickup_bobbing(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PickupBobbing), With<Pickup>>,
) {
    for (mut tf, mut bob) in query.iter_mut() {
        bob.time += time.delta_secs();
        let offset = (bob.time * 2.5).sin() * 4.0;
        tf.translation.y = bob.base_y + offset;
    }
}

// ── Tempo de vida ─────────────────────────────────────────────────────────────

pub fn pickup_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut PickupLifetime), With<Pickup>>,
) {
    for (entity, mut lt) in query.iter_mut() {
        lt.0.tick(time.delta());
        if lt.0.finished() {
            if let Some(mut cmd) = commands.get_entity(entity) {
                cmd.despawn();
            }
        }
    }
}
