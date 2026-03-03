pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;

use systems::{attract_pickups, collect_pickups, pickup_bobbing, pickup_lifetime, spawn_pickup_on_death};

// ── Componentes ───────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PickupKind {
    HealthPack,
    ShieldPack,
    EnergyPack,
    SpecialAmmoPack,
    Scrap,
    Fuel,
    MysteryCrate,
}

#[derive(Component)]
pub struct Pickup;

#[derive(Component)]
pub struct PickupLifetime(pub Timer);

#[derive(Component)]
pub struct PickupBobbing {
    pub base_y: f32,
    pub time: f32,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_pickup_on_death,
                attract_pickups,
                collect_pickups,
                pickup_bobbing,
                pickup_lifetime,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
