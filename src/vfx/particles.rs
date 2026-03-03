use bevy::prelude::*;
use rand::Rng;

use crate::constants::Z_VFX;
use super::components::Particle;

/// Spawna uma explosão de partículas em `position`.
pub fn spawn_explosion(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    color: Color,
    count: u32,
    speed: f32,
    lifetime_secs: f32,
) {
    let mut rng = rand::thread_rng();
    let mesh = meshes.add(Circle::new(2.5));

    for _ in 0..count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let mag = rng.gen_range(speed * 0.3..speed);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * mag;

        commands.spawn((
            Particle {
                velocity,
                lifetime: Timer::from_seconds(lifetime_secs, TimerMode::Once),
                initial_scale: 1.0,
                fade: true,
            },
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_translation(position.extend(Z_VFX)),
        ));
    }
}

/// Spawna uma explosão em tamanho de morte de inimigo pequeno/médio.
pub fn spawn_enemy_death_explosion(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    color: Color,
) {
    spawn_explosion(commands, meshes, materials, position, color, 18, 180.0, 0.6);
}
