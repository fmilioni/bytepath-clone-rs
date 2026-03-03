use bevy::prelude::*;

use crate::weapons::components::SpecialAmmoKind;

// ── Resource: classe selecionada pelo jogador ────────────────────────────────

#[derive(Resource, Default, Clone)]
pub struct SelectedShipClass(pub ShipClass);

// ── Enum das 8 classes ───────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum ShipClass {
    #[default]
    Balanced,
    FastScout,
    SlowTank,
    CargoHauler,
    GlassCannon,
    SupportEnergy,
    Stealth,
    Brawler,
}

pub const ALL_CLASSES: [ShipClass; 8] = [
    ShipClass::Balanced,
    ShipClass::FastScout,
    ShipClass::SlowTank,
    ShipClass::CargoHauler,
    ShipClass::GlassCannon,
    ShipClass::SupportEnergy,
    ShipClass::Stealth,
    ShipClass::Brawler,
];

// ── Dados de cada classe ─────────────────────────────────────────────────────

pub struct ShipClassData {
    pub name: &'static str,
    pub description: &'static str,
    pub speed: f32,
    pub max_hp: f32,
    pub max_shield: f32,
    pub max_energy: f32,
    pub energy_regen: f32,    // energia/segundo
    pub fire_rate: f32,
    pub damage_multiplier: f32,
    pub bullet_speed: f32,
    pub cargo_slots: u32,
    pub collider_radius: f32,
    pub special_ammo_kind: SpecialAmmoKind,
    pub special_ammo_max: u32,
    pub color: Color,
}

impl ShipClass {
    pub fn data(&self) -> ShipClassData {
        match self {
            ShipClass::Balanced => ShipClassData {
                name: "BALANCED",
                description: "Nave versátil. Boa em tudo, excelente em nada.",
                speed: 160.0,
                max_hp: 100.0,
                max_shield: 60.0,
                max_energy: 100.0,
                energy_regen: 10.0,
                fire_rate: 5.0,
                damage_multiplier: 1.0,
                bullet_speed: 500.0,
                cargo_slots: 3,
                collider_radius: 14.0,
                special_ammo_kind: SpecialAmmoKind::AreaExplosion,
                special_ammo_max: 3,
                color: Color::srgb(0.0, 4.0, 8.0), // ciano
            },
            ShipClass::FastScout => ShipClassData {
                name: "FAST SCOUT",
                description: "Velocidade extrema. Ativa stun e foge antes do impacto.",
                speed: 270.0,
                max_hp: 55.0,
                max_shield: 30.0,
                max_energy: 80.0,
                energy_regen: 16.0,
                fire_rate: 7.0,
                damage_multiplier: 0.8,
                bullet_speed: 620.0,
                cargo_slots: 1,
                collider_radius: 10.0,
                special_ammo_kind: SpecialAmmoKind::StunExplosion,
                special_ammo_max: 5,
                color: Color::srgb(4.0, 8.0, 0.0), // verde-lima
            },
            ShipClass::SlowTank => ShipClassData {
                name: "SLOW TANK",
                description: "Lenta e massiva. Explosões de área devastadoras.",
                speed: 85.0,
                max_hp: 280.0,
                max_shield: 150.0,
                max_energy: 120.0,
                energy_regen: 7.0,
                fire_rate: 2.5,
                damage_multiplier: 2.2,
                bullet_speed: 380.0,
                cargo_slots: 2,
                collider_radius: 22.0,
                special_ammo_kind: SpecialAmmoKind::AreaExplosion,
                special_ammo_max: 2,
                color: Color::srgb(6.0, 0.5, 0.0), // laranja escuro
            },
            ShipClass::CargoHauler => ShipClassData {
                name: "CARGO HAULER",
                description: "Muitos slots de carga. Carrega mais, combate médio.",
                speed: 130.0,
                max_hp: 110.0,
                max_shield: 60.0,
                max_energy: 100.0,
                energy_regen: 10.0,
                fire_rate: 4.0,
                damage_multiplier: 0.9,
                bullet_speed: 460.0,
                cargo_slots: 6,
                collider_radius: 18.0,
                special_ammo_kind: SpecialAmmoKind::Ice,
                special_ammo_max: 4,
                color: Color::srgb(0.5, 3.0, 5.0), // azul aço
            },
            ShipClass::GlassCannon => ShipClassData {
                name: "GLASS CANNON",
                description: "Dano absurdo. Morre com dois tiros. Risco máximo.",
                speed: 175.0,
                max_hp: 30.0,
                max_shield: 10.0,
                max_energy: 90.0,
                energy_regen: 14.0,
                fire_rate: 8.0,
                damage_multiplier: 4.0,
                bullet_speed: 650.0,
                cargo_slots: 2,
                collider_radius: 11.0,
                special_ammo_kind: SpecialAmmoKind::Nuclear,
                special_ammo_max: 2,
                color: Color::srgb(8.0, 0.0, 4.0), // rosa neon
            },
            ShipClass::SupportEnergy => ShipClassData {
                name: "SUPPORT ENERGY",
                description: "Escudo imenso, energia abundante. Dano elétrico em cadeia.",
                speed: 150.0,
                max_hp: 90.0,
                max_shield: 200.0,
                max_energy: 250.0,
                energy_regen: 28.0,
                fire_rate: 4.5,
                damage_multiplier: 0.85,
                bullet_speed: 470.0,
                cargo_slots: 3,
                collider_radius: 13.0,
                special_ammo_kind: SpecialAmmoKind::Electric,
                special_ammo_max: 6,
                color: Color::srgb(2.0, 0.0, 8.0), // roxo neon
            },
            ShipClass::Stealth => ShipClassData {
                name: "STEALTH",
                description: "Cloaking ativo (Q). Ataque furtivo com Plasma.",
                speed: 190.0,
                max_hp: 80.0,
                max_shield: 50.0,
                max_energy: 180.0,
                energy_regen: 22.0,
                fire_rate: 6.0,
                damage_multiplier: 1.8,
                bullet_speed: 540.0,
                cargo_slots: 2,
                collider_radius: 12.0,
                special_ammo_kind: SpecialAmmoKind::Plasma,
                special_ammo_max: 4,
                color: Color::srgb(0.0, 6.0, 4.0), // verde escuro
            },
            ShipClass::Brawler => ShipClassData {
                name: "BRAWLER",
                description: "Corpo enorme e resistente. Aura de dano corpo-a-corpo passiva.",
                speed: 110.0,
                max_hp: 450.0,
                max_shield: 80.0,
                max_energy: 100.0,
                energy_regen: 8.0,
                fire_rate: 3.0,
                damage_multiplier: 1.5,
                bullet_speed: 400.0,
                cargo_slots: 2,
                collider_radius: 20.0,
                special_ammo_kind: SpecialAmmoKind::BlackHole,
                special_ammo_max: 2,
                color: Color::srgb(6.0, 2.0, 0.0), // vermelho escuro
            },
        }
    }

