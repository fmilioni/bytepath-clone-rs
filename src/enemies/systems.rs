use bevy::prelude::*;
use rand::Rng;

use crate::combat::components::{ColliderRadius, DamageEvent, Health, Shield};
use crate::constants::{HALF_H, HALF_W, Z_VFX};
use crate::player::components::{EnemyHitCooldown, Player};
use crate::vfx::components::{Particle, ScreenShake};

use super::components::{
    BomberData, CircleOrbit, Enemy, EnemyKind, EnemyStats, MinionSpawnData,
    SniperState, SniperWarning, SplitterData, TeleportData,
};
use super::spawner::spawn_enemy;

// ── AI Principal ─────────────────────────────────────────────────────────────

/// Controla o movimento de cada tipo de inimigo.
pub fn enemy_ai(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut circler_query: Query<(&mut Transform, &EnemyStats, &mut CircleOrbit), (With<Enemy>, Without<Player>)>,
    mut basic_query: Query<
        (&mut Transform, &EnemyKind, &EnemyStats),
        (With<Enemy>, Without<Player>, Without<CircleOrbit>),
    >,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let dt = time.delta_secs();

    // Circler orbita o player
    for (mut transform, stats, mut orbit) in circler_query.iter_mut() {
        orbit.angle += orbit.angular_speed * dt;
        let target = player_pos + Vec2::from_angle(orbit.angle) * orbit.radius;
        let current = transform.translation.truncate();
        let to_target = target - current;

        if to_target.length() > 2.0 {
            let dir = to_target.normalize();
            transform.translation += (dir * stats.speed * dt).extend(0.0);
        }

        // Rotaciona para o player
        let to_player = (player_pos - current).normalize_or_zero();
        if to_player != Vec2::ZERO {
            let angle = to_player.y.atan2(to_player.x) - std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }

    // Outros tipos de inimigos
    for (mut transform, kind, stats) in basic_query.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let to_player = player_pos - enemy_pos;
        let distance = to_player.length();

        if distance < 0.1 {
            continue;
        }

        let direction = to_player.normalize();

        let movement: Vec2 = match kind {
            EnemyKind::Swarmer => direction * stats.speed,
            EnemyKind::Charger => direction * stats.speed,
            EnemyKind::Bomber => direction * stats.speed * 0.85,
            EnemyKind::Sniper => {
                // Mantém distância grande
                let preferred = 380.0;
                if distance > preferred + 30.0 {
                    direction * stats.speed
                } else if distance < preferred - 30.0 {
                    -direction * stats.speed * 0.7
                } else {
                    Vec2::ZERO
                }
            }
            EnemyKind::ShieldEnemy => {
                // Avança lentamente
                direction * stats.speed
            }
            EnemyKind::Splitter => direction * stats.speed * 0.75,
            EnemyKind::Teleporter => {
                // Movimento errático: tenta ficar perto mas não muito
                let preferred = 200.0;
                if distance > preferred + 50.0 {
                    direction * stats.speed
                } else {
                    Vec2::ZERO
                }
            }
            EnemyKind::MinionSpawner => {
                // Fica bem recuado
                let preferred = 350.0;
                if distance > preferred + 40.0 {
                    direction * stats.speed * 0.5
                } else if distance < preferred - 40.0 {
                    -direction * stats.speed * 0.5
                } else {
                    Vec2::ZERO
                }
            }
            EnemyKind::Shooter => {
                let preferred = 250.0;
                if distance > preferred + 20.0 {
                    direction * stats.speed
                } else if distance < preferred - 20.0 {
                    -direction * stats.speed * 0.5
                } else {
                    Vec2::ZERO
                }
            }
            EnemyKind::Circler => Vec2::ZERO, // tratado acima
        };

        transform.translation += (movement * dt).extend(0.0);

        // Rotaciona para o player
        if direction != Vec2::ZERO {
            let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

// ── Tick do cooldown de invencibilidade do player ────────────────────────────

pub fn tick_hit_cooldown(
    time: Res<Time>,
    mut query: Query<&mut EnemyHitCooldown, With<Player>>,
) {
    let dt = time.delta_secs();
    for mut cd in query.iter_mut() {
        cd.remaining = (cd.remaining - dt).max(0.0);
    }
}

// ── Colisão corpo-a-corpo (inimigo → player) ─────────────────────────────────
// Contato com inimigo: drena todo o shield e aplica 30 de dano direto na HP.
// O inimigo morre instantaneamente.

pub fn enemy_player_collision(
    enemy_query: Query<(Entity, &Transform, &ColliderRadius), (With<Enemy>, Without<BomberData>)>,
    player_query: Query<(Entity, &Transform, &ColliderRadius), With<Player>>,
    mut shield_query: Query<&mut Shield>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let Ok((player_entity, player_transform, player_radius)) = player_query.get_single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for (enemy_entity, enemy_transform, enemy_radius) in enemy_query.iter() {
        let dist = player_pos.distance(enemy_transform.translation.truncate());
        if dist < player_radius.0 + enemy_radius.0 {
            if let Ok(mut shield) = shield_query.get_mut(player_entity) {
                shield.current = 0.0;
            }
            damage_events.send(DamageEvent { target: player_entity, amount: 30.0 });
            damage_events.send(DamageEvent { target: enemy_entity, amount: 99999.0 });
            break;
        }
    }
}

// ── Sniper: carregamento e aviso visual ──────────────────────────────────────

pub fn sniper_system(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut sniper_query: Query<(&Transform, &mut SniperState, &EnemyStats), With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut damage_events: EventWriter<DamageEvent>,
    player_entity_query: Query<(Entity, &crate::combat::components::ColliderRadius), With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let dt = time.delta();

    for (sniper_transform, mut state, stats) in sniper_query.iter_mut() {
        let sniper_pos = sniper_transform.translation.truncate();

        if !state.charging {
            state.cooldown_timer.tick(dt);
            if state.cooldown_timer.finished() {
                // Começa a carregar: salva posição do player e mostra aviso
                state.charging = true;
                state.aim_pos = player_pos;
                state.charge_timer.reset();

                // Aviso visual: pequeno ponto vermelho na posição do player
                commands.spawn((
                    SniperWarning { lifetime: Timer::from_seconds(0.6, TimerMode::Once) },
                    Mesh2d(meshes.add(Circle::new(5.0))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(
                        Color::srgb(8.0, 0.0, 0.0),
                    ))),
                    Transform::from_translation(player_pos.extend(Z_VFX + 1.0)),
                ));
            }
        } else {
            state.charge_timer.tick(dt);
            if state.charge_timer.finished() {
                state.charging = false;
                state.cooldown_timer.reset();

                // Dispara: verifica se player está próximo da posição mirada
                let Ok((player_entity, player_collider)) = player_entity_query.get_single() else {
                    continue;
                };

                let dist_to_aim = player_transform.translation.truncate().distance(state.aim_pos);
                let _ = sniper_pos; // silencia warning
                // Se o player não se moveu muito longe, acerta
                if dist_to_aim < 60.0 {
                    damage_events.send(DamageEvent {
                        target: player_entity,
                        amount: stats.damage * 3.0, // dano alto
                    });
                }
            }
        }
    }
}

/// Remove os avisos visuais do Sniper.
pub fn update_sniper_warnings(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SniperWarning, &mut Transform)>,
) {
    for (entity, mut warning, mut transform) in query.iter_mut() {
        warning.lifetime.tick(time.delta());
        // Pulsa (cresce e encolhe)
        let pulse = (warning.lifetime.fraction() * std::f32::consts::TAU).sin().abs();
        transform.scale = Vec3::splat(0.5 + pulse * 0.8);

        if warning.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ── Bomber: explode ao contato com o player ──────────────────────────────────

pub fn bomber_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<(Entity, &Transform, &crate::combat::components::ColliderRadius), With<Player>>,
    mut bomber_query: Query<(Entity, &Transform, &mut BomberData, &ColliderRadius), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut shake: ResMut<ScreenShake>,
) {
    let Ok((player_entity, player_transform, player_col)) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (bomber_entity, bomber_transform, mut bomber, bomber_col) in bomber_query.iter_mut() {
        if bomber.triggered {
            continue;
        }

        let dist = player_pos.distance(bomber_transform.translation.truncate());
        if dist < player_col.0 + bomber_col.0 + bomber.explosion_radius {
            bomber.triggered = true;

            // Dano ao player
            damage_events.send(DamageEvent {
                target: player_entity,
                amount: bomber.explosion_damage,
            });

            // Partículas de explosão laranja intensa
            let pos = bomber_transform.translation.truncate();
            let mesh = meshes.add(Circle::new(3.0));
            let mut rng = rand::thread_rng();
            for _ in 0..30 {
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let speed = rng.gen_range(100.0..280.0_f32);
                let vel = Vec2::new(angle.cos(), angle.sin()) * speed;
                commands.spawn((
                    Particle {
                        velocity: vel,
                        lifetime: Timer::from_seconds(0.7, TimerMode::Once),
                        initial_scale: 1.0,
                        fade: true,
                    },
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(
                        Color::srgb(8.0, 3.0, 0.0),
                    ))),
                    Transform::from_translation(pos.extend(Z_VFX)),
                ));
            }

            shake.add_trauma(0.5);
            commands.entity(bomber_entity).despawn();
        }
    }
}

