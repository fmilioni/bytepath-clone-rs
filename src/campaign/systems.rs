use bevy::prelude::*;

use crate::bosses::components::{ActiveBoss, Boss};
use crate::bosses::spawner::spawn_boss;
use crate::combat::components::DeathEvent;
use crate::enemies::components::Enemy;
use crate::enemies::spawner::EnemySpawnTimer;
use crate::pickups::Pickup;
use crate::vfx::components::{Particle, TrailSegment};
use crate::weapons::components::Projectile;
use crate::states::GameState;

use super::components::{
    ActiveScenario, CampaignProgress, ScenarioKillCount, ScenarioWinTimer,
};

const SAVE_PATH: &str = "save.json";

// ── Persistência ──────────────────────────────────────────────────────────────

pub fn load_campaign_progress(mut progress: ResMut<CampaignProgress>) {
    if let Ok(data) = std::fs::read_to_string(SAVE_PATH) {
        if let Ok(loaded) = serde_json::from_str::<CampaignProgress>(&data) {
            *progress = loaded;
            info!("Progresso carregado: {} cenários completados", progress.completed.len());
        }
    }
}

fn save_campaign_progress(progress: &CampaignProgress) {
    match serde_json::to_string_pretty(progress) {
        Ok(data) => {
            if let Err(e) = std::fs::write(SAVE_PATH, data) {
                warn!("Falha ao salvar progresso: {}", e);
            } else {
                info!("Progresso salvo.");
            }
        }
        Err(e) => warn!("Falha ao serializar progresso: {}", e),
    }
}

// ── Configuração do cenário ───────────────────────────────────────────────────

/// Inicializa o ScenarioKillCount e o timer de spawn ao entrar em Playing.
pub fn setup_scenario(
    active: Res<ActiveScenario>,
    mut kill_count: ResMut<ScenarioKillCount>,
    mut win_timer: ResMut<ScenarioWinTimer>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if active.id == 0 { return; }

    let def = active.def();
    *kill_count = ScenarioKillCount {
        kills: 0,
        goal: def.kill_goal,
        boss: def.boss,
        boss_killed: false,
        boss_spawned: false,
    };
    win_timer.0 = None;

    // Ajusta intervalo de spawn ao do cenário
    spawn_timer.0 = Timer::from_seconds(def.spawn_interval_secs, TimerMode::Repeating);

    info!("Cenário {} iniciado: mata {} inimigos", def.id, def.kill_goal);
}

// ── Tracking de kills ─────────────────────────────────────────────────────────

pub fn track_scenario_kills(
    mut death_events: EventReader<DeathEvent>,
    mut kill_count: ResMut<ScenarioKillCount>,
    active: Res<ActiveScenario>,
    boss_q: Query<(), With<Boss>>,
) {
    if active.id == 0 { return; }

    for event in death_events.read() {
        if !event.was_enemy { continue; }

        if boss_q.contains(event.entity) {
            kill_count.boss_killed = true;
            info!("Boss derrotado! Cenário completo.");
        } else {
            kill_count.kills += 1;
        }
    }
}

/// Spawna o boss automaticamente quando kill_goal é atingido (em cenários com boss).
pub fn auto_spawn_boss(
    mut commands: Commands,
    active: Res<ActiveScenario>,
    mut kill_count: ResMut<ScenarioKillCount>,
    active_boss: Res<ActiveBoss>,
    boss_q: Query<(), With<Boss>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if active.id == 0 { return; }
    let def = active.def();
    let Some(boss_kind) = def.boss else { return; };

    // Spawna boss uma vez quando os kills normais forem suficientes
    if !kill_count.boss_spawned
        && kill_count.kills >= kill_count.goal
        && active_boss.0.is_none()
        && boss_q.is_empty()
    {
        spawn_boss(&mut commands, &mut meshes, &mut materials, boss_kind);
        kill_count.boss_spawned = true;
        info!("Boss spawnou!");
    }
}

// ── Condição de vitória ───────────────────────────────────────────────────────

pub fn check_scenario_win(
    kill_count: Res<ScenarioKillCount>,
    active: Res<ActiveScenario>,
    mut progress: ResMut<CampaignProgress>,
    mut win_timer: ResMut<ScenarioWinTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if active.id == 0 { return; }

    // Inicia timer de vitória quando cenário é completado
    if win_timer.0.is_none() && kill_count.is_complete() {
        if progress.completed.insert(active.id) {
            save_campaign_progress(&progress);
        }
        win_timer.0 = Some(Timer::from_seconds(3.5, TimerMode::Once));
        info!("Cenário {} completado!", active.id);
    }

    // Aguarda timer e volta ao ScenarioSelect
    if let Some(ref mut timer) = win_timer.0 {
        timer.tick(time.delta());
        if timer.finished() {
            win_timer.0 = None;
            next_state.set(GameState::ScenarioSelect);
        }
    }
}

// ── Limpeza ao sair de Playing ────────────────────────────────────────────────

pub fn cleanup_gameplay(
    mut commands: Commands,
    enemy_q: Query<Entity, With<Enemy>>,
    pickup_q: Query<Entity, With<Pickup>>,
    projectile_q: Query<Entity, With<Projectile>>,
    particle_q: Query<Entity, With<Particle>>,
    trail_q: Query<Entity, With<TrailSegment>>,
) {
    // Player é despawnado pelo PlayerPlugin (OnExit Playing → despawn_player)
    for e in enemy_q.iter()
        .chain(pickup_q.iter())
        .chain(projectile_q.iter())
        .chain(particle_q.iter())
        .chain(trail_q.iter())
    {
        if let Some(cmd) = commands.get_entity(e) {
            cmd.despawn_recursive();
        }
    }
}
