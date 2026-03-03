pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use systems::{asteroid_movement, asteroid_player_collision, handle_asteroid_deaths,
              spawn_asteroids, AsteroidSpawnTimer};

// ── Componentes ──────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, PartialEq)]
pub enum AsteroidSize {
    Large,
    Small,
}

#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
    pub velocity: Vec2,
    pub angular_velocity: f32,
}

// ── Plugin ───────────────────────────────────────────────────────────────────

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            4.5,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (
                spawn_asteroids,
                asteroid_movement,
                asteroid_player_collision,
                handle_asteroid_deaths,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
