use bevy::prelude::*;

use crate::skill_tree::components::{PlayerSkills, SkillTreeUiState};
use crate::skill_tree::data::SkillCluster;

// ── Marcadores de UI ──────────────────────────────────────────────────────────

#[derive(Component)]
pub struct SkillTreeUiRoot;

#[derive(Component)]
pub struct SkillClusterTab(pub usize);

#[derive(Component)]
pub struct SkillNodeRow(pub usize); // 0..8 visíveis de cada vez

#[derive(Component)]
pub struct SkillDetailName;

#[derive(Component)]
pub struct SkillDetailInfo;

#[derive(Component)]
pub struct SkillSpCount;

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_skill_tree_ui(mut commands: Commands) {
    // Overlay escuro cobrindo a tela inteira
    commands
        .spawn((
            SkillTreeUiRoot,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.05, 0.92)),
            Visibility::Hidden,
        ))
        .with_children(|root| {
            // Painel central
            root.spawn(Node {
                width: Val::Px(760.0),
                height: Val::Px(520.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|panel| {
                // ── Cabeçalho ────────────────────────────────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("SKILL TREE  [TAB = close | A/D = cluster | W/S = node | ENTER = unlock]"),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.5, 0.6)),
                    ));
                    header.spawn((
                        SkillSpCount,
                        Text::new("SP: 0"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(8.0, 6.0, 0.0)),
                    ));
                });

                // ── Abas de cluster ───────────────────────────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.0),
                    ..default()
                })
                .with_children(|tabs| {
                    for (i, cluster) in SkillCluster::ALL.iter().enumerate() {
                        tabs.spawn((
                            SkillClusterTab(i),
                            Text::new(cluster.name()),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::srgb(0.4, 0.4, 0.4)),
                        ));
                    }
                });

                // Divisor
                panel.spawn((
                    Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ));

                // ── Lista de nós (9 linhas) ───────────────────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(3.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(230.0),
                    ..default()
                })
                .with_children(|list| {
                    for i in 0..9 {
                        list.spawn((
                            SkillNodeRow(i),
                            Text::new(""),
                            TextFont { font_size: 13.0, ..default() },
                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                        ));
                    }
                });

                // Divisor
                panel.spawn((
                    Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ));

                // ── Painel de detalhe ─────────────────────────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|detail| {
                    detail.spawn((
                        SkillDetailName,
                        Text::new(""),
                        TextFont { font_size: 15.0, ..default() },
                        TextColor(Color::srgb(8.0, 8.0, 4.0)),
                    ));
                    detail.spawn((
                        SkillDetailInfo,
                        Text::new(""),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });
            });
        });
}

// ── Atualização da UI ─────────────────────────────────────────────────────────

