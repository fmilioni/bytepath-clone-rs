use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ── Catálogo de itens ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum ItemId {
    ThrusterMk2,
    NanoArmor,
    DeflectorArray,
    CapacitorBank,
    Autoloader,
    WarheadRounds,
    RegenerationCore,
    ShieldAmplifier,
    PowerConduit,
    PickupMagnet,
    HullReinforcement,
    OverclockModule,
}

pub const ALL_ITEMS: &[ItemId] = &[
    ItemId::ThrusterMk2, ItemId::NanoArmor, ItemId::DeflectorArray,
    ItemId::CapacitorBank, ItemId::Autoloader, ItemId::WarheadRounds,
    ItemId::RegenerationCore, ItemId::ShieldAmplifier, ItemId::PowerConduit,
    ItemId::PickupMagnet, ItemId::HullReinforcement, ItemId::OverclockModule,
];

pub struct ItemDef {
    pub id: ItemId,
    pub name: &'static str,
    pub description: &'static str,
    pub cost: u32,
    // Multiplicadores (1.0 = neutro)
    pub speed_mul: f32,
    pub fire_rate_mul: f32,
    pub damage_mul: f32,
    pub bullet_speed_mul: f32,
    pub shield_regen_mul: f32,
    pub energy_regen_mul: f32,
    // Bônus planos
    pub max_hp: f32,
    pub max_shield: f32,
    pub max_energy: f32,
    pub hp_regen: f32,
    pub pickup_radius: f32,
}

fn base_def(id: ItemId, name: &'static str, description: &'static str, cost: u32) -> ItemDef {
    ItemDef {
        id, name, description, cost,
        speed_mul: 1.0, fire_rate_mul: 1.0, damage_mul: 1.0, bullet_speed_mul: 1.0,
        shield_regen_mul: 1.0, energy_regen_mul: 1.0,
        max_hp: 0.0, max_shield: 0.0, max_energy: 0.0, hp_regen: 0.0, pickup_radius: 0.0,
    }
}

impl ItemId {
    pub fn def(self) -> ItemDef {
        match self {
            ItemId::ThrusterMk2 =>
                ItemDef { speed_mul: 1.25, ..base_def(self, "Thruster Mk2", "+25% velocidade", 200) },
            ItemId::NanoArmor =>
                ItemDef { max_hp: 70.0, ..base_def(self, "Nano-Armadura", "+70 HP maximo", 150) },
            ItemId::DeflectorArray =>
                ItemDef { max_shield: 60.0, ..base_def(self, "Array Deflector", "+60 escudo maximo", 175) },
            ItemId::CapacitorBank =>
                ItemDef { max_energy: 50.0, ..base_def(self, "Banco de Capacitores", "+50 energia maxima", 130) },
            ItemId::Autoloader =>
                ItemDef { fire_rate_mul: 1.4, ..base_def(self, "Autoloader", "+40% cadencia de tiro", 225) },
            ItemId::WarheadRounds =>
                ItemDef { damage_mul: 1.7, ..base_def(self, "Balas Explosivas", "+70% dano de bala", 250) },
            ItemId::RegenerationCore =>
                ItemDef { hp_regen: 6.0, ..base_def(self, "Core de Regeneracao", "+6 HP/s regeneracao", 300) },
            ItemId::ShieldAmplifier =>
                ItemDef { shield_regen_mul: 2.0, ..base_def(self, "Amplificador de Escudo", "+100% recarga de escudo", 225) },
            ItemId::PowerConduit =>
                ItemDef { energy_regen_mul: 1.8, ..base_def(self, "Conduto de Energia", "+80% recarga de energia", 200) },
            ItemId::PickupMagnet =>
                ItemDef { pickup_radius: 150.0, ..base_def(self, "Magneto de Pickup", "+150 raio de coleta", 100) },
            ItemId::HullReinforcement =>
                ItemDef { max_hp: 50.0, max_shield: 30.0, ..base_def(self, "Reforco de Casco", "+50 HP e +30 escudo", 275) },
            ItemId::OverclockModule =>
                ItemDef { fire_rate_mul: 1.2, bullet_speed_mul: 1.2, ..base_def(self, "Modulo Overclock", "+20% cadencia e +20% vel. bala", 325) },
        }
    }
}

// ── Bônus agregado de todos os itens equipados ────────────────────────────────

pub struct ItemBonus {
    pub speed_mul: f32,
    pub fire_rate_mul: f32,
    pub damage_mul: f32,
    pub bullet_speed_mul: f32,
    pub shield_regen_mul: f32,
    pub energy_regen_mul: f32,
    pub max_hp: f32,
    pub max_shield: f32,
    pub max_energy: f32,
    pub hp_regen: f32,
    pub pickup_radius: f32,
}

impl Default for ItemBonus {
    fn default() -> Self {
        Self {
            speed_mul: 1.0, fire_rate_mul: 1.0, damage_mul: 1.0, bullet_speed_mul: 1.0,
            shield_regen_mul: 1.0, energy_regen_mul: 1.0,
            max_hp: 0.0, max_shield: 0.0, max_energy: 0.0, hp_regen: 0.0, pickup_radius: 0.0,
        }
    }
}

// ── Recursos ──────────────────────────────────────────────────────────────────

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct Credits(pub u32);

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct PlayerInventory {
    pub items: Vec<ItemId>,
}

impl PlayerInventory {
    pub fn combined_bonus(&self) -> ItemBonus {
        let mut b = ItemBonus::default();
        for &id in &self.items {
            let d = id.def();
            b.speed_mul       *= d.speed_mul;
            b.fire_rate_mul   *= d.fire_rate_mul;
            b.damage_mul      *= d.damage_mul;
            b.bullet_speed_mul *= d.bullet_speed_mul;
            b.shield_regen_mul *= d.shield_regen_mul;
            b.energy_regen_mul *= d.energy_regen_mul;
            b.max_hp          += d.max_hp;
            b.max_shield      += d.max_shield;
            b.max_energy      += d.max_energy;
            b.hp_regen        += d.hp_regen;
            b.pickup_radius   += d.pickup_radius;
        }
        b
    }
}

#[derive(Resource, Default)]
pub struct ShopUiState {
    pub open: bool,
    pub offered: Vec<ItemId>,
    pub selected: usize,
}
