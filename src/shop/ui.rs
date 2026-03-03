use bevy::prelude::*;

use super::components::{Credits, PlayerInventory, ShopUiState};

// ── Marcadores de UI ──────────────────────────────────────────────────────────

#[derive(Component)]
pub struct ShopUiRoot;

#[derive(Component)]
pub struct ShopCreditsCount;

#[derive(Component)]
pub struct ShopItemRow(pub usize); // 0..3 — itens da oferta

#[derive(Component)]
pub struct ShopDetailName;

#[derive(Component)]
pub struct ShopDetailInfo;

#[derive(Component)]
pub struct ShopInventoryLine;

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_shop_ui(mut commands: Commands) {
    commands
        .spawn((
            ShopUiRoot,
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
            // Painel central — mesmas dimensões do skill tree
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
                        Text::new("LOJA  [E = fechar | A/D = selecionar | ENTER = comprar | R = reroll (30cr)]"),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.5, 0.6)),
                    ));
                    header.spawn((
                        ShopCreditsCount,
                        Text::new("CR: 0"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(8.0, 6.0, 0.0)),
                    ));
                });

                // Divisor
                panel.spawn((
                    Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ));

                // ── Lista de itens da oferta (4 linhas) ──────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(210.0),
                    ..default()
                })
                .with_children(|list| {
                    for i in 0..4 {
                        list.spawn((
                            ShopItemRow(i),
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

                // ── Inventário ────────────────────────────────────────────────
                panel.spawn((
                    ShopInventoryLine,
                    Text::new(""),
                    TextFont { font_size: 13.0, ..default() },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));

                // Divisor
                panel.spawn((
                    Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ));

                // ── Painel de detalhe do item selecionado ─────────────────────
                panel.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|detail| {
                    detail.spawn((
                        ShopDetailName,
                        Text::new(""),
                        TextFont { font_size: 15.0, ..default() },
                        TextColor(Color::srgb(8.0, 8.0, 4.0)),
                    ));
                    detail.spawn((
                        ShopDetailInfo,
                        Text::new(""),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });
            });
        });
}

// ── Atualização ───────────────────────────────────────────────────────────────

pub fn update_shop_ui(
    shop_state: Res<ShopUiState>,
    credits: Res<Credits>,
    inventory: Res<PlayerInventory>,
    mut root_q: Query<&mut Visibility, With<ShopUiRoot>>,
    mut cr_q: Query<&mut Text, (With<ShopCreditsCount>, Without<ShopItemRow>, Without<ShopDetailName>, Without<ShopDetailInfo>, Without<ShopInventoryLine>)>,
    mut row_q: Query<(&ShopItemRow, &mut Text, &mut TextColor), Without<ShopCreditsCount>>,
    mut inv_q: Query<&mut Text, (With<ShopInventoryLine>, Without<ShopCreditsCount>, Without<ShopItemRow>, Without<ShopDetailName>, Without<ShopDetailInfo>)>,
    mut det_name_q: Query<&mut Text, (With<ShopDetailName>, Without<ShopCreditsCount>, Without<ShopItemRow>, Without<ShopInventoryLine>, Without<ShopDetailInfo>)>,
    mut det_info_q: Query<&mut Text, (With<ShopDetailInfo>, Without<ShopCreditsCount>, Without<ShopItemRow>, Without<ShopInventoryLine>, Without<ShopDetailName>)>,
) {
    let Ok(mut vis) = root_q.get_single_mut() else { return; };
    *vis = if shop_state.open { Visibility::Visible } else { Visibility::Hidden };
    if !shop_state.open { return; }

    // Créditos
    if let Ok(mut txt) = cr_q.get_single_mut() {
        **txt = format!("CR: {}", credits.0);
    }

    // Linhas de itens
    for (row, mut text, mut color) in row_q.iter_mut() {
        let i = row.0;
        if let Some(&item_id) = shop_state.offered.get(i) {
            let def = item_id.def();
            let is_selected = i == shop_state.selected;
            let can_afford = credits.0 >= def.cost;

            let cursor = if is_selected { ">" } else { " " };
            let status = if can_afford { "[ ]" } else { "[!]" };

            **text = format!(
                "{} {} {:<28} {:<24}  {}cr",
                cursor, status, def.name, def.description, def.cost
            );

            *color = TextColor(if is_selected && can_afford {
                Color::srgb(8.0, 8.0, 4.0)      // amarelo brilhante — selecionado
            } else if is_selected {
                Color::srgb(0.7, 0.5, 0.1)       // laranja — selecionado mas sem créditos
            } else if can_afford {
                Color::srgb(0.8, 0.8, 0.8)       // branco — disponível
            } else {
                Color::srgb(0.4, 0.3, 0.1)       // marrom escuro — sem créditos
            });
        } else {
            **text = String::new();
        }
    }

    // Inventário
    if let Ok(mut txt) = inv_q.get_single_mut() {
        if inventory.items.is_empty() {
            **txt = "INVENTARIO: vazio".to_string();
        } else {
            let names: Vec<&str> = inventory.items.iter().map(|&id| id.def().name).collect();
            **txt = format!("INVENTARIO ({}/12): {}", inventory.items.len(), names.join("  |  "));
        }
    }

    // Detalhe do item selecionado
    if let Some(&item_id) = shop_state.offered.get(shop_state.selected) {
        let def = item_id.def();
        let can_afford = credits.0 >= def.cost;

        if let Ok(mut txt) = det_name_q.get_single_mut() {
            **txt = def.name.to_string();
        }
        if let Ok(mut txt) = det_info_q.get_single_mut() {
            let action = if can_afford {
                format!("Pressione ENTER para comprar por {}cr", def.cost)
            } else {
                format!("Creditos insuficientes (precisa {}cr, tem {}cr)", def.cost, credits.0)
            };
            **txt = format!("{}  |  {}", def.description, action);
        }
    } else {
        if let Ok(mut txt) = det_name_q.get_single_mut() { **txt = String::new(); }
        if let Ok(mut txt) = det_info_q.get_single_mut() {
            **txt = if shop_state.offered.is_empty() {
                "Todos os itens foram comprados.".to_string()
            } else {
                String::new()
            };
        }
    }
}
