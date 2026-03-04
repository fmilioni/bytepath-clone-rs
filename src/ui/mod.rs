pub mod boss_hud;
pub mod hud;
pub mod main_menu;
pub mod scenario_select;
pub mod ship_select;
pub mod skill_tree_ui;

use bevy::prelude::*;
use crate::campaign::components::{ActiveScenario, CampaignProgress, ScenarioKillCount};
use crate::shop::components::{Credits, PlayerInventory};
use crate::skill_tree::components::PlayerSkills;
use crate::states::GameState;

// ── Fonte com suporte a acentos ───────────────────────────────────────────────
#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

fn load_game_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameFont(asset_server.load("fonts/font.ttf")));
}

/// Aplica a fonte correta em todo TextFont recém-criado (Added<TextFont>).
/// Não precisa modificar nenhum spawn function — funciona automaticamente.
fn apply_font_to_new_text(
    game_font: Option<Res<GameFont>>,
    mut q: Query<&mut TextFont, Added<TextFont>>,
) {
    let Some(gf) = game_font else { return; };
    for mut tf in q.iter_mut() {
        tf.font = gf.0.clone();
    }
}

use boss_hud::{spawn_boss_hud, update_boss_hud, BossHudRoot};
use hud::{spawn_hud, update_hud, HudRoot};
use main_menu::{
    despawn_main_menu, main_menu_input, spawn_main_menu, update_main_menu, MainMenuState,
};
use scenario_select::{
    despawn_scenario_select, init_star_tint, scenario_select_input, spawn_scenario_select,
    tint_stars_scenario_select, update_scenario_map, update_win_overlay,
};
use ship_select::{
    despawn_ship_select, rotate_preview, ship_select_input, spawn_ship_select,
    update_ship_select_ui,
};
use skill_tree_ui::{spawn_skill_tree_ui, update_skill_tree_ui, SkillTreeUiRoot};
use crate::shop::ui::{spawn_shop_ui, ShopUiRoot};
use crate::shop::components::ShopUiState;
use crate::skill_tree::components::SkillTreeUiState;

// ── Pause Menu ────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
struct PauseMenuState {
    selected: usize, // 0=Continuar 1=Sair
}

#[derive(Component)]
struct PauseOverlayRoot;

#[derive(Component)]
struct PauseMenuOption(usize);

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        PauseOverlayRoot,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(18.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
        Visibility::Hidden,
    ))
    .with_children(|p| {
        p.spawn((
            Text::new("PAUSADO"),
            TextFont { font_size: 56.0, ..default() },
            TextColor(Color::srgb(8.0, 8.0, 4.0)),
        ));

        p.spawn(Node { height: Val::Px(16.0), ..default() });

        for (i, label) in ["CONTINUAR", "SAIR"].iter().enumerate() {
            p.spawn((
                PauseMenuOption(i),
                Text::new(format!("   {}   ", label)),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::srgb(0.35, 0.35, 0.45)),
            ));
        }

        p.spawn(Node { height: Val::Px(16.0), ..default() });

        p.spawn((
            Text::new("↑↓  —  selecionar     ENTER  —  confirmar"),
            TextFont { font_size: 13.0, ..default() },
            TextColor(Color::srgb(0.28, 0.28, 0.35)),
        ));
    });
}

fn despawn_pause_overlay(
    mut commands: Commands,
    q: Query<Entity, With<PauseOverlayRoot>>,
) {
    for e in q.iter() { commands.entity(e).despawn_recursive(); }
}

fn update_pause_overlay(
    shop_state: Res<ShopUiState>,
    skill_ui: Res<SkillTreeUiState>,
    pause_menu: Res<PauseMenuState>,
    mut root_q: Query<&mut Visibility, With<PauseOverlayRoot>>,
    mut option_q: Query<(&PauseMenuOption, &mut Text, &mut TextColor)>,
) {
    let Ok(mut vis) = root_q.get_single_mut() else { return; };
    *vis = if !shop_state.open && !skill_ui.open {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    const LABELS: [&str; 2] = ["CONTINUAR", "SAIR"];
    for (opt, mut text, mut color) in option_q.iter_mut() {
        if opt.0 == pause_menu.selected {
            **text = format!("▶  {}  ◀", LABELS[opt.0]);
            *color = TextColor(Color::srgb(8.0, 8.0, 4.0));
        } else {
            **text = format!("   {}   ", LABELS[opt.0]);
            *color = TextColor(Color::srgb(0.35, 0.35, 0.45));
        }
    }
}

fn pause_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    shop_state: Res<ShopUiState>,
    skill_ui: Res<SkillTreeUiState>,
    mut pause_menu: ResMut<PauseMenuState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if shop_state.open || skill_ui.open { return; }

    let up   = keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp);
    let down = keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown);

    if up   && pause_menu.selected > 0 { pause_menu.selected -= 1; }
    if down && pause_menu.selected < 1 { pause_menu.selected += 1; }

    if keys.just_pressed(KeyCode::Enter) {
        match pause_menu.selected {
            0 => next_state.set(GameState::Playing),
            _ => next_state.set(GameState::ScenarioSelect),
        }
        pause_menu.selected = 0;
    }
}

// ── Game Over overlay ─────────────────────────────────────────────────────────

#[derive(Component)]
struct GameOverRoot;

