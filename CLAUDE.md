# Bytepath-RS — CLAUDE.md

## Visão Geral
Space shooter estilo Bytepath com gráficos geométricos neon e bloom HDR.
- **Engine**: Bevy 0.15.3
- **Build**: `cargo build` | `cargo run`
- **Save**: `save.json` (raiz do projeto)
- **Fonte**: `assets/fonts/font.ttf` (SFNSMono, suporte a caracteres acentuados)

## Fluxo de Estados
```
MainMenu → ShipSelect → ScenarioSelect → Playing ↔ Paused
Playing → (morte) → GameOver → MainMenu
Playing → (vitória) → ScenarioSelect
ESC: Playing→ScenarioSelect, ScenarioSelect→ShipSelect, ShipSelect→MainMenu, MainMenu→sair
Paused: Tab fecha skill tree, E fecha loja (ambos voltam a Playing)
```

**IMPORTANTE sobre Paused:**
- `OnExit(Playing)` dispara ao ir para Paused — NÃO coloque cleanup de gameplay aqui
- Cleanup de entidades fica em `OnEnter(ScenarioSelect)` e `OnEnter(GameOver)`
- `spawn_player` tem guard `if !existing.is_empty() { return; }` para não spawnar duplo
- `setup_scenario` tem guard `if kill_count.scenario_id == active.id { return; }` idem
- `Time<Virtual>` é pausado em `OnEnter(Paused)` e despausado em `OnExit(Paused)`

## Estrutura de Arquivos
```
src/
├── main.rs              — App setup, close_on_esc, começa em MainMenu
├── states.rs            — GameState enum
├── constants.rs         — Cores neon HDR, Z-layers, tamanhos
├── campaign/
│   ├── components.rs    — Region, ScenarioDef, ActiveScenario, ScenarioKillCount (scenario_id!), CampaignProgress, SelectedScenario
│   ├── data.rs          — 25 cenários via OnceLock<Vec<ScenarioDef>>
│   ├── systems.rs       — setup_scenario, track_kills, auto_spawn_boss, check_win, cleanup_gameplay, load_save, autosave
│   └── mod.rs           — CampaignPlugin
├── player/
│   ├── spawn.rs         — spawn_player (com guard), despawn_player, spawn_camera
│   ├── systems.rs       — player_movement
│   ├── abilities.rs     — stealth_cloak, brawler_melee
│   ├── ship_classes.rs  — 8 classes (SelectedShipClass)
│   ├── stat_calc.rs     — compute_full_stats (base + skills + items)
│   └── components.rs    — Player, ShipStats, PlayerThrottle
├── enemies/
│   ├── spawner.rs       — spawn_enemies com ActiveScenario (pesos/dificuldade)
│   ├── systems.rs       — movimento, IA, colisão
│   └── components.rs    — EnemyKind, EnemyStats, etc.
├── bosses/
│   ├── spawner.rs       — spawn_boss por BossKind
│   ├── systems.rs       — 5 bosses com 3 fases
│   └── components.rs    — Boss, BossKind, ActiveBoss
├── skill_tree/
│   ├── components.rs    — PlayerSkills (Resource), SkillTreeUiState, SkillNodeUnlocked
│   ├── data.rs          — 101 nós em 6 clusters (custos: 8/15/25/40/60 SP)
│   └── systems.rs       — toggle_skill_tree, skill_tree_input, award_skill_points (1/kill, 20/boss)
├── shop/
│   ├── components.rs    — ItemId (Serialize!), Credits, PlayerInventory, ShopUiState
│   ├── systems.rs       — open_close_shop, shop_buy, award_credits (3/kill, 40/boss)
│   └── ui.rs            — spawn_shop_ui, update_shop_ui
├── ui/
│   ├── mod.rs           — UiPlugin, GameFont resource, apply_font_to_new_text, despawn_playing_ui, despawn_paused_ui
│   ├── hud.rs           — HudRoot, HudHp/Shield/Energy, HudScenarioBar, HudEconomy
│   ├── boss_hud.rs      — BossHudRoot
│   ├── skill_tree_ui.rs — SkillTreeUiRoot, spawn/update_skill_tree_ui
│   ├── main_menu.rs     — MainMenuState, spawn/update/despawn_main_menu
│   ├── ship_select.rs   — spawn/update_ship_select_ui
│   ├── scenario_select.rs — ScenarioSelectRoot, ScenarioCountHint/TitleHint/DescHint
│   └── (game over, campaign complete em mod.rs)
├── combat/
│   ├── components.rs    — Health, Shield, Energy, ColliderRadius, DeathEvent
│   └── systems.rs       — collision_detection, apply_damage, handle_player_death
├── weapons/
│   ├── components.rs    — Projectile, WeaponCooldown, SpecialAmmo
│   └── systems.rs       — move_projectiles, player_shoot
├── pickups/
│   ├── mod.rs           — 7 tipos de pickup
│   └── systems.rs       — attract_pickups, collect_pickups
├── vfx/
│   ├── mod.rs           — VfxPlugin, spawn_stars (Startup), drift_stars_menu
│   ├── systems.rs       — update_particles, apply_screen_shake, drift_stars_menu
│   └── components.rs    — Particle, TrailSegment, Star, GameCamera
└── obstacles/           — asteroides destrutíveis Large→Small
```

