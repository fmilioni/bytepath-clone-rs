use bevy::prelude::*;

/// Partícula individual (trail, explosão, etc).
#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub initial_scale: f32,
    pub fade: bool, // reduz alpha com o tempo
}

/// Segmento de trail do motor da nave.
#[derive(Component)]
pub struct TrailSegment {
    pub lifetime: Timer,
}

/// Resource: intensidade atual do screen shake (trauma system).
#[derive(Resource, Default)]
pub struct ScreenShake {
    /// 0.0 = sem shake, 1.0 = shake máximo.
    pub trauma: f32,
    /// Semente para ruído do shake.
    pub time_acc: f32,
}

impl ScreenShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }
}

/// Marcador da câmera de jogo (para aplicar screen shake).
#[derive(Component)]
pub struct GameCamera;

/// Estrela do fundo (paralaxe suave).
#[derive(Component)]
pub struct Star {
    pub parallax_factor: f32, // 0.0 = imóvel, 1.0 = move com câmera
}

/// Anel visual do escudo — filho do player, visível quando shield.current > 0.
#[derive(Component)]
pub struct ShieldRing;
