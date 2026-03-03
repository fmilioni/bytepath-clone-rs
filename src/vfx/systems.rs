use bevy::prelude::*;
use rand::Rng;

use crate::combat::components::DeathEvent;
use crate::constants::{COLOR_STAR, HALF_H, HALF_W, Z_BACKGROUND, Z_STAR, Z_VFX};
use crate::player::components::Player;

use super::components::{GameCamera, Particle, ScreenShake, Star, TrailSegment};
use super::particles::spawn_enemy_death_explosion;

const MAX_SHAKE_OFFSET: f32 = 18.0;
const TRAUMA_DECAY: f32 = 1.2; // por segundo

/// Drift suave de estrelas nos menus (paralaxe baseada no tempo).
pub fn drift_stars_menu(
    time: Res<Time>,
    mut star_q: Query<(&Star, &mut Transform)>,
) {
    let t = time.elapsed_secs();
    let dt = time.delta_secs();
    for (star, mut tf) in star_q.iter_mut() {
        // Deriva lentamente para cima com velocidade proporcional ao parallax_factor
        tf.translation.y += star.parallax_factor * 8.0 * dt;
        // Oscilação lateral suave e periódica
        tf.translation.x += (t * star.parallax_factor * 0.7).sin() * star.parallax_factor * 1.5 * dt;
        // Wrap vertical
        if tf.translation.y > HALF_H * 1.6 {
            tf.translation.y = -HALF_H * 1.6;
        }
    }
}

/// Inicializa o fundo de estrelas.
pub fn spawn_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // Camada de estrelas distantes (pequenas, paralaxe lenta)
    for _ in 0..120 {
        let x = rng.gen_range(-HALF_W * 1.5..HALF_W * 1.5);
        let y = rng.gen_range(-HALF_H * 1.5..HALF_H * 1.5);
        let size = rng.gen_range(0.4..1.0_f32);
        let brightness = rng.gen_range(0.3..0.9_f32);
        let color = Color::srgb(brightness * 1.5, brightness * 1.5, brightness * 2.0);

        commands.spawn((
            Star { parallax_factor: 0.05 },
            Mesh2d(meshes.add(Circle::new(size))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_xyz(x, y, Z_BACKGROUND),
        ));
    }

    // Camada de estrelas próximas (maiores, paralaxe um pouco maior)
    for _ in 0..30 {
        let x = rng.gen_range(-HALF_W * 1.5..HALF_W * 1.5);
        let y = rng.gen_range(-HALF_H * 1.5..HALF_H * 1.5);
        let size = rng.gen_range(1.0..2.2_f32);
        let color = COLOR_STAR;

        commands.spawn((
            Star { parallax_factor: 0.12 },
            Mesh2d(meshes.add(Circle::new(size))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_xyz(x, y, Z_STAR),
        ));
    }
}

/// Spawna segmentos de trail atrás da nave do jogador.
pub fn spawn_trail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(transform) = player_query.get_single() else {
        return;
    };

    // Posição um pouco atrás da nave
    let back = transform.rotation * Vec3::NEG_Y;
    let trail_pos = transform.translation + back * 12.0;

    commands.spawn((
        TrailSegment {
            lifetime: Timer::from_seconds(0.18, TimerMode::Once),
        },
        Mesh2d(meshes.add(Circle::new(3.5))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(
            Color::srgba(0.0, 2.0, 4.0, 0.8),
        ))),
        Transform::from_translation(trail_pos.with_z(Z_VFX - 0.1)),
    ));
}

/// Atualiza segmentos de trail: encolhe e remove ao expirar.
pub fn update_trail(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TrailSegment)>,
) {
    for (entity, mut transform, mut segment) in query.iter_mut() {
        segment.lifetime.tick(time.delta());
        let progress = segment.lifetime.fraction();
        let scale = 1.0 - progress;
        transform.scale = Vec3::splat(scale.max(0.01));

        if segment.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Atualiza partículas: move, decai, remove ao expirar.
pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut particle) in query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Move
        let movement = particle.velocity * dt;
        transform.translation += movement.extend(0.0);

        // Desacelera gradualmente
        particle.velocity *= 0.92_f32.powf(dt * 60.0);

        // Encolhe conforme o tempo passa
        let progress = particle.lifetime.fraction();
        let scale = 1.0 - progress * 0.85;
        transform.scale = Vec3::splat(scale.max(0.01));
    }
}

/// Aplica screen shake à câmera baseado no trauma.
pub fn apply_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
    let dt = time.delta_secs();
    shake.time_acc += dt * 12.0;

    if shake.trauma > 0.0 {
        shake.trauma = (shake.trauma - TRAUMA_DECAY * dt).max(0.0);
    }

    let Ok(mut cam_transform) = camera_query.get_single_mut() else {
        return;
    };

    let intensity = shake.trauma * shake.trauma; // quadrático = mais sutil
    let offset_x = (shake.time_acc * 1.3).sin() * intensity * MAX_SHAKE_OFFSET;
    let offset_y = (shake.time_acc * 1.7).cos() * intensity * MAX_SHAKE_OFFSET;

    cam_transform.translation.x = offset_x;
    cam_transform.translation.y = offset_y;
}

/// Reage a eventos de morte: spawna explosão de partículas.
pub fn on_death_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut death_events: EventReader<DeathEvent>,
    mut shake: ResMut<ScreenShake>,
) {
    for event in death_events.read() {
        if event.was_enemy {
            spawn_enemy_death_explosion(
                &mut commands,
                &mut meshes,
                &mut materials,
                event.position,
                Color::srgb(8.0, 1.5, 0.0),
            );
            shake.add_trauma(0.15);
        }
    }
}
