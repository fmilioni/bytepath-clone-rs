use bevy::prelude::*;

/// Pontos de vida do entity.
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

/// Escudo que absorve dano antes do HP (consome energia).
#[derive(Component)]
pub struct Shield {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,   // por segundo
    pub active: bool,      // true = player está segurando Shift
}

impl Shield {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            regen_rate: 5.0,
            active: false,
        }
    }
}

/// Energia usada pelo escudo e habilidades especiais.
#[derive(Component)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

impl Energy {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            regen_rate: 8.0,
        }
    }
}

/// Raio de colisão do entity (para detecção por distância).
#[derive(Component)]
pub struct ColliderRadius(pub f32);

/// Marker: esta entidade gerencia seu próprio despawn na morte.
/// handle_deaths pula o despawn de entidades com este componente.
#[derive(Component)]
pub struct SelfHandlesDespawn;

/// Evento: um entity sofreu dano.
#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
}

/// Evento: um entity morreu.
#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub was_enemy: bool,
}
