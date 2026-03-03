use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    ShipSelect,   // Tela de seleção de nave
    ScenarioSelect,
    Playing,
    Paused,
    SkillTree,
    Shop,
    GameOver,
    BossTransition,
}