// ── Splitter: divide ao atingir 50% HP ───────────────────────────────────────

pub fn splitter_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Health, &mut SplitterData, &Transform), With<Enemy>>,
) {
    for (entity, health, mut splitter, transform) in query.iter_mut() {
        if splitter.has_split {
            continue;
        }
        if health.current <= health.max * splitter.split_threshold && health.current > 0.0 {
            splitter.has_split = true;
            let pos = transform.translation.truncate();

            // Spawna 2 Swarmers menores
            for offset in [Vec2::new(15.0, 0.0), Vec2::new(-15.0, 0.0)] {
                spawn_enemy(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    EnemyKind::Swarmer,
                    pos + offset,
                );
            }
            // Despawna o Splitter
            commands.entity(entity).despawn();
        }
    }
}

// ── Teleporter: pisca para posição aleatória ─────────────────────────────────

pub fn teleporter_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut query: Query<(&mut Transform, &mut TeleportData), (With<Enemy>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let mut rng = rand::thread_rng();

    for (mut transform, mut teleport) in query.iter_mut() {
        teleport.timer.tick(time.delta());
        if teleport.timer.finished() {
            teleport.timer.reset();

            let r = teleport.teleport_radius;
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let dist = rng.gen_range(r * 0.4..r);
            let new_pos = player_pos + Vec2::new(angle.cos(), angle.sin()) * dist;

            // Garante que está dentro da tela
            let clamped = Vec2::new(
                new_pos.x.clamp(-HALF_W + 20.0, HALF_W - 20.0),
                new_pos.y.clamp(-HALF_H + 20.0, HALF_H - 20.0),
            );

            transform.translation.x = clamped.x;
            transform.translation.y = clamped.y;
        }
    }
}

// ── MinionSpawner: gera Swarmers periodicamente ──────────────────────────────

pub fn minion_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Transform, &mut MinionSpawnData), With<Enemy>>,
    swarmer_count: Query<(), (With<Enemy>, With<crate::enemies::components::EnemyKind>)>,
) {
    let total_enemies = swarmer_count.iter().count();
    let mut rng = rand::thread_rng();

    for (transform, mut spawn_data) in query.iter_mut() {
        spawn_data.timer.tick(time.delta());
        if !spawn_data.timer.finished() {
            continue;
        }
        spawn_data.timer.reset();

        // Limita o total de inimigos no mapa
        if total_enemies >= 30 {
            continue;
        }

        let pos = transform.translation.truncate();
        let offset = Vec2::new(
            rng.gen_range(-50.0..50.0_f32),
            rng.gen_range(-50.0..50.0_f32),
        );

        spawn_enemy(
            &mut commands,
            &mut meshes,
            &mut materials,
            EnemyKind::Swarmer,
            pos + offset,
        );
    }
}
