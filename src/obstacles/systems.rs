use bevy::prelude::*;
use rand::Rng;

use crate::combat::components::{ColliderRadius, DamageEvent, DeathEvent, Health};
use crate::constants::{HALF_H, HALF_W, Z_OBSTACLE};
use crate::player::components::Player;
use super::{Asteroid, AsteroidSize};

#[derive(Resource)]
pub struct AsteroidSpawnTimer(pub Timer);

// ── Cor dos asteroides (sem bloom, tom rochoso) ───────────────────────────────

fn asteroid_color(size: AsteroidSize) -> Color {
    match size {
        AsteroidSize::Large => Color::srgb(0.45, 0.35, 0.22),
        AsteroidSize::Small => Color::srgb(0.38, 0.28, 0.18),
    }
}

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_asteroids(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asteroid_count: Query<(), With<Asteroid>>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    timer.0.reset();

    if asteroid_count.iter().count() >= 12 {
        return;
    }

    let mut rng = rand::thread_rng();
    let pos = random_asteroid_spawn(&mut rng);
    let target = Vec2::new(
        rng.gen_range(-HALF_W * 0.7..HALF_W * 0.7),
        rng.gen_range(-HALF_H * 0.7..HALF_H * 0.7),
    );
    let velocity = (target - pos).normalize() * rng.gen_range(40.0..90.0_f32);

    spawn_asteroid(&mut commands, &mut meshes, &mut materials, pos, velocity, AsteroidSize::Large, &mut rng);
}

pub fn spawn_asteroid(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    velocity: Vec2,
    size: AsteroidSize,
    rng: &mut impl Rng,
) {
    let (radius, sides, hp) = match size {
        AsteroidSize::Large => (rng.gen_range(20.0..28.0_f32), rng.gen_range(7_u32..10), 80.0),
        AsteroidSize::Small => (rng.gen_range(9.0..14.0_f32), rng.gen_range(7_u32..10), 25.0),
    };
    let angular_velocity = rng.gen_range(-2.0..2.0_f32);

    commands.spawn((
        Asteroid {
            size,
            velocity,
            angular_velocity,
        },
        Health::new(hp),
        ColliderRadius(radius),
        Mesh2d(meshes.add(RegularPolygon::new(radius, sides))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(asteroid_color(size)))),
        Transform::from_translation(position.extend(Z_OBSTACLE)),
    ));
}

fn random_asteroid_spawn(rng: &mut impl Rng) -> Vec2 {
    match rng.gen_range(0..4) {
        0 => Vec2::new(rng.gen_range(-HALF_W..HALF_W), HALF_H + 40.0),
        1 => Vec2::new(rng.gen_range(-HALF_W..HALF_W), -HALF_H - 40.0),
        2 => Vec2::new(HALF_W + 40.0, rng.gen_range(-HALF_H..HALF_H)),
        _ => Vec2::new(-HALF_W - 40.0, rng.gen_range(-HALF_H..HALF_H)),
    }
}

// ── Movimento ─────────────────────────────────────────────────────────────────

pub fn asteroid_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Asteroid)>,
) {
    let dt = time.delta_secs();
    for (mut transform, asteroid) in query.iter_mut() {
        transform.translation += (asteroid.velocity * dt).extend(0.0);
        transform.rotation *= Quat::from_rotation_z(asteroid.angular_velocity * dt);

        // Remove se muito longe da tela
        let pos = transform.translation.truncate();
        if pos.x.abs() > HALF_W * 2.5 || pos.y.abs() > HALF_H * 2.5 {
            // Marcado para remoção via despawn no sistema de morte
            // (simplesmente sai de tela, novo spawn virá)
        }
    }
}

// ── Colisão com player ────────────────────────────────────────────────────────

pub fn asteroid_player_collision(
    asteroid_query: Query<(&Transform, &ColliderRadius), With<Asteroid>>,
    player_query: Query<(Entity, &Transform, &ColliderRadius), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let Ok((player_entity, player_transform, player_radius)) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (asteroid_transform, asteroid_radius) in asteroid_query.iter() {
        let dist = player_pos.distance(asteroid_transform.translation.truncate());
        if dist < player_radius.0 + asteroid_radius.0 {
            damage_events.send(DamageEvent {
                target: player_entity,
                amount: 20.0,
            });
        }
    }
}

// ── Colisão projétil ← asteroidé é tratada em weapons/systems.rs ──────────────
// O projectile_collision já itera sobre todos os targets com ColliderRadius.
// Asteroides têm ColliderRadius mas não têm Enemy, então só balas do player os acertam
// porque balas do player testam Enemy e balas de inimigos testam Player.
// Para que balas do player acertem asteroides precisamos ajustar a query.

// ── Morte de asteroide: quebra em fragmentos ──────────────────────────────────

pub fn handle_asteroid_deaths(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut death_events: EventReader<DeathEvent>,
    asteroid_query: Query<&Asteroid>,
) {
    let mut rng = rand::thread_rng();

    for event in death_events.read() {
        // Verifica se é um asteroide
        let Ok(asteroid) = asteroid_query.get(event.entity) else {
            continue;
        };

        if asteroid.size == AsteroidSize::Large {
            // Quebra em 2-3 pequenos
            let count = rng.gen_range(2_u32..4);
            for _ in 0..count {
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let speed = rng.gen_range(60.0..120.0_f32);
                let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
                let offset = Vec2::new(
                    rng.gen_range(-10.0..10.0_f32),
                    rng.gen_range(-10.0..10.0_f32),
                );
                spawn_asteroid(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    event.position + offset,
                    velocity,
                    AsteroidSize::Small,
                    &mut rng,
                );
            }
        }
        // Small asteroides apenas desaparecem (já despawnados por handle_deaths)
    }
}
