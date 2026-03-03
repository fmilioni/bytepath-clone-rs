use bevy::prelude::*;

use crate::combat::components::{Energy, Health, Shield};
use crate::player::components::ShipStats;
use crate::shop::components::{ItemBonus, PlayerInventory};
use crate::skill_tree::components::PlayerSkills;
use crate::weapons::components::WeaponCooldown;

/// Computa stats finais: base da classe → skills → itens.
pub fn compute_full_stats(base: &ShipStats, skills: &PlayerSkills, items: &ItemBonus) -> ShipStats {
    let s = skills.compute_stats(base);
    ShipStats {
        speed:         s.speed         * items.speed_mul,
        max_hp:        s.max_hp        + items.max_hp,
        fire_rate:     s.fire_rate     * items.fire_rate_mul,
        bullet_speed:  s.bullet_speed  * items.bullet_speed_mul,
        bullet_damage: s.bullet_damage * items.damage_mul,
        cargo_slots:   s.cargo_slots,
        max_shield:    s.max_shield    + items.max_shield,
        max_energy:    s.max_energy    + items.max_energy,
        energy_regen:  s.energy_regen  * items.energy_regen_mul,
    }
}

/// Aplica o conjunto completo de stats (skills + itens) a um jogador vivo.
pub fn apply_full_stats(
    skills: &PlayerSkills,
    inventory: &PlayerInventory,
    base: &ShipStats,
    stats: &mut ShipStats,
    health: &mut Health,
    shield: &mut Shield,
    energy: &mut Energy,
    cooldown: &mut WeaponCooldown,
) {
    let items = inventory.combined_bonus();
    let new = compute_full_stats(base, skills, &items);

    let hp_delta = new.max_hp - stats.max_hp;
    let sh_delta = new.max_shield - stats.max_shield;
    let en_delta = new.max_energy - stats.max_energy;

    health.max = new.max_hp;
    health.current = (health.current + hp_delta.max(0.0)).min(health.max);
    shield.max = new.max_shield;
    shield.current = (shield.current + sh_delta.max(0.0)).min(shield.max);
    energy.max = new.max_energy;
    energy.current = (energy.current + en_delta.max(0.0)).min(energy.max);

    shield.regen_rate = 5.0 * skills.shield_regen_mul * items.shield_regen_mul;
    energy.regen_rate = new.energy_regen;

    *stats = new;
    cooldown.0 = Timer::from_seconds(1.0 / stats.fire_rate, TimerMode::Once);
}
