use bevy::prelude::*;

/// Marcador genérico para todas as aeronaves inimigas.
#[derive(Component)]
pub struct Enemy;

/// Tipo do inimigo — define qual AI system o controla.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyKind {
    /// Mantém distância e atira no player.
    Shooter,
    /// Avança em linha reta em alta velocidade.
    Charger,
    /// Pequeno, corre direto ao player em grupos.
    Swarmer,
    /// Orbita o player em raio fixo, atira ocasionalmente.
    Circler,
    /// Lento, explode em AOE ao encostar no player.
    Bomber,
    /// Longo alcance, carrega e atira projétil rápido com aviso visual.
    Sniper,
    /// Tem escudo direcional que absorve metade do dano de bala.
    ShieldEnemy,
    /// Quando HP cai abaixo de 50%, divide em 2 Swarmers menores.
    Splitter,
    /// Teleporta para posição aleatória próxima ao player periodicamente.
    Teleporter,
    /// Fica recuado e gera Swarmers periodicamente.
    MinionSpawner,
}

/// Estatísticas base do inimigo.
#[derive(Component, Clone)]
pub struct EnemyStats {
    pub speed: f32,
    pub damage: f32,
    pub fire_rate: f32,
    pub max_hp: f32,
    pub score: u32,
}

/// Timer de disparo (inimigos que atiram).
#[derive(Component)]
pub struct EnemyShootTimer(pub Timer);

/// Timer de ação de IA (teleporte, spawn de minion, etc).
#[derive(Component)]
pub struct EnemyAITimer(pub Timer);

// ── Componentes especializados por tipo ──────────────────────────────────────

/// Circler: ângulo atual de órbita.
#[derive(Component)]
pub struct CircleOrbit {
    pub angle: f32,
    pub radius: f32,
    pub angular_speed: f32,
}

/// Bomber: raio e dano da explosão ao contato.
#[derive(Component)]
pub struct BomberData {
    pub explosion_radius: f32,
    pub explosion_damage: f32,
    pub triggered: bool, // garante que explode só uma vez
}

/// Sniper: estado do carregamento antes do disparo.
#[derive(Component)]
pub struct SniperState {
    pub charging: bool,
    pub charge_timer: Timer,
    pub cooldown_timer: Timer,
    pub aim_pos: Vec2, // posição do player quando começou a carregar
}

/// ShieldEnemy: absorve 60% do dano de projéteis.
#[derive(Component)]
pub struct EnemyShield;

/// Splitter: rastrea se já dividiu (para não dividir duas vezes).
#[derive(Component)]
pub struct SplitterData {
    pub has_split: bool,
    pub split_threshold: f32, // fração de HP para dividir (ex: 0.5)
}

/// Teleporter: timer de próximo teleporte.
#[derive(Component)]
pub struct TeleportData {
    pub timer: Timer,
    pub teleport_radius: f32,
}

/// MinionSpawner: timer de spawn + limite de minions no mapa.
#[derive(Component)]
pub struct MinionSpawnData {
    pub timer: Timer,
    pub max_minions: u32,
    pub current_minions: u32,
}

/// Marcador de projétil de aviso do Sniper (linha de mira).
#[derive(Component)]
pub struct SniperWarning {
    pub lifetime: Timer,
}
