use bevy::prelude::*;

mod bosses;
mod campaign;
mod combat;
mod constants;
mod enemies;
mod obstacles;
mod pickups;
mod player;
mod shop;
mod skill_tree;
mod states;
mod ui;
mod vfx;
mod weapons;

use states::GameState;

fn close_on_esc(
    mut app_exit: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keys.just_pressed(KeyCode::Escape) { return; }

    match state.get() {
        GameState::Playing        => { next_state.set(GameState::ScenarioSelect); }
        GameState::ScenarioSelect => { next_state.set(GameState::ShipSelect); }
        GameState::ShipSelect     => { next_state.set(GameState::MainMenu); }
        GameState::GameOver       => { next_state.set(GameState::MainMenu); }
        GameState::MainMenu       => { let _ = app_exit.send(AppExit::Success); }
        // Paused: fechado por skill tree (Tab) ou loja (E)
        _ => {}
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bytepath-RS".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(ClearColor(constants::COLOR_BACKGROUND))
        .add_plugins(vfx::VfxPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(weapons::WeaponsPlugin)
        .add_plugins(enemies::EnemiesPlugin)
        .add_plugins(obstacles::ObstaclesPlugin)
        .add_plugins(bosses::BossPlugin)
        .add_plugins(skill_tree::SkillTreePlugin)
        .add_plugins(pickups::PickupsPlugin)
        .add_plugins(shop::ShopPlugin)
        .add_plugins(campaign::CampaignPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(ui::UiPlugin)
        // Começa no menu principal
        .add_systems(Startup, |mut next: ResMut<NextState<GameState>>| {
            next.set(GameState::MainMenu);
        })
        .add_systems(Update, close_on_esc)
        .run();
}
