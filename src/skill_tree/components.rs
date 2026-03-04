use std::collections::HashSet;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::player::components::ShipStats;
use super::data::{SkillEffect, all_nodes};

/// Estado global da skill tree: pontos, nós desbloqueados e bônus calculados.
#[derive(Resource)]
pub struct PlayerSkills {
    pub skill_points: u32,
    pub unlocked: HashSet<u32>,
    // Multiplicadores acumulados (aplicados aos stats base da nave)
    pub speed_mul: f32,
    pub damage_mul: f32,
    pub fire_rate_mul: f32,
    pub bullet_speed_mul: f32,
    // Bônus planos acumulados
    pub hp_bonus: f32,
    pub shield_bonus: f32,
    pub energy_bonus: f32,
    pub pickup_radius: f32,
    // Efeitos passivos por segundo
    pub hp_regen: f32,
    pub shield_regen_mul: f32,
    pub energy_regen_mul: f32,
}

impl Default for PlayerSkills {
    fn default() -> Self {
        Self {
            skill_points: 0,
            unlocked: HashSet::new(),
            speed_mul: 1.0,
            damage_mul: 1.0,
            fire_rate_mul: 1.0,
            bullet_speed_mul: 1.0,
            hp_bonus: 0.0,
            shield_bonus: 0.0,
            energy_bonus: 0.0,
            pickup_radius: 50.0,
            hp_regen: 0.0,
            shield_regen_mul: 1.0,
            energy_regen_mul: 1.0,
        }
    }
}

impl PlayerSkills {
    /// Calcula os stats finais do player combinando stats base da classe com os bônus da skill tree.
    pub fn compute_stats(&self, base: &ShipStats) -> ShipStats {
        ShipStats {
            speed: base.speed * self.speed_mul,
            max_hp: base.max_hp + self.hp_bonus,
            fire_rate: base.fire_rate * self.fire_rate_mul,
            bullet_speed: base.bullet_speed * self.bullet_speed_mul,
            bullet_damage: base.bullet_damage * self.damage_mul,
            cargo_slots: base.cargo_slots,
            max_shield: base.max_shield + self.shield_bonus,
            max_energy: base.max_energy + self.energy_bonus,
            energy_regen: base.energy_regen * self.energy_regen_mul,
        }
    }

    /// Recalcula todos os multiplicadores a partir dos nós desbloqueados (para respec ou carregamento).
    pub fn recalculate(&mut self) {
        self.speed_mul = 1.0;
        self.damage_mul = 1.0;
        self.fire_rate_mul = 1.0;
        self.bullet_speed_mul = 1.0;
        self.hp_bonus = 0.0;
        self.shield_bonus = 0.0;
        self.energy_bonus = 0.0;
        self.pickup_radius = 50.0;
        self.hp_regen = 0.0;
        self.shield_regen_mul = 1.0;
        self.energy_regen_mul = 1.0;

        for node in all_nodes() {
            if self.unlocked.contains(&node.id) {
                apply_effect_to_skills(self, &node.effect);
            }
        }
    }
}

/// Aplica o efeito de um nó incrementalmente.
pub fn apply_effect_to_skills(skills: &mut PlayerSkills, effect: &SkillEffect) {
    match *effect {
        SkillEffect::SpeedMul(v)       => skills.speed_mul *= v,
        SkillEffect::DamageMul(v)      => skills.damage_mul *= v,
        SkillEffect::FireRateMul(v)    => skills.fire_rate_mul *= v,
        SkillEffect::BulletSpeedMul(v) => skills.bullet_speed_mul *= v,
        SkillEffect::MaxHpFlat(v)      => skills.hp_bonus += v,
        SkillEffect::MaxShieldFlat(v)  => skills.shield_bonus += v,
        SkillEffect::MaxEnergyFlat(v)  => skills.energy_bonus += v,
        SkillEffect::PickupRadius(v)   => skills.pickup_radius += v,
        SkillEffect::HpRegen(v)        => skills.hp_regen += v,
        SkillEffect::ShieldRegenMul(v) => skills.shield_regen_mul *= v,
        SkillEffect::EnergyRegenMul(v) => skills.energy_regen_mul *= v,
    }
}

/// Estado da UI da skill tree (navegação).
#[derive(Resource, Default)]
pub struct SkillTreeUiState {
    pub open: bool,
    pub selected_cluster: usize,
    pub selected_node_idx: usize,
    pub scroll_offset: usize,
}

/// Evento disparado quando um nó é desbloqueado.
#[derive(Event)]
pub struct SkillNodeUnlocked(pub u32);
