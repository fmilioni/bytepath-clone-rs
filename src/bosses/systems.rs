use bevy::prelude::*;
use rand::Rng;

use crate::combat::components::{ColliderRadius, DamageEvent, DeathEvent, Health};
use crate::constants::{HALF_H, HALF_W, Z_BULLET, Z_VFX};
use crate::enemies::components::{Enemy, EnemyKind, EnemyShootTimer, EnemyStats};
use crate::player::components::Player;
use crate::vfx::components::{Particle, ScreenShake};
use crate::weapons::components::{Projectile, ProjectileOwner};

use super::components::{
    ActiveBoss, Boss, BossKind, BossPhase, BossPhaseTransition, DreadnoughtData,
    PhantomClone, PhantomData, SentinelData, SentinelShieldPanel,
    SingularityData, SwarmmotherData,
};
use super::spawner::{spawn_phantom_clone, spawn_shield_panels};

// ── Rastreia qual boss está vivo ──────────────────────────────────────────────

pub fn track_active_boss(
    mut active: ResMut<ActiveBoss>,
    boss_query: Query<Entity, With<Boss>>,
) {
    active.0 = boss_query.iter().next();
}

// ── Transição de fase ─────────────────────────────────────────────────────────

pub fn boss_phase_check(
    mut query: Query<(Entity, &Health, &mut BossPhase, &BossKind), With<Boss>>,
    mut events: EventWriter<BossPhaseTransition>,
) {
    for (entity, health, mut phase, &kind) in query.iter_mut() {
        let frac = health.current / health.max;
        let new_phase = if frac <= 0.33 { 3 } else if frac <= 0.66 { 2 } else { 1 };
        if new_phase > phase.phase {
            phase.phase = new_phase;
            events.send(BossPhaseTransition { boss_entity: entity, new_phase, boss_kind: kind });
        }
    }
}

