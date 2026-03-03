use bevy::prelude::*;

/// Marcador de boss — entidade principal.
#[derive(Component)]
pub struct Boss;

/// Qual boss é este.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BossKind {
    /// Região 1 — muros de escudo rotativos.
    Sentinel,
    /// Região 2 — ondas infinitas de minions.
    Swarmmother,
    /// Região 3 — barrage de artilharia em leque.
    Dreadnought,
    /// Região 4 — teleporte + clones decoy.
    Phantom,
    /// Região 5 — buraco negro com gravidade.
    Singularity,
}

/// Fase atual do boss (1 → 2 ao atingir 66% HP; 2 → 3 ao atingir 33% HP).
#[derive(Component)]
pub struct BossPhase {
    pub phase: u8,
}

impl BossPhase {
    pub fn new() -> Self {
        Self { phase: 1 }
    }
}

// ── Dados específicos por boss ────────────────────────────────────────────────

/// Sentinel: rotação dos painéis de escudo + ataque em leque.
#[derive(Component)]
pub struct SentinelData {
    pub shield_angle: f32,   // ângulo atual dos painéis
    pub rotate_speed: f32,   // rad/s
    pub attack_timer: Timer, // tempo entre disparos
    pub panel_count: u32,    // número de painéis ativos
}

/// Tag de um painel de escudo (entidade filha do Sentinel).
#[derive(Component)]
pub struct SentinelShieldPanel {
    pub angle_offset: f32, // deslocamento angular fixo entre painéis
    pub boss_entity: Entity,
}

/// Swarmmother: ondas de minions + disparo ocasional.
#[derive(Component)]
pub struct SwarmmotherData {
    pub wave_timer: Timer,
    pub minions_per_wave: u32,
    pub shoot_timer: Timer,
}

/// Dreadnought: barrage em leque.
#[derive(Component)]
pub struct DreadnoughtData {
    pub barrage_timer: Timer,
    pub barrage_count: u32, // projéteis por disparo
    pub nova_timer: Timer,  // disparo 360° — fase 3
}

/// Phantom: teleporte + clones decoy.
#[derive(Component)]
pub struct PhantomData {
    pub teleport_timer: Timer,
    pub clone_count: u32, // clones que devem existir em cena
}

/// Marcador de clone decoy do Phantom.
#[derive(Component)]
pub struct PhantomClone;

/// Singularity: campo gravitacional + pulso de nova.
#[derive(Component)]
pub struct SingularityData {
    pub pull_force: f32,
    pub pulse_timer: Timer,  // pulso visual
    pub nova_timer: Timer,   // explosão de energia que empurra
    pub spawn_timer: Timer,  // spawn de inimigos/asteroides — fase 3
}

/// Evento: boss entrou em nova fase.
#[derive(Event)]
pub struct BossPhaseTransition {
    pub boss_entity: Entity,
    pub new_phase: u8,
    pub boss_kind: BossKind,
}

/// Resource: entidade do boss ativo (None = sem boss em cena).
#[derive(Resource, Default)]
pub struct ActiveBoss(pub Option<Entity>);
