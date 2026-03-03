use bevy::prelude::*;

use crate::combat::components::{ColliderRadius, DamageEvent};
use crate::constants::{COLOR_BULLET_ENEMY, COLOR_BULLET_PLAYER, Z_BULLET};
use crate::player::components::Player;

use super::components::{Projectile, ProjectileOwner, WeaponCooldown};

/// A nave dispara automaticamente na direção que aponta.
pub fn player_shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Transform, &mut WeaponCooldown, &crate::player::components::ShipStats), With<Player>>,
) {
    let Ok((transform, mut cooldown, stats)) = query.get_single_mut() else {
        return;
    };

    cooldown.0.tick(time.delta());

    if cooldown.0.finished() {
        cooldown.0.reset();

        let direction = (transform.rotation * Vec3::Y).truncate().normalize();
        let spawn_pos = transform.translation + (direction * 20.0).extend(0.0);

        commands.spawn((
            Projectile {
                damage: stats.bullet_damage,
                speed: stats.bullet_speed,
                direction,
                owner: ProjectileOwner::Player,
                lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            },
            ColliderRadius(4.0),
            Mesh2d(meshes.add(Circle::new(3.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(COLOR_BULLET_PLAYER))),
            Transform::from_translation(spawn_pos.with_z(Z_BULLET)),
        ));
    }
}

/// Move projéteis e remove os que expiraram.
pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    for (entity, mut transform, mut projectile) in query.iter_mut() {
        projectile.lifetime.tick(time.delta());
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        let movement = projectile.direction * projectile.speed * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

/// Detecta colisões entre projéteis e seus alvos usando distância.
pub fn projectile_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Projectile, &ColliderRadius)>,
    target_query: Query<(Entity, &Transform, &ColliderRadius), Without<Projectile>>,
    mut damage_events: EventWriter<DamageEvent>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<crate::enemies::components::Enemy>>,
    shield_query: Query<(), With<crate::enemies::components::EnemyShield>>,
    asteroid_query: Query<Entity, With<crate::obstacles::Asteroid>>,
) {
    let player_entity = player_query.get_single().ok();

    for (proj_entity, proj_transform, projectile, proj_radius) in projectile_query.iter() {
        let proj_pos = proj_transform.translation.truncate();

        for (target_entity, target_transform, target_radius) in target_query.iter() {
            // Projéteis do player acertam inimigos; inimigos acertam player
            let valid_target = match projectile.owner {
                // Balas do player acertam inimigos E asteroides
                ProjectileOwner::Player => {
                    enemy_query.contains(target_entity)
                        || asteroid_query.contains(target_entity)
                }
                ProjectileOwner::Enemy => {
                    player_entity.map_or(false, |p| p == target_entity)
                }
            };

            if !valid_target {
                continue;
            }

            let target_pos = target_transform.translation.truncate();
            let distance = proj_pos.distance(target_pos);

            if distance < proj_radius.0 + target_radius.0 {
                // ShieldEnemy absorve 60% do dano de projéteis do player
                let damage = if projectile.owner == ProjectileOwner::Player
                    && shield_query.contains(target_entity)
                {
                    projectile.damage * 0.4
                } else {
                    projectile.damage
                };
                damage_events.send(DamageEvent {
                    target: target_entity,
                    amount: damage,
                });
                commands.entity(proj_entity).despawn();
                break;
            }
        }
    }
}

/// Spawna projéteis de inimigos (atirados em direção ao player).
pub fn enemy_shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut enemy_query: Query<(&Transform, &mut crate::enemies::components::EnemyShootTimer, &crate::enemies::components::EnemyStats)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (transform, mut shoot_timer, stats) in enemy_query.iter_mut() {
        if stats.fire_rate <= 0.0 {
            continue;
        }

        shoot_timer.0.tick(time.delta());
        if !shoot_timer.0.finished() {
            continue;
        }
        shoot_timer.0.reset();

        let enemy_pos = transform.translation.truncate();
        let direction = (player_pos - enemy_pos).normalize_or_zero();

        if direction == Vec2::ZERO {
            continue;
        }

        commands.spawn((
            Projectile {
                damage: stats.damage,
                speed: 250.0,
                direction,
                owner: ProjectileOwner::Enemy,
                lifetime: Timer::from_seconds(3.0, TimerMode::Once),
            },
            ColliderRadius(3.5),
            Mesh2d(meshes.add(Circle::new(3.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(COLOR_BULLET_ENEMY))),
            Transform::from_translation(transform.translation.with_z(Z_BULLET)),
        ));
    }
}
