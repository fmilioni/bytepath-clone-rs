use bevy::prelude::*;

use crate::combat::components::Energy;
use crate::constants::{PLAYER_ROTATION_SPEED, HALF_W, HALF_H};
use super::components::{Player, PlayerThrottle, ShipStats};

/// Move a nave continuamente para frente (estilo Bytepath: a nave nunca para).
/// A/D rotaciona. W acelera (consome energia), S freia.
pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &ShipStats, &mut PlayerThrottle, &mut Energy), With<Player>>,
) {
    let (mut transform, stats, mut throttle, mut energy) = query.single_mut();
    let dt = time.delta_secs();

    // Rotação: A/esquerda = anti-horário, D/direita = horário
    let rotation_dir = if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        1.0
    } else if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        -1.0
    } else {
        0.0
    };

    if rotation_dir != 0.0 {
        transform.rotation *=
            Quat::from_rotation_z(rotation_dir * PLAYER_ROTATION_SPEED * dt);
    }

    // W = acelera (requer energia), S = freia, sem tecla = volta ao normal
    let boosting = keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp);
    let braking  = keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown);

    if boosting && energy.current > 0.0 {
        throttle.multiplier = (throttle.multiplier + 1.5 * dt).min(2.0);
    } else if braking {
        throttle.multiplier = (throttle.multiplier - 2.0 * dt).max(0.25);
    } else {
        // Retorna gradualmente à velocidade base
        let diff = 1.0_f32 - throttle.multiplier;
        throttle.multiplier += diff.signum() * (1.5 * dt).min(diff.abs());
    }

    // Drena energia enquanto throttle > 1.0 (30 energia/s no boost máximo)
    if throttle.multiplier > 1.0 {
        let drain = (throttle.multiplier - 1.0) * 30.0 * dt;
        energy.current = (energy.current - drain).max(0.0);
        if energy.current <= 0.0 {
            throttle.multiplier = throttle.multiplier.min(1.0);
        }
    }

    // Movimento
    let forward = transform.rotation * Vec3::Y;
    transform.translation += forward * stats.speed * throttle.multiplier * dt;

    // Wrap ao sair dos limites da tela
    let pos = &mut transform.translation;
    if pos.x > HALF_W {
        pos.x = -HALF_W;
    } else if pos.x < -HALF_W {
        pos.x = HALF_W;
    }
    if pos.y > HALF_H {
        pos.y = -HALF_H;
    } else if pos.y < -HALF_H {
        pos.y = HALF_H;
    }
}
