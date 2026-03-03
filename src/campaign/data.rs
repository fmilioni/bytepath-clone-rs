use std::sync::OnceLock;

use crate::bosses::components::BossKind;
use super::components::{Region, ScenarioDef};

static SCENARIOS: OnceLock<Vec<ScenarioDef>> = OnceLock::new();

pub fn scenario_def(id: u32) -> &'static ScenarioDef {
    let all = SCENARIOS.get_or_init(build_scenarios);
    all.iter().find(|s| s.id == id).expect("Scenario ID inválido")
}

pub fn all_scenarios() -> &'static [ScenarioDef] {
    SCENARIOS.get_or_init(build_scenarios)
}

fn build_scenarios() -> Vec<ScenarioDef> {
    vec![
        // ── FRONTIER (1-5) ────────────────────────────────────────────────────
        // Tutorial: swarmers e chargers, baixa dificuldade
        ScenarioDef {
            id: 1, name: "Patrol Zone",
            description: "Zona de patrulha da fronteira. Ameaças básicas.",
            region: Region::Frontier,
            kill_goal: 15, boss: None,
            enemy_weights: [8, 4, 0, 0, 0, 0, 0, 0, 0, 0],
            enemy_hp_scale: 0.8, enemy_speed_scale: 0.8,
            spawn_interval_secs: 3.2, max_enemies: 10,
        },
        ScenarioDef {
            id: 2, name: "Hostile Entry",
            description: "Invasores surgem em maior número. Cuidado com os atiradores.",
            region: Region::Frontier,
            kill_goal: 20, boss: None,
            enemy_weights: [6, 4, 2, 0, 0, 0, 0, 0, 0, 0],
            enemy_hp_scale: 0.9, enemy_speed_scale: 0.85,
            spawn_interval_secs: 2.8, max_enemies: 12,
        },
        ScenarioDef {
            id: 3, name: "Border Skirmish",
            description: "Escaramuça na fronteira. Novos tipos de inimigos aparecem.",
            region: Region::Frontier,
            kill_goal: 25, boss: None,
            enemy_weights: [5, 4, 3, 2, 0, 0, 0, 0, 0, 0],
            enemy_hp_scale: 0.95, enemy_speed_scale: 0.9,
            spawn_interval_secs: 2.5, max_enemies: 14,
        },
        ScenarioDef {
            id: 4, name: "Frontier Defense",
            description: "Defenda a fronteira contra uma frota variada.",
            region: Region::Frontier,
            kill_goal: 30, boss: None,
            enemy_weights: [4, 3, 3, 2, 1, 1, 0, 0, 0, 0],
            enemy_hp_scale: 1.0, enemy_speed_scale: 1.0,
            spawn_interval_secs: 2.2, max_enemies: 16,
        },
        ScenarioDef {
            id: 5, name: "BOSS: Sentinel",
            description: "O guardião da fronteira. Destrua seus painéis de escudo primeiro.",
            region: Region::Frontier,
            kill_goal: 15, boss: Some(BossKind::Sentinel),
            enemy_weights: [4, 2, 2, 1, 0, 0, 0, 0, 0, 0],
            enemy_hp_scale: 1.0, enemy_speed_scale: 1.0,
            spawn_interval_secs: 3.5, max_enemies: 8,
        },

        // ── NEBULA (6-11) ─────────────────────────────────────────────────────
        // Inimigos com escudo, atiradores circulantes, snipers aparecem
        ScenarioDef {
            id: 6, name: "Nebula Entry",
            description: "A nebulosa interfere nos sistemas. Inimigos protegidos aparecem.",
            region: Region::Nebula,
            kill_goal: 30, boss: None,
            enemy_weights: [5, 3, 3, 3, 0, 0, 1, 0, 0, 0],
            enemy_hp_scale: 1.05, enemy_speed_scale: 1.0,
            spawn_interval_secs: 2.2, max_enemies: 16,
        },
        ScenarioDef {
            id: 7, name: "Ion Storm",
            description: "Tempestade iônica. Snipers à distância e escudos direcionais.",
            region: Region::Nebula,
            kill_goal: 35, boss: None,
            enemy_weights: [4, 3, 3, 3, 0, 1, 2, 0, 0, 0],
            enemy_hp_scale: 1.1, enemy_speed_scale: 1.0,
            spawn_interval_secs: 2.0, max_enemies: 18,
        },
        ScenarioDef {
            id: 8, name: "Nebula Patrol",
            description: "Patrulha profunda na nebulosa. Bombers à vista.",
            region: Region::Nebula,
            kill_goal: 40, boss: None,
            enemy_weights: [4, 3, 3, 3, 1, 1, 2, 1, 0, 0],
            enemy_hp_scale: 1.15, enemy_speed_scale: 1.05,
            spawn_interval_secs: 2.0, max_enemies: 18,
        },
        ScenarioDef {
            id: 9, name: "Frequency Zone",
            description: "Zona de alta frequência. Splitters se multiplicam.",
            region: Region::Nebula,
            kill_goal: 45, boss: None,
            enemy_weights: [3, 3, 3, 3, 2, 2, 2, 2, 0, 0],
            enemy_hp_scale: 1.2, enemy_speed_scale: 1.05,
            spawn_interval_secs: 1.8, max_enemies: 20,
        },
        ScenarioDef {
            id: 10, name: "Nebula Deep",
            description: "Fundo da nebulosa. Teleportadores surgem pela primeira vez.",
            region: Region::Nebula,
            kill_goal: 50, boss: None,
            enemy_weights: [3, 3, 3, 3, 2, 2, 2, 2, 1, 0],
            enemy_hp_scale: 1.25, enemy_speed_scale: 1.1,
            spawn_interval_secs: 1.8, max_enemies: 20,
        },
        ScenarioDef {
            id: 11, name: "BOSS: Swarmmother",
            description: "Mãe dos enxames. Ondas infinitas de minions em cada fase.",
            region: Region::Nebula,
            kill_goal: 20, boss: Some(BossKind::Swarmmother),
            enemy_weights: [4, 2, 2, 2, 0, 0, 2, 1, 0, 0],
            enemy_hp_scale: 1.2, enemy_speed_scale: 1.0,
            spawn_interval_secs: 3.5, max_enemies: 8,
        },

        // ── ASTEROID BELT (12-17) ─────────────────────────────────────────────
        // Bombers, Splitters e MinionSpawners em alta densidade
        ScenarioDef {
            id: 12, name: "Belt Entry",
            description: "Cinturão de asteroides. Bombers explodem em área.",
            region: Region::AsteroidBelt,
            kill_goal: 35, boss: None,
            enemy_weights: [3, 3, 3, 2, 3, 1, 2, 2, 0, 0],
            enemy_hp_scale: 1.2, enemy_speed_scale: 1.05,
            spawn_interval_secs: 2.0, max_enemies: 18,
        },
        ScenarioDef {
            id: 13, name: "Rock Field",
            description: "Campo de detritos denso. Splitters se dividem em dois.",
            region: Region::AsteroidBelt,
            kill_goal: 40, boss: None,
            enemy_weights: [3, 3, 3, 2, 3, 2, 2, 3, 0, 0],
            enemy_hp_scale: 1.3, enemy_speed_scale: 1.05,
            spawn_interval_secs: 1.8, max_enemies: 20,
        },
        ScenarioDef {
            id: 14, name: "Debris Storm",
            description: "Tempestade de detritos. Inimigos mais rápidos e resistentes.",
            region: Region::AsteroidBelt,
            kill_goal: 45, boss: None,
            enemy_weights: [2, 3, 3, 2, 4, 2, 3, 3, 1, 0],
            enemy_hp_scale: 1.35, enemy_speed_scale: 1.1,
            spawn_interval_secs: 1.8, max_enemies: 20,
        },
        ScenarioDef {
            id: 15, name: "Mining Colony",
            description: "Colônia mineira abandonada. Geradores de minions ocupam o espaço.",
            region: Region::AsteroidBelt,
            kill_goal: 50, boss: None,
            enemy_weights: [2, 3, 3, 2, 4, 3, 3, 3, 1, 1],
            enemy_hp_scale: 1.4, enemy_speed_scale: 1.1,
            spawn_interval_secs: 1.6, max_enemies: 22,
        },
        ScenarioDef {
            id: 16, name: "Belt Ambush",
            description: "Emboscada no cinturão. Maior concentração de ameaças.",
            region: Region::AsteroidBelt,
            kill_goal: 55, boss: None,
            enemy_weights: [2, 3, 3, 2, 4, 3, 3, 4, 2, 1],
            enemy_hp_scale: 1.5, enemy_speed_scale: 1.15,
            spawn_interval_secs: 1.6, max_enemies: 22,
        },
        ScenarioDef {
            id: 17, name: "BOSS: Dreadnought",
            description: "Couraçado de guerra. Barragens em leque e nova explosiva na fase 3.",
            region: Region::AsteroidBelt,
            kill_goal: 25, boss: Some(BossKind::Dreadnought),
            enemy_weights: [2, 2, 3, 2, 3, 2, 2, 2, 1, 0],
            enemy_hp_scale: 1.5, enemy_speed_scale: 1.0,
            spawn_interval_secs: 3.0, max_enemies: 10,
        },

        // ── VOID (18-22) ──────────────────────────────────────────────────────
        // Teleportadores, Snipers, MinionSpawners em alta densidade
        ScenarioDef {
            id: 18, name: "Void Entry",
            description: "Entrada do Void. Teleportadores aparecem em grande número.",
            region: Region::Void,
            kill_goal: 50, boss: None,
            enemy_weights: [2, 2, 3, 2, 2, 3, 2, 2, 3, 1],
            enemy_hp_scale: 1.5, enemy_speed_scale: 1.15,
            spawn_interval_secs: 1.8, max_enemies: 20,
        },
        ScenarioDef {
            id: 19, name: "Phantom Zone",
            description: "Zona de fantasmas. Inimigos imprevisíveis em padrões caóticos.",
            region: Region::Void,
            kill_goal: 55, boss: None,
            enemy_weights: [2, 2, 3, 2, 2, 3, 2, 2, 4, 2],
            enemy_hp_scale: 1.6, enemy_speed_scale: 1.2,
            spawn_interval_secs: 1.6, max_enemies: 22,
        },
        ScenarioDef {
            id: 20, name: "Dark Matter",
            description: "Matéria escura distorce o espaço. Snipers letais.",
            region: Region::Void,
            kill_goal: 60, boss: None,
            enemy_weights: [2, 2, 3, 2, 2, 4, 2, 2, 4, 2],
            enemy_hp_scale: 1.7, enemy_speed_scale: 1.2,
            spawn_interval_secs: 1.6, max_enemies: 22,
        },
        ScenarioDef {
            id: 21, name: "Void Core",
            description: "Centro do Void. Tudo ao mesmo tempo. Prepare-se para o boss.",
            region: Region::Void,
            kill_goal: 65, boss: None,
            enemy_weights: [2, 2, 3, 2, 2, 4, 2, 3, 4, 3],
            enemy_hp_scale: 1.85, enemy_speed_scale: 1.25,
            spawn_interval_secs: 1.5, max_enemies: 24,
        },
        ScenarioDef {
            id: 22, name: "BOSS: Phantom",
            description: "O Fantasma. Teleportes constantes e clones decoy enganam a visão.",
            region: Region::Void,
            kill_goal: 30, boss: Some(BossKind::Phantom),
            enemy_weights: [1, 2, 3, 2, 1, 3, 2, 2, 5, 1],
            enemy_hp_scale: 1.8, enemy_speed_scale: 1.0,
            spawn_interval_secs: 3.0, max_enemies: 8,
        },

        // ── CORE (23-25) ──────────────────────────────────────────────────────
        // Máxima dificuldade, todos os tipos, densidade máxima
        ScenarioDef {
            id: 23, name: "Core Breach",
            description: "Invasão ao núcleo. Todos os tipos de inimigos em alta densidade.",
            region: Region::Core,
            kill_goal: 70, boss: None,
            enemy_weights: [3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
            enemy_hp_scale: 2.0, enemy_speed_scale: 1.3,
            spawn_interval_secs: 1.5, max_enemies: 22,
        },
        ScenarioDef {
            id: 24, name: "Heart of Darkness",
            description: "O coração das trevas. Sobreviva ao caos total.",
            region: Region::Core,
            kill_goal: 80, boss: None,
            enemy_weights: [3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
            enemy_hp_scale: 2.2, enemy_speed_scale: 1.35,
            spawn_interval_secs: 1.3, max_enemies: 24,
        },
        ScenarioDef {
            id: 25, name: "BOSS: Singularity",
            description: "O colapso final. Singularidade gravitacional devora tudo.",
            region: Region::Core,
            kill_goal: 40, boss: Some(BossKind::Singularity),
            enemy_weights: [3, 3, 3, 3, 2, 3, 2, 2, 3, 2],
            enemy_hp_scale: 2.5, enemy_speed_scale: 1.4,
            spawn_interval_secs: 2.5, max_enemies: 12,
        },
    ]
}
