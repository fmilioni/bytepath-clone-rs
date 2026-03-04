use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::bosses::components::BossKind;

// ── Regiões ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Frontier,
    Nebula,
    AsteroidBelt,
    Void,
    Core,
}

impl Region {
    pub const fn name(self) -> &'static str {
        match self {
            Region::Frontier     => "FRONTIER",
            Region::Nebula       => "NEBULA",
            Region::AsteroidBelt => "ASTEROID BELT",
            Region::Void         => "VOID",
            Region::Core         => "CORE",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Region::Frontier     => Color::srgb(0.0, 6.0, 2.0),
            Region::Nebula       => Color::srgb(4.0, 0.0, 8.0),
            Region::AsteroidBelt => Color::srgb(6.0, 3.0, 0.0),
            Region::Void         => Color::srgb(0.0, 2.0, 6.0),
            Region::Core         => Color::srgb(8.0, 0.5, 0.0),
        }
    }

    /// Cor de tint das estrelas quando jogando nesta região.
    pub fn star_tint(self) -> Color {
        match self {
            Region::Frontier     => Color::srgb(0.9, 2.2, 1.2),
            Region::Nebula       => Color::srgb(2.5, 0.5, 3.5),
            Region::AsteroidBelt => Color::srgb(2.8, 1.6, 0.4),
            Region::Void         => Color::srgb(0.4, 0.8, 2.8),
            Region::Core         => Color::srgb(3.0, 0.4, 0.2),
        }
    }

    /// ClearColor de fundo ao jogar nesta região.
    pub fn bg_color(self) -> Color {
        match self {
            Region::Frontier     => Color::srgb(0.01, 0.03, 0.01),
            Region::Nebula       => Color::srgb(0.03, 0.01, 0.06),
            Region::AsteroidBelt => Color::srgb(0.04, 0.02, 0.01),
            Region::Void         => Color::srgb(0.0,  0.0,  0.02),
            Region::Core         => Color::srgb(0.06, 0.01, 0.0),
        }
    }
}

// ── Definição de cenário ──────────────────────────────────────────────────────

pub struct ScenarioDef {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub region: Region,
    /// Kills de inimigos normais necessários antes do boss (ou para completar, se sem boss).
    pub kill_goal: u32,
    /// Boss opcional — spawna quando kill_goal for atingido.
    pub boss: Option<BossKind>,
    /// Pesos por tipo de inimigo:
    /// [Swarmer, Charger, Shooter, Circler, Bomber, Sniper, Shield, Splitter, Teleporter, MinionSpawner]
    pub enemy_weights: [u32; 10],
    pub enemy_hp_scale: f32,
    pub enemy_speed_scale: f32,
    pub spawn_interval_secs: f32,
    pub max_enemies: u32,
}

// ── Resources de campanha ─────────────────────────────────────────────────────

/// Cenário atualmente ativo (definido antes de entrar em Playing).
#[derive(Resource, Clone)]
pub struct ActiveScenario {
    pub id: u32,
}

impl ActiveScenario {
    pub fn def(&self) -> &'static ScenarioDef {
        super::data::scenario_def(self.id)
    }
}

/// Contagem de kills dentro do cenário ativo.
#[derive(Resource, Default)]
pub struct ScenarioKillCount {
    pub scenario_id: u32, // 0 = não inicializado
    pub kills: u32,
    pub goal: u32,
    pub boss: Option<BossKind>,
    pub boss_killed: bool,
    pub boss_spawned: bool,
}

impl ScenarioKillCount {
    pub fn is_complete(&self) -> bool {
        self.kills >= self.goal && (self.boss.is_none() || self.boss_killed)
    }
}

/// Progresso persistido da campanha (salvo em disco).
#[derive(Resource, Serialize, Deserialize, Default, Clone)]
pub struct CampaignProgress {
    pub completed: HashSet<u32>,
}

impl CampaignProgress {
    pub fn is_unlocked(&self, id: u32) -> bool {
        if id == 1 { return true; }
        self.completed.contains(&(id - 1))
    }

    pub fn is_completed(&self, id: u32) -> bool {
        self.completed.contains(&id)
    }
}

/// Cenário selecionado na tela de seleção.
#[derive(Resource, Default)]
pub struct SelectedScenario {
    pub id: u32,           // 1-based; 0 = nenhum
    pub scroll_offset: usize,
}

/// Timer de transição pós-vitória (delay antes de voltar ao ScenarioSelect).
#[derive(Resource, Default)]
pub struct ScenarioWinTimer(pub Option<Timer>);
