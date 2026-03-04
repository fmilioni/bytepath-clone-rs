use bevy::prelude::*;
use rand::Rng;

use crate::combat::components::{ColliderRadius, DamageEvent, DeathEvent, Health, SelfHandlesDespawn};
use crate::constants::{HALF_H, HALF_W, Z_OBSTACLE, Z_VFX};
use crate::player::components::Player;
use crate::vfx::components::Particle;
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
    asteroid_query: Query<&AsteroidSize, With<Asteroid>>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    timer.0.reset();

    // Conta apenas Large: os Smalls (fragmentos) não bloqueiam novos spawns
    let large_count = asteroid_query.iter().filter(|&&s| s == AsteroidSize::Large).count();
    if large_count >= 6 {
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
        AsteroidSize::Large => (rng.gen_range(20.0..28.0_f32), rng.gen_range(7_u32..10), 50.0),
        AsteroidSize::Small => (rng.gen_range(9.0..14.0_f32), rng.gen_range(7_u32..10), 15.0),
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
        SelfHandlesDespawn, // handle_asteroid_deaths faz o despawn + spawna fragmentos
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
    asteroid_query: Query<(Entity, &Asteroid, &Transform, &ColliderRadius)>,
    player_query: Query<(Entity, &Transform, &ColliderRadius), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let Ok((player_entity, player_transform, player_radius)) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (asteroid_entity, asteroid, asteroid_transform, asteroid_radius) in asteroid_query.iter() {
        let dist = player_pos.distance(asteroid_transform.translation.truncate());
        if dist < player_radius.0 + asteroid_radius.0 {
            match asteroid.size {
                AsteroidSize::Large => {
                    damage_events.send(DamageEvent { target: player_entity, amount: 99999.0 });
                }
                AsteroidSize::Small => {
                    damage_events.send(DamageEvent { target: asteroid_entity, amount: 99999.0 });
                    damage_events.send(DamageEvent { target: player_entity, amount: 20.0 });
                }
            }
        }
    }
}

// ── Colisão projétil ← asteroidé é tratada em weapons/systems.rs ──────────────
// O projectile_collision já itera sobre todos os targets com ColliderRadius.
// Asteroides têm ColliderRadius mas não têm Enemy, então só balas do player os acertam
// porque balas do player testam Enemy e balas de inimigos testam Player.
// Para que balas do player acertem asteroides precisamos ajustar a query.

// ── Morte de asteroide: quebra em fragmentos + partículas ────────────────────

pub fn handle_asteroid_deaths(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut death_events: EventReader<DeathEvent>,
    asteroid_query: Query<&Asteroid>,
) {
    let mut rng = rand::thread_rng();

    for event in death_events.read() {
        let Ok(asteroid) = asteroid_query.get(event.entity) else {
            continue;
        };

        let pos = event.position;

        match asteroid.size {
            AsteroidSize::Large => {
                // Quebra em 3-5 fragmentos menores
                let count = rng.gen_range(3_u32..6);
                for _ in 0..count {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(60.0..130.0_f32);
                    let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
                    let offset = Vec2::new(
                        rng.gen_range(-12.0..12.0_f32),
                        rng.gen_range(-12.0..12.0_f32),
                    );
                    spawn_asteroid(
                        &mut commands, &mut meshes, &mut materials,
                        pos + offset, velocity, AsteroidSize::Small, &mut rng,
                    );
                }

                // Partículas de rocha (tom terroso/cinza)
                let rock_mesh = meshes.add(Circle::new(2.5));
                for _ in 0..18 {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(60.0..200.0_f32);
                    let vel = Vec2::new(angle.cos(), angle.sin()) * speed;
                    let brightness = rng.gen_range(0.3..0.6_f32);
                    commands.spawn((
                        Particle {
                            velocity: vel,
                            lifetime: Timer::from_seconds(rng.gen_range(0.4..0.9), TimerMode::Once),
                            initial_scale: 1.0,
                            fade: true,
                        },
                        Mesh2d(rock_mesh.clone()),
                        MeshMaterial2d(materials.add(ColorMaterial::from_color(
                            Color::srgb(brightness * 1.8, brightness * 1.4, brightness),
                        ))),
                        Transform::from_translation(pos.extend(Z_VFX)),
                    ));
                }
            }
            AsteroidSize::Small => {
                // Partículas pequenas ao morrer
                let rock_mesh = meshes.add(Circle::new(1.5));
                for _ in 0..8 {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(40.0..120.0_f32);
                    let vel = Vec2::new(angle.cos(), angle.sin()) * speed;
                    let brightness = rng.gen_range(0.25..0.5_f32);
                    commands.spawn((
                        Particle {
                            velocity: vel,
                            lifetime: Timer::from_seconds(rng.gen_range(0.2..0.5), TimerMode::Once),
                            initial_scale: 1.0,
                            fade: true,
                        },
                        Mesh2d(rock_mesh.clone()),
                        MeshMaterial2d(materials.add(ColorMaterial::from_color(
                            Color::srgb(brightness * 1.8, brightness * 1.4, brightness),
                        ))),
                        Transform::from_translation(pos.extend(Z_VFX)),
                    ));
                }
            }
        }

        // Despawn da entidade — handle_deaths pula asteroides (SelfHandlesDespawn)
        if let Some(mut cmd) = commands.get_entity(event.entity) {
            cmd.despawn();
        }
    }
}
