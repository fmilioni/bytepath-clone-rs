use bevy::prelude::*;

use crate::combat::components::{ColliderRadius, DamageEvent, Energy};
use crate::enemies::components::Enemy;

use super::components::Player;
use super::ship_classes::ShipClass;

// ── Stealth Cloak ─────────────────────────────────────────────────────────────

/// Componente presente apenas na Stealth.
#[derive(Component, Default)]
pub struct StealthAbility {
    pub cloaked: bool,
}

const CLOAK_ENERGY_COST: f32 = 25.0; // energia/segundo enquanto cloakado

/// Sistema de cloaking da Stealth (Q mantido = invisível e sem colisão inimiga).
pub fn stealth_cloak_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (&mut Visibility, &mut StealthAbility, &mut Energy),
        With<Player>,
    >,
) {
    let Ok((mut visibility, mut ability, mut energy)) = query.get_single_mut() else {
        return;
    };

    let want_cloak = keys.pressed(KeyCode::KeyQ);

    if want_cloak && energy.current >= CLOAK_ENERGY_COST * time.delta_secs() {
        ability.cloaked = true;
        energy.current -= CLOAK_ENERGY_COST * time.delta_secs();
        *visibility = Visibility::Hidden;
    } else {
        ability.cloaked = false;
        *visibility = Visibility::Visible;
    }
}

// ── Brawler Melee Aura ───────────────────────────────────────────────────────

/// Componente presente apenas no Brawler.
#[derive(Component)]
pub struct MeleeAura {
    pub damage_per_second: f32,
    pub radius: f32,
}

/// Sistema de aura corpo-a-corpo passiva do Brawler.
pub fn brawler_melee_system(
    time: Res<Time>,
    player_query: Query<(&Transform, &ColliderRadius, &MeleeAura), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &ColliderRadius), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let Ok((player_transform, _, melee)) = player_query.get_single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let dt = time.delta_secs();

    for (enemy_entity, enemy_transform, enemy_radius) in enemy_query.iter() {
        let dist = player_pos.distance(enemy_transform.translation.truncate());
        if dist < melee.radius + enemy_radius.0 {
            damage_events.send(DamageEvent {
                target: enemy_entity,
                amount: melee.damage_per_second * dt,
            });
        }
    }
}

// ── Helper: adiciona componente de habilidade conforme a classe ──────────────

pub fn attach_class_ability(
    commands: &mut Commands,
    entity: Entity,
    class: ShipClass,
) {
    match class {
        ShipClass::Stealth => {
            commands.entity(entity).insert(StealthAbility::default());
        }
        ShipClass::Brawler => {
            commands.entity(entity).insert(MeleeAura {
                damage_per_second: 40.0,
                radius: 40.0,
            });
        }
        _ => {}
    }
}
