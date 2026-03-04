use bevy::prelude::*;

use super::components::{DamageEvent, DeathEvent, Energy, Health, Shield, SelfHandlesDespawn};
use crate::enemies::components::Enemy;
use crate::player::components::Player;

/// Drena/regenera escudo e energia a cada frame.
pub fn shield_energy_regen(
    time: Res<Time>,
    mut query: Query<(&mut Shield, &mut Energy)>,
) {
    for (mut shield, mut energy) in query.iter_mut() {
        let dt = time.delta_secs();

        // Regenera energia
        energy.current = (energy.current + energy.regen_rate * dt).min(energy.max);

        // Regenera escudo lentamente quando não está ativo
        if !shield.active {
            shield.current = (shield.current + shield.regen_rate * dt).min(shield.max);
        }
    }
}

/// Aplica dano: escudo absorve primeiro, depois HP.
pub fn apply_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
    mut shield_query: Query<&mut Shield>,
    mut death_events: EventWriter<DeathEvent>,
    transform_query: Query<(&Transform, Option<&Enemy>)>,
) {
    for event in damage_events.read() {
        let Ok(mut health) = health_query.get_mut(event.target) else {
            continue;
        };

        let remaining = if let Ok(mut shield) = shield_query.get_mut(event.target) {
            let shield_absorbed = shield.current.min(event.amount);
            shield.current -= shield_absorbed;
            event.amount - shield_absorbed
        } else {
            event.amount
        };

        health.current -= remaining;

        if health.is_dead() {
            let (pos, enemy_opt) = transform_query
                .get(event.target)
                .map(|(t, e)| (t.translation.truncate(), e))
                .unwrap_or((Vec2::ZERO, None));

            death_events.send(DeathEvent {
                entity: event.target,
                position: pos,
                was_enemy: enemy_opt.is_some(),
            });
        }
    }
}

/// Despacha entidades mortas.
/// Entidades com SelfHandlesDespawn são ignoradas — elas gerenciam seu próprio despawn.
pub fn handle_deaths(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    player_query: Query<Entity, With<Player>>,
    self_handle_query: Query<(), With<SelfHandlesDespawn>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    for event in death_events.read() {
        if let Ok(player_entity) = player_query.get_single() {
            if event.entity == player_entity {
                info!("Player morreu! Game Over.");
                next_state.set(crate::states::GameState::GameOver);
                return;
            }
        }

        // Pula entidades que gerenciam seu próprio despawn (ex: asteroides)
        if self_handle_query.contains(event.entity) {
            continue;
        }

        if let Some(mut cmd) = commands.get_entity(event.entity) {
            cmd.despawn();
        }
    }
}
