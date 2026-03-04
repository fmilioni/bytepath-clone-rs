use bevy::prelude::*;

/// Marcador da entidade principal do jogador.
#[derive(Component, Default)]
pub struct Player;

/// Estatísticas base da nave (modificadas por upgrades e skill tree).
#[derive(Component)]
pub struct ShipStats {
    pub speed: f32,
    pub max_hp: f32,
    pub fire_rate: f32,       // tiros por segundo
    pub bullet_speed: f32,
    pub bullet_damage: f32,
    pub cargo_slots: u32,
    pub max_shield: f32,
    pub max_energy: f32,
    pub energy_regen: f32,    // energia regenerada por segundo
}

impl Default for ShipStats {
    fn default() -> Self {
        Self {
            speed: 280.0,
            max_hp: 100.0,
            fire_rate: 5.0,
            bullet_speed: 500.0,
            bullet_damage: 10.0,
            cargo_slots: 3,
            max_shield: 60.0,
            max_energy: 100.0,
            energy_regen: 10.0,
        }
    }
}

/// Estado de rotação da nave em radianos (ângulo atual).
#[derive(Component, Default)]
pub struct ShipRotation(pub f32);

/// Cooldown de invencibilidade após levar dano por contato com inimigo.
/// A colisão com meteoro ignora esse timer — só contato de corpo inimigo verifica.
#[derive(Component, Default)]
pub struct EnemyHitCooldown {
    pub remaining: f32, // segundos restantes; 0.0 = pode levar dano
}

/// Controle de aceleração/freio da nave.
/// W aumenta (consome energia), S diminui, sem tecla volta a 1.0.
#[derive(Component)]
pub struct PlayerThrottle {
    pub multiplier: f32, // 0.25 (freio) a 2.0 (boost máximo)
}

impl Default for PlayerThrottle {
    fn default() -> Self {
        Self { multiplier: 1.0 }
    }
}