fn spawn_game_over(
    mut commands: Commands,
    kill_count: Res<ScenarioKillCount>,
    active: Res<ActiveScenario>,
    credits: Res<Credits>,
    skills: Res<PlayerSkills>,
    inventory: Res<PlayerInventory>,
    progress: Res<CampaignProgress>,
) {
    let scenario_name = if active.id != 0 {
        active.def().name
    } else {
        "—"
    };

    let items_count = inventory.items.len();

    commands
        .spawn((
            GameOverRoot,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.0, 0.0, 0.88)),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("GAME OVER"),
                TextFont { font_size: 56.0, ..default() },
                TextColor(Color::srgb(8.0, 0.5, 0.0)),
            ));

            root.spawn((
                Text::new(format!("Cenário: {}  ({} kills)", scenario_name, kill_count.kills)),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.6, 0.4, 0.3)),
            ));

            root.spawn((
                Text::new(format!(
                    "Créditos: {}   Skill pts: {}   Itens: {}   Cenários completos: {}/25",
                    credits.0, skills.skill_points, items_count, progress.completed.len()
                )),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
            ));

            root.spawn(Node { height: Val::Px(10.0), ..default() });

            root.spawn((
                Text::new("ENTER — tentar novamente   |   ESC — menu principal"),
                TextFont { font_size: 15.0, ..default() },
                TextColor(Color::srgb(0.45, 0.4, 0.4)),
            ));
        });
}

fn despawn_game_over(
    mut commands: Commands,
    q: Query<Entity, With<GameOverRoot>>,
) {
    for e in q.iter() { commands.entity(e).despawn_recursive(); }
}

fn game_over_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::ScenarioSelect);
    }
}

// ── Campaign Complete overlay ─────────────────────────────────────────────────

#[derive(Component)]
struct CampaignCompleteRoot;

fn check_campaign_complete(
    progress: Res<CampaignProgress>,
    existing: Query<(), With<CampaignCompleteRoot>>,
    mut commands: Commands,
) {
    if progress.completed.len() < 25 { return; }
    if !existing.is_empty() { return; }

    commands
        .spawn((
            CampaignCompleteRoot,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(16.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("★  CAMPANHA COMPLETA — TODOS OS 25 CENÁRIOS CONQUISTADOS  ★"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgb(8.0, 7.0, 0.0)),
            ));
        });
}

fn despawn_campaign_complete(
    mut commands: Commands,
    q: Query<Entity, With<CampaignCompleteRoot>>,
) {
    for e in q.iter() { commands.entity(e).despawn_recursive(); }
}

// ── Despawn de UI de gameplay ─────────────────────────────────────────────────

fn despawn_playing_ui(
    mut commands: Commands,
    hud_q: Query<Entity, With<HudRoot>>,
    boss_hud_q: Query<Entity, With<BossHudRoot>>,
    skill_tree_q: Query<Entity, With<SkillTreeUiRoot>>,
    shop_q: Query<Entity, With<ShopUiRoot>>,
) {
    for e in hud_q.iter()
        .chain(boss_hud_q.iter())
        .chain(skill_tree_q.iter())
        .chain(shop_q.iter())
    {
        commands.entity(e).despawn_recursive();
    }
}

/// Limpa skill tree e shop ao sair de Paused (Playing vai re-spawnar tudo via OnEnter).
fn despawn_paused_ui(
    mut commands: Commands,
    skill_tree_q: Query<Entity, With<SkillTreeUiRoot>>,
    shop_q: Query<Entity, With<ShopUiRoot>>,
) {
    for e in skill_tree_q.iter().chain(shop_q.iter()) {
        commands.entity(e).despawn_recursive();
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MainMenuState>()
            .init_resource::<PauseMenuState>()
            .add_systems(Startup, load_game_font)
            .add_systems(Update, apply_font_to_new_text)
            // ── Main Menu ─────────────────────────────────────────
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                (main_menu_input, update_main_menu)
                    .run_if(in_state(GameState::MainMenu)),
            )
            // ── Ship Select ──────────────────────────────────────
            .add_systems(OnEnter(GameState::ShipSelect), spawn_ship_select)
            .add_systems(OnExit(GameState::ShipSelect), despawn_ship_select)
            .add_systems(
                Update,
                (ship_select_input, update_ship_select_ui, rotate_preview)
                    .run_if(in_state(GameState::ShipSelect)),
            )
            // ── Scenario Select ───────────────────────────────────
            .add_systems(OnEnter(GameState::ScenarioSelect), (spawn_scenario_select, init_star_tint))
            .add_systems(OnExit(GameState::ScenarioSelect), despawn_scenario_select)
            .add_systems(
                Update,
                (scenario_select_input, update_scenario_map, tint_stars_scenario_select, check_campaign_complete)
                    .run_if(in_state(GameState::ScenarioSelect)),
            )
            .add_systems(OnExit(GameState::ScenarioSelect), despawn_campaign_complete)
            // ── HUD in-game ───────────────────────────────────────
            .add_systems(
                OnEnter(GameState::Playing),
                (spawn_hud, spawn_boss_hud, spawn_skill_tree_ui, spawn_shop_ui),
            )
            .add_systems(OnExit(GameState::Playing), despawn_playing_ui)
            // Ao entrar em Paused, re-spawna os overlays que OnExit(Playing) destruiu
            .add_systems(
                OnEnter(GameState::Paused),
                (spawn_skill_tree_ui, spawn_shop_ui, spawn_pause_overlay),
            )
            .add_systems(OnExit(GameState::Paused), (despawn_paused_ui, despawn_pause_overlay))
            .add_systems(
                Update,
                (update_pause_overlay, pause_menu_input).run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                Update,
                (update_hud, update_boss_hud, update_win_overlay)
                    .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            )
            // Skill tree + shop UI rodam em Playing E Paused
            .add_systems(
                Update,
                update_skill_tree_ui
                    .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            )
            // ── Game Over ─────────────────────────────────────────
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over)
            .add_systems(
                Update,
                game_over_input.run_if(in_state(GameState::GameOver)),
            );
    }
}