    /// Constrói o mesh da nave (geometria distinta por classe).
    pub fn build_mesh(&self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        match self {
            ShipClass::Balanced => meshes.add(Triangle2d::new(
                Vec2::new(0.0, 14.0),
                Vec2::new(-8.0, -10.0),
                Vec2::new(8.0, -10.0),
            )),
            ShipClass::FastScout => meshes.add(Triangle2d::new(
                Vec2::new(0.0, 20.0),   // alongado
                Vec2::new(-5.0, -9.0),
                Vec2::new(5.0, -9.0),
            )),
            ShipClass::SlowTank => meshes.add(RegularPolygon::new(22.0, 6)), // hexágono
            ShipClass::CargoHauler => meshes.add(Rectangle::new(26.0, 16.0)), // retangular
            ShipClass::GlassCannon => meshes.add(Triangle2d::new(
                Vec2::new(0.0, 24.0),   // muito fino e longo
                Vec2::new(-4.0, -12.0),
                Vec2::new(4.0, -12.0),
            )),
            ShipClass::SupportEnergy => meshes.add(RegularPolygon::new(14.0, 5)), // pentágono
            ShipClass::Stealth => meshes.add(Triangle2d::new(
                Vec2::new(0.0, 16.0),
                Vec2::new(-16.0, -8.0), // asa delta larga
                Vec2::new(16.0, -8.0),
            )),
            ShipClass::Brawler => meshes.add(RegularPolygon::new(20.0, 8)), // octógono
        }
    }

    /// Converte os dados da classe para ShipStats.
    pub fn to_ship_stats(&self) -> crate::player::components::ShipStats {
        let d = self.data();
        crate::player::components::ShipStats {
            speed: d.speed,
            max_hp: d.max_hp,
            fire_rate: d.fire_rate,
            bullet_speed: d.bullet_speed,
            bullet_damage: 10.0 * d.damage_multiplier,
            cargo_slots: d.cargo_slots,
            max_shield: d.max_shield,
            max_energy: d.max_energy,
            energy_regen: d.energy_regen,
        }
    }

    /// Próxima classe (cicla na lista).
    pub fn next(&self) -> ShipClass {
        let idx = ALL_CLASSES.iter().position(|c| c == self).unwrap_or(0);
        ALL_CLASSES[(idx + 1) % ALL_CLASSES.len()]
    }

    /// Classe anterior (cicla na lista).
    pub fn prev(&self) -> ShipClass {
        let idx = ALL_CLASSES.iter().position(|c| c == self).unwrap_or(0);
        ALL_CLASSES[(idx + ALL_CLASSES.len() - 1) % ALL_CLASSES.len()]
    }
}