pub fn apply_boss_phase(
    mut events: EventReader<BossPhaseTransition>,
    mut sentinel_q: Query<&mut SentinelData>,
    mut swarmmother_q: Query<&mut SwarmmotherData>,
    mut dreadnought_q: Query<&mut DreadnoughtData>,
    mut phantom_q: Query<&mut PhantomData>,
    mut singularity_q: Query<&mut SingularityData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shake: ResMut<ScreenShake>,
) {
    for ev in events.read() {
        shake.add_trauma(0.7);
        match ev.boss_kind {
            BossKind::Sentinel => {
                if let Ok(mut d) = sentinel_q.get_mut(ev.boss_entity) {
                    match ev.new_phase {
                        2 => {
                            d.rotate_speed = 1.8;
                            d.panel_count = 3;
                            d.attack_timer = Timer::from_seconds(2.0, TimerMode::Repeating);
                            spawn_shield_panels(&mut commands, &mut meshes, &mut materials, ev.boss_entity, 1);
                        }
                        3 => {
                            d.rotate_speed = 2.8;
                            d.panel_count = 4;
                            d.attack_timer = Timer::from_seconds(1.5, TimerMode::Repeating);
                            spawn_shield_panels(&mut commands, &mut meshes, &mut materials, ev.boss_entity, 1);
                        }
                        _ => {}
                    }
                }
            }
            BossKind::Swarmmother => {
                if let Ok(mut d) = swarmmother_q.get_mut(ev.boss_entity) {
                    match ev.new_phase {
                        2 => { d.minions_per_wave = 5; d.wave_timer = Timer::from_seconds(2.5, TimerMode::Repeating); }
                        3 => { d.minions_per_wave = 7; d.wave_timer = Timer::from_seconds(2.0, TimerMode::Repeating); }
                        _ => {}
                    }
                }
            }
            BossKind::Dreadnought => {
                if let Ok(mut d) = dreadnought_q.get_mut(ev.boss_entity) {
                    match ev.new_phase {
                        2 => { d.barrage_count = 7; d.barrage_timer = Timer::from_seconds(2.0, TimerMode::Repeating); }
                        3 => {
                            d.barrage_count = 9;
                            d.barrage_timer = Timer::from_seconds(1.5, TimerMode::Repeating);
                            d.nova_timer = Timer::from_seconds(3.5, TimerMode::Repeating);
                        }
                        _ => {}
                    }
                }
            }
            BossKind::Phantom => {
                if let Ok(mut d) = phantom_q.get_mut(ev.boss_entity) {
                    match ev.new_phase {
                        2 => { d.clone_count = 2; d.teleport_timer = Timer::from_seconds(2.0, TimerMode::Repeating); }
                        3 => { d.clone_count = 3; d.teleport_timer = Timer::from_seconds(1.2, TimerMode::Repeating); }
                        _ => {}
                    }
                }
            }
            BossKind::Singularity => {
                if let Ok(mut d) = singularity_q.get_mut(ev.boss_entity) {
                    match ev.new_phase {
                        2 => { d.pull_force = 220.0; d.nova_timer = Timer::from_seconds(3.0, TimerMode::Repeating); }
                        3 => {
                            d.pull_force = 380.0;
                            d.nova_timer = Timer::from_seconds(2.0, TimerMode::Repeating);
                            d.spawn_timer = Timer::from_seconds(4.0, TimerMode::Repeating);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

// ── Sentinel ──────────────────────────────────────────────────────────────────

pub fn sentinel_system(
    mut commands: Commands,
    time: Res<Time>,
    mut boss_q: Query<(&mut Transform, &mut SentinelData, &BossPhase, &EnemyStats), With<Boss>>,
    player_q: Query<&Transform, (With<Player>, Without<Boss>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(player_tf) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();

    for (mut tf, mut data, phase, stats) in boss_q.iter_mut() {
        let dt = time.delta_secs();
        data.shield_angle += data.rotate_speed * dt;
        data.attack_timer.tick(time.delta());

        // Deriva lentamente em direção ao player
        let to_player = (player_pos - tf.translation.truncate()).normalize_or_zero();
        tf.translation += (to_player * stats.speed * dt).extend(0.0);

        if data.attack_timer.just_finished() {
            let shot_count = match phase.phase { 1 => 4, 2 => 8, _ => 12 };
            fire_radial_burst(
                &mut commands, &mut meshes, &mut materials,
                tf.translation.truncate(), shot_count, 300.0, stats.damage,
            );
        }
    }
}

pub fn update_shield_panels(
    boss_q: Query<(&Transform, &SentinelData), (With<Boss>, Without<SentinelShieldPanel>)>,
    mut panel_q: Query<(&SentinelShieldPanel, &mut Transform), Without<Boss>>,
) {
    for (panel, mut panel_tf) in panel_q.iter_mut() {
        let Ok((boss_tf, data)) = boss_q.get(panel.boss_entity) else { continue; };
        let angle = data.shield_angle + panel.angle_offset;
        let offset = Vec2::new(angle.cos(), angle.sin()) * 60.0;
        panel_tf.translation = boss_tf.translation + offset.extend(0.0);
        panel_tf.rotation = Quat::from_rotation_z(angle);
    }
}

// ── Swarmmother ───────────────────────────────────────────────────────────────

pub fn swarmmother_system(
    mut commands: Commands,
    time: Res<Time>,
    mut boss_q: Query<(&Transform, &mut SwarmmotherData, &BossPhase, &EnemyStats), With<Boss>>,
    player_q: Query<&Transform, (With<Player>, Without<Boss>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_count: Query<(), With<Enemy>>,
) {
    let Ok(player_tf) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();
    let total = enemy_count.iter().count();

    for (tf, mut data, phase, stats) in boss_q.iter_mut() {
        data.wave_timer.tick(time.delta());
        data.shoot_timer.tick(time.delta());

        if data.wave_timer.just_finished() && total < 40 {
            let boss_pos = tf.translation.truncate();
            let count = data.minions_per_wave;
            let mut rng = rand::thread_rng();
            for i in 0..count {
                let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
                let offset = Vec2::new(angle.cos(), angle.sin()) * 50.0;
                spawn_swarmer_at(&mut commands, &mut meshes, &mut materials, boss_pos + offset);
            }
            if phase.phase >= 2 {
                let offset = Vec2::new(rng.gen_range(-70.0..70.0), rng.gen_range(-70.0..70.0));
                spawn_shooter_at(&mut commands, &mut meshes, &mut materials, boss_pos + offset);
            }
        }

        if phase.phase >= 2 && data.shoot_timer.just_finished() {
            let dir = (player_pos - tf.translation.truncate()).normalize_or_zero();
            if dir != Vec2::ZERO {
                fire_single(&mut commands, &mut meshes, &mut materials,
                    tf.translation.truncate(), dir, 260.0, stats.damage);
            }
        }
    }
}

// ── Dreadnought ───────────────────────────────────────────────────────────────

pub fn dreadnought_system(
    mut commands: Commands,
    time: Res<Time>,
    mut boss_q: Query<(&mut Transform, &mut DreadnoughtData, &BossPhase, &EnemyStats), With<Boss>>,
    player_q: Query<&Transform, (With<Player>, Without<Boss>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shake: ResMut<ScreenShake>,
) {
    let Ok(player_tf) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();

    for (mut tf, mut data, phase, stats) in boss_q.iter_mut() {
        let dt = time.delta_secs();
        data.barrage_timer.tick(time.delta());
        data.nova_timer.tick(time.delta());

        // Move lentamente
        let to_player = (player_pos - tf.translation.truncate()).normalize_or_zero();
        tf.translation += (to_player * stats.speed * dt).extend(0.0);

        if data.barrage_timer.just_finished() {
            let boss_pos = tf.translation.truncate();
            let base_dir = (player_pos - boss_pos).normalize_or_zero();
            if base_dir != Vec2::ZERO {
                let count = data.barrage_count;
                let spread = 0.8_f32 * std::f32::consts::PI / (count as f32 - 1.0).max(1.0);
                let base_angle = base_dir.y.atan2(base_dir.x);
                for i in 0..count {
                    let angle = base_angle - spread * (count as f32 - 1.0) / 2.0 + spread * i as f32;
                    fire_single(&mut commands, &mut meshes, &mut materials,
                        boss_pos, Vec2::new(angle.cos(), angle.sin()), 280.0, stats.damage);
                }
                shake.add_trauma(0.2);
            }
        }

        if phase.phase >= 3 && data.nova_timer.just_finished() {
            fire_radial_burst(&mut commands, &mut meshes, &mut materials,
                tf.translation.truncate(), 18, 320.0, stats.damage * 1.5);
            shake.add_trauma(0.5);
        }
    }
}

// ── Phantom ───────────────────────────────────────────────────────────────────

pub fn phantom_system(
    mut commands: Commands,
    time: Res<Time>,
    mut boss_q: Query<(&mut Transform, &mut PhantomData, &BossPhase, &EnemyStats), With<Boss>>,
    player_q: Query<&Transform, (With<Player>, Without<Boss>)>,
    clone_q: Query<Entity, With<PhantomClone>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(player_tf) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();

    for (mut tf, mut data, phase, stats) in boss_q.iter_mut() {
        data.teleport_timer.tick(time.delta());
        if !data.teleport_timer.just_finished() { continue; }

        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let dist = rng.gen_range(150.0..320.0_f32);
        let new_x = (player_pos.x + angle.cos() * dist).clamp(-HALF_W + 40.0, HALF_W - 40.0);
        let new_y = (player_pos.y + angle.sin() * dist).clamp(-HALF_H + 40.0, HALF_H - 40.0);
        let new_pos = Vec2::new(new_x, new_y);
        tf.translation.x = new_pos.x;
        tf.translation.y = new_pos.y;

        let existing = clone_q.iter().count() as u32;
        for _ in existing..data.clone_count {
            let ca = rng.gen_range(0.0..std::f32::consts::TAU);
            let cd = rng.gen_range(80.0..180.0_f32);
            let cx = (new_pos.x + ca.cos() * cd).clamp(-HALF_W + 40.0, HALF_W - 40.0);
            let cy = (new_pos.y + ca.sin() * cd).clamp(-HALF_H + 40.0, HALF_H - 40.0);
            spawn_phantom_clone(&mut commands, &mut meshes, &mut materials, Vec2::new(cx, cy));
        }

        if phase.phase >= 3 {
            let dir = (player_pos - new_pos).normalize_or_zero();
            if dir != Vec2::ZERO {
                fire_single(&mut commands, &mut meshes, &mut materials,
                    new_pos, dir, 300.0, stats.damage);
            }
        }
    }
}

// ── Singularity ───────────────────────────────────────────────────────────────

pub fn singularity_system(
    mut commands: Commands,
    time: Res<Time>,
    mut boss_q: Query<(&Transform, &mut SingularityData, &BossPhase, &EnemyStats), With<Boss>>,
    mut player_q: Query<(Entity, &mut Transform, &ColliderRadius), (With<Player>, Without<Boss>)>,
    mut damage_events: EventWriter<DamageEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shake: ResMut<ScreenShake>,
    enemy_count: Query<(), With<Enemy>>,
) {
    let Ok((player_entity, mut player_tf, player_col)) = player_q.get_single_mut() else { return; };
    let player_pos = player_tf.translation.truncate();

    for (boss_tf, mut data, phase, stats) in boss_q.iter_mut() {
        let dt = time.delta_secs();
        data.pulse_timer.tick(time.delta());
        data.nova_timer.tick(time.delta());
        data.spawn_timer.tick(time.delta());

        let boss_pos = boss_tf.translation.truncate();
        let to_boss = boss_pos - player_pos;
        let dist = to_boss.length();

        // Atração gravitacional constante
        if dist > 10.0 {
            player_tf.translation += (to_boss.normalize() * data.pull_force * dt).extend(0.0);
        }

        // Dano por contato com núcleo
        if dist < player_col.0 + 30.0 {
            damage_events.send(DamageEvent { target: player_entity, amount: stats.damage });
        }

        // Nova — empurra player + dano
        if data.nova_timer.just_finished() {
            let away = (player_pos - boss_pos).normalize_or_zero();
            player_tf.translation += (away * 400.0 * dt).extend(0.0);
            damage_events.send(DamageEvent { target: player_entity, amount: stats.damage * 2.0 });
            shake.add_trauma(0.6);

            let mut rng = rand::thread_rng();
            let mesh = meshes.add(Circle::new(4.0));
            for _ in 0..20 {
                let a = rng.gen_range(0.0..std::f32::consts::TAU);
                let s = rng.gen_range(150.0..350.0_f32);
                commands.spawn((
                    Particle { velocity: Vec2::new(a.cos(), a.sin()) * s,
                        lifetime: Timer::from_seconds(0.6, TimerMode::Once), initial_scale: 1.0, fade: true },
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(4.0, 0.0, 8.0)))),
                    Transform::from_translation(boss_pos.extend(Z_VFX)),
                ));
            }
        }

        // Fase 3: spawna inimigos
        if phase.phase >= 3 && data.spawn_timer.just_finished() {
            if enemy_count.iter().count() < 35 {
                let mut rng = rand::thread_rng();
                for _ in 0..3 {
                    let a = rng.gen_range(0.0..std::f32::consts::TAU);
                    let r = rng.gen_range(100.0..200.0_f32);
                    spawn_swarmer_at(&mut commands, &mut meshes, &mut materials,
                        boss_pos + Vec2::new(a.cos(), a.sin()) * r);
                }
            }
        }
    }
}

// ── Limpeza ao morte do boss ──────────────────────────────────────────────────

pub fn boss_death_cleanup(
    mut death_events: EventReader<DeathEvent>,
    mut commands: Commands,
    boss_query: Query<(), With<Boss>>,
    panel_query: Query<(Entity, &SentinelShieldPanel)>,
    clone_query: Query<Entity, With<PhantomClone>>,
    mut shake: ResMut<ScreenShake>,
) {
    for event in death_events.read() {
        if !boss_query.contains(event.entity) { continue; }
        shake.add_trauma(1.0);

        for (panel_entity, panel) in panel_query.iter() {
            if panel.boss_entity == event.entity {
                if let Some(mut cmd) = commands.get_entity(panel_entity) {
                    cmd.despawn();
                }
            }
        }
        for clone_entity in clone_query.iter() {
            if let Some(mut cmd) = commands.get_entity(clone_entity) {
                cmd.despawn();
            }
        }
    }
}

// ── Painéis de escudo absorvem balas do player ────────────────────────────────

pub fn shield_panel_absorb(
    mut commands: Commands,
    projectile_q: Query<(Entity, &Transform, &Projectile, &ColliderRadius)>,
    panel_q: Query<(&Transform, &ColliderRadius), With<SentinelShieldPanel>>,
) {
    for (proj_entity, proj_tf, projectile, proj_rad) in projectile_q.iter() {
        if projectile.owner != ProjectileOwner::Player { continue; }
        let proj_pos = proj_tf.translation.truncate();
        for (panel_tf, panel_rad) in panel_q.iter() {
            if proj_pos.distance(panel_tf.translation.truncate()) < proj_rad.0 + panel_rad.0 {
                if let Some(mut cmd) = commands.get_entity(proj_entity) {
                    cmd.despawn();
                }
                break;
            }
        }
    }
}

// ── Helpers de disparo ────────────────────────────────────────────────────────

fn fire_single(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2, direction: Vec2, speed: f32, damage: f32,
) {
    commands.spawn((
        Projectile { damage, speed, direction, owner: ProjectileOwner::Enemy,
            lifetime: Timer::from_seconds(3.0, TimerMode::Once) },
        ColliderRadius(3.5),
        Mesh2d(meshes.add(Circle::new(3.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(8.0, 2.0, 0.0)))),
        Transform::from_translation(pos.extend(Z_BULLET)),
    ));
}

fn fire_radial_burst(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2, count: u32, speed: f32, damage: f32,
) {
    let mesh = meshes.add(Circle::new(3.5));
    let mat = materials.add(ColorMaterial::from_color(Color::srgb(8.0, 4.0, 0.0)));
    for i in 0..count {
        let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
        commands.spawn((
            Projectile { damage, speed, direction: Vec2::new(angle.cos(), angle.sin()),
                owner: ProjectileOwner::Enemy, lifetime: Timer::from_seconds(3.0, TimerMode::Once) },
            ColliderRadius(3.5),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(mat.clone()),
            Transform::from_translation(pos.extend(Z_BULLET)),
        ));
    }
}

// ── Inimigos normais spawnadados pelos bosses ─────────────────────────────────

fn spawn_swarmer_at(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) {
    use crate::constants::Z_ENEMY;
    commands.spawn((
        Enemy, EnemyKind::Swarmer,
        Health::new(20.0), ColliderRadius(7.0),
        EnemyStats { speed: 160.0, damage: 8.0, fire_rate: 0.0, max_hp: 20.0, score: 10 },
        Mesh2d(meshes.add(RegularPolygon::new(7.0, 3))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(6.0, 1.0, 0.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}

fn spawn_shooter_at(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) {
    use crate::constants::Z_ENEMY;
    commands.spawn((
        Enemy, EnemyKind::Shooter,
        Health::new(50.0), ColliderRadius(10.0),
        EnemyStats { speed: 80.0, damage: 10.0, fire_rate: 0.5, max_hp: 50.0, score: 50 },
        EnemyShootTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
        Mesh2d(meshes.add(RegularPolygon::new(10.0, 5))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(6.0, 2.0, 0.0)))),
        Transform::from_translation(pos.extend(Z_ENEMY)),
    ));
}
