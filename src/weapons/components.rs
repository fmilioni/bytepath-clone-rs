use bevy::prelude::*;

/// Quem disparou o projétil.
#[derive(Component, PartialEq, Clone, Copy)]
pub enum ProjectileOwner {
    Player,
    Enemy,
}

/// Projétil em movimento.
#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub direction: Vec2,
    pub owner: ProjectileOwner,
    pub lifetime: Timer,
}

/// Cooldown de disparo da nave do jogador.
#[derive(Component)]
pub struct WeaponCooldown(pub Timer);

/// Munição especial (limitada).
#[derive(Component)]
pub struct SpecialAmmo {
    pub kind: SpecialAmmoKind,
    pub count: u32,
    pub max: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpecialAmmoKind {
    AreaExplosion,
    StunExplosion,
    Electric,
    Nuclear,
    BlackHole,
    Plasma,
    Ice,
}