pub fn update_skill_tree_ui(
    ui_state: Res<SkillTreeUiState>,
    skills: Res<PlayerSkills>,
    mut root_q: Query<&mut Visibility, With<SkillTreeUiRoot>>,
    mut tab_q: Query<(&SkillClusterTab, &mut Text, &mut TextColor)>,
    mut row_q: Query<(&SkillNodeRow, &mut Text, &mut TextColor), Without<SkillClusterTab>>,
    mut detail_name_q: Query<&mut Text, (With<SkillDetailName>, Without<SkillNodeRow>, Without<SkillClusterTab>)>,
    mut detail_info_q: Query<&mut Text, (With<SkillDetailInfo>, Without<SkillDetailName>, Without<SkillNodeRow>, Without<SkillClusterTab>)>,
    mut sp_q: Query<&mut Text, (With<SkillSpCount>, Without<SkillDetailInfo>, Without<SkillDetailName>, Without<SkillNodeRow>, Without<SkillClusterTab>)>,
) {
    // Mostra/oculta overlay
    if let Ok(mut vis) = root_q.get_single_mut() {
        *vis = if ui_state.open { Visibility::Visible } else { Visibility::Hidden };
    }
    if !ui_state.open { return; }

    // SP counter
    if let Ok(mut txt) = sp_q.get_single_mut() {
        **txt = format!("SP: {}", skills.skill_points);
    }

    // Abas de cluster
    for (tab, mut text, mut color) in tab_q.iter_mut() {
        let cluster = SkillCluster::ALL[tab.0];
        let is_selected = tab.0 == ui_state.selected_cluster;
        if is_selected {
            *color = TextColor(Color::srgb(8.0, 6.0, 0.0));
            **text = format!("[{}]", cluster.name());
        } else {
            *color = TextColor(Color::srgb(0.4, 0.4, 0.4));
            **text = cluster.name().to_string();
        }
    }

    // Nós do cluster atual
    let nodes = crate::skill_tree::data::all_nodes();
    let cluster = SkillCluster::ALL[ui_state.selected_cluster];
    let cluster_nodes: Vec<_> = nodes.iter().filter(|n| n.cluster == cluster).collect();

    for (row, mut text, mut color) in row_q.iter_mut() {
        let abs_idx = ui_state.scroll_offset + row.0;
        if let Some(node) = cluster_nodes.get(abs_idx) {
            let is_selected = abs_idx == ui_state.selected_node_idx;
            let is_unlocked = skills.unlocked.contains(&node.id);
            let prereq_ok = node.prereq.map_or(true, |p| skills.unlocked.contains(&p));
            let can_afford = skills.skill_points >= node.cost;

            let cursor = if is_selected { ">" } else { " " };
            let status = if is_unlocked {
                "[X]"
            } else if prereq_ok {
                "[ ]"
            } else {
                "[-]"
            };
            let prereq_label = if let Some(prereq_id) = node.prereq {
                if !skills.unlocked.contains(&prereq_id) {
                    if let Some(p) = nodes.iter().find(|n| n.id == prereq_id) {
                        format!(" (req: {})", p.name)
                    } else { String::new() }
                } else { String::new() }
            } else { String::new() };

            **text = format!(
                "{} {} {:<24} {:<20}  {}sp{}",
                cursor, status, node.name, node.desc, node.cost, prereq_label
            );

            *color = TextColor(if is_unlocked {
                Color::srgb(0.0, 6.0, 1.0)
            } else if !prereq_ok {
                Color::srgb(0.3, 0.3, 0.3)
            } else if !can_afford {
                Color::srgb(0.7, 0.5, 0.1)
            } else if is_selected {
                Color::srgb(8.0, 8.0, 4.0)
            } else {
                Color::srgb(0.8, 0.8, 0.8)
            });
        } else {
            **text = String::new();
        }
    }

    // Painel de detalhes do nó selecionado
    if let Some(node) = cluster_nodes.get(ui_state.selected_node_idx) {
        let is_unlocked = skills.unlocked.contains(&node.id);
        let prereq_ok   = node.prereq.map_or(true, |p| skills.unlocked.contains(&p));
        let can_afford  = skills.skill_points >= node.cost;

        if let Ok(mut txt) = detail_name_q.get_single_mut() {
            **txt = node.name.to_string();
        }
        if let Ok(mut txt) = detail_info_q.get_single_mut() {
            let action = if is_unlocked {
                "Already unlocked".to_string()
            } else if !prereq_ok {
                let pid = node.prereq.unwrap();
                let pname = nodes.iter().find(|n| n.id == pid).map_or("?", |n| n.name);
                format!("Requires: {}", pname)
            } else if !can_afford {
                format!("Not enough SP (need {}, have {})", node.cost, skills.skill_points)
            } else {
                format!("Press ENTER to unlock for {} SP", node.cost)
            };
            **txt = format!("{}  |  {}", node.desc, action);
        }
    } else {
        if let Ok(mut txt) = detail_name_q.get_single_mut() { **txt = String::new(); }
        if let Ok(mut txt) = detail_info_q.get_single_mut() { **txt = String::new(); }
    }
}