## Bevy 0.15 — Gotchas Críticos

### Queries
- `query.single()` / `single_mut()` retorna valor **diretamente** (não Result)
- `query.get_single()` / `get_single_mut()` retorna `Result` ✓
- Para despawn seguro: `if let Some(cmd) = commands.get_entity(e) { cmd.despawn_recursive(); }`

### Events
- `EventWriter::send()` retorna `EventId<T>` — em match arms usar `let _ = writer.send(...)`
- `.write()` NÃO existe em Bevy 0.15 (é 0.16+)

### States
- `bevy_state` feature é **obrigatória** para `States`, `OnEnter`, `in_state`, `NextState`
- `NextState` é buffer — aplicado no **próximo frame** via `PreUpdate → StateTransition`
- `OnExit(A)` e `OnEnter(B)` rodam no **mesmo frame** (ambos em StateTransition)

### UI / Texto
- `TextFont { font_size: X, ..default() }` usa fonte padrão (sem acentos)
- `apply_font_to_new_text` (UiPlugin) aplica `GameFont` automaticamente em todo `Added<TextFont>`
- Para queries de múltiplos `Text` na mesma entidade: usar `Without<OtherMarker>` para disjoint

### Tempo
- `ResMut<Time<Virtual>>` → `.pause()` / `.unpause()` para congelar física e sistemas

### Tuplas em spawn
- `.spawn((Marker, Node { ... }))` precisa de **duas camadas** de parênteses

## Economia e Progressão
| Recurso | Valor |
|---------|-------|
| SP por kill regular | 1 |
| SP por boss | 20 |
| Créditos por kill | 3 |
| Créditos por boss | 40 |
| Custo nó tier 1 | 8 SP |
| Custo nó tier 2 | 15 SP |
| Custo nó tier 3 | 25 SP |
| Custo nó tier 4 | 40 SP |
| Custo nó tier 5 | 60 SP |
| Total para maxar skills | ~2400 SP |
| Máximo disponível (25 cenários) | ~1800 SP |

## Cenários e Regiões
- 25 cenários, 5 regiões: Frontier(1-5), Nebula(6-11), AsteroidBelt(12-17), Void(18-22), Core(23-25)
- Boss cenários: 5(Sentinel), 11(Swarmmother), 17(Dreadnought), 22(Phantom), 25(Singularity)
- `ActiveScenario.id` é definido antes de entrar em Playing
- `ScenarioKillCount.scenario_id == 0` = não inicializado; setup_scenario inicializa e reseta em cleanup

## Sistema de Save (`save.json`)
Salva automaticamente quando `Credits`, `PlayerSkills`, `PlayerInventory` ou `CampaignProgress` mudam.
```json
{
  "completed": [1, 2, 3],
  "credits": 150,
  "skill_points": 45,
  "unlocked_skills": [0, 1, 11],
  "inventory": ["NanoArmor", "PickupMagnet"]
}
```
Para resetar o save: deletar `save.json`.

## Padrões de UI
- Toda UI de gameplay (HUD, BossHUD, SkillTree, Shop) é despawnada em `despawn_playing_ui` (OnExit Playing)
- `OnEnter(Paused)` re-spawna SkillTree + Shop (os outros ficam ocultos pelo overlay escuro)
- `OnExit(Paused)` despawna SkillTree + Shop antes do `OnEnter(Playing)` re-spawnar tudo
- Markers distintos para queries: `ScenarioCountHint`, `ScenarioTitleHint`, `ScenarioDescHint`

## Controles
| Tecla | Ação |
|-------|------|
| A/D | Girar nave |
| Espaço | Atirar |
| Q | Habilidade especial |
| Tab | Abrir/fechar skill tree |
| E | Abrir/fechar loja |
| B | Spawnar boss aleatório (debug) |
| ESC | Voltar ao estado anterior |
