use bevy::prelude::*;
use crate::states::GameState;

// ── Marcadores ────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct MainMenuTitle;

#[derive(Component)]
pub struct MainMenuOption(pub usize); // 0=Jogar 1=Sair

// ── Resource ──────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct MainMenuState {
    pub selected: usize,
}

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(18.0),
                ..default()
            },
        ))
        .with_children(|root| {
            // Título principal
            root.spawn((
                MainMenuTitle,
                Text::new("BYTEPATH-RS"),
                TextFont { font_size: 76.0, ..default() },
                TextColor(Color::srgb(0.0, 6.0, 8.0)),
            ));

            // Subtítulo
            root.spawn((
                Text::new("◆  space shooter neon  ◆"),
                TextFont { font_size: 17.0, ..default() },
                TextColor(Color::srgb(0.25, 0.25, 0.35)),
            ));

            // Spacer
            root.spawn(Node { height: Val::Px(28.0), ..default() });

            // Opções
            for (i, label) in ["JOGAR", "SAIR"].iter().enumerate() {
                root.spawn((
                    MainMenuOption(i),
                    Text::new(format!("  {}  ", label)),
                    TextFont { font_size: 26.0, ..default() },
                    TextColor(Color::srgb(0.35, 0.35, 0.45)),
                ));
            }

            // Spacer
            root.spawn(Node { height: Val::Px(30.0), ..default() });

            // Instruções
            root.spawn((
                Text::new("↑↓  —  selecionar     ENTER  —  confirmar"),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.28, 0.28, 0.35)),
            ));

            // Versão
            root.spawn((
                Text::new("v0.1  ·  TAB=skills  E=loja  B=boss"),
                TextFont { font_size: 11.0, ..default() },
                TextColor(Color::srgb(0.2, 0.2, 0.25)),
            ));
        });
}

pub fn despawn_main_menu(
    mut commands: Commands,
    root_q: Query<Entity, With<MainMenuRoot>>,
) {
    for e in root_q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// ── Input ─────────────────────────────────────────────────────────────────────

pub fn main_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<MainMenuState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    let up   = keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp);
    let down = keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown);

    if up   && state.selected > 0 { state.selected -= 1; }
    if down && state.selected < 1 { state.selected += 1; }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        match state.selected {
            0 => next_state.set(GameState::ShipSelect),
            _ => { let _ = app_exit.send(AppExit::Success); }
        }
    }
}

// ── Animação ──────────────────────────────────────────────────────────────────

pub fn update_main_menu(
    time: Res<Time>,
    state: Res<MainMenuState>,
    mut title_q: Query<&mut TextColor, (With<MainMenuTitle>, Without<MainMenuOption>)>,
    mut option_q: Query<(&MainMenuOption, &mut Text, &mut TextColor), Without<MainMenuTitle>>,
) {
    // Título pulsa entre ciano e azul
    let t = time.elapsed_secs();
    let pulse = (t * 1.2).sin() * 0.5 + 0.5;
    if let Ok(mut color) = title_q.get_single_mut() {
        *color = TextColor(Color::srgb(
            pulse * 2.0,
            (1.0 - pulse * 0.4) * 6.0,
            8.0,
        ));
    }

    // Destaca opção selecionada
    const LABELS: [&str; 2] = ["JOGAR", "SAIR"];
    for (opt, mut text, mut color) in option_q.iter_mut() {
        if opt.0 == state.selected {
            **text = format!("▶  {}  ◀", LABELS[opt.0]);
            *color = TextColor(Color::srgb(8.0, 8.0, 4.0));
        } else {
            **text = format!("   {}   ", LABELS[opt.0]);
            *color = TextColor(Color::srgb(0.35, 0.35, 0.45));
        }
    }
}
