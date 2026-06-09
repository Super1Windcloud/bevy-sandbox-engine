use rand::Rng;

use bevy_sandbox_engine::{EditorPlugin, RuntimePlugin, bevy::prelude::*};

const ARENA_HALF_SIZE: f32 = 540.0;
const PLAYER_SPEED: f32 = 360.0;
const BULLET_SPEED: f32 = 720.0;
const ENEMY_SPEED: f32 = 140.0;
const PLAYER_RADIUS: f32 = 18.0;
const ENEMY_RADIUS: f32 = 16.0;
const BULLET_RADIUS: f32 = 6.0;
const PLAYER_FIRE_INTERVAL: f32 = 0.18;
const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
const PLAYER_MAX_HEALTH: i32 = 5;

fn main() {
    let editor_mode = !std::env::args().any(|arg| arg == "-game");

    let mut app = App::new();
    app.add_plugins(RuntimePlugin)
        .insert_resource(GameSession::default())
        .add_plugins(ShooterTemplatePlugin);

    if editor_mode {
        app.add_plugins(EditorPlugin);
    }

    app.run();
}

struct ShooterTemplatePlugin;

impl Plugin for ShooterTemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                restart_input,
                player_movement,
                player_shooting,
                bullet_motion,
                enemy_spawner,
                enemy_motion,
                bullet_enemy_collisions,
                enemy_player_collisions,
                cleanup_outside_arena,
                update_hud,
            ),
        );
    }
}

#[derive(Resource)]
struct GameSession {
    score: u32,
    health: i32,
    fire_cooldown: Timer,
    enemy_spawn_timer: Timer,
    game_over: bool,
}

impl Default for GameSession {
    fn default() -> Self {
        Self {
            score: 0,
            health: PLAYER_MAX_HEALTH,
            fire_cooldown: Timer::from_seconds(PLAYER_FIRE_INTERVAL, TimerMode::Repeating),
            enemy_spawn_timer: Timer::from_seconds(ENEMY_SPAWN_INTERVAL, TimerMode::Repeating),
            game_over: false,
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct ArenaEntity;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands) {
    spawn_arena(&mut commands);
    spawn_player(&mut commands);
    spawn_hud(&mut commands);
    commands.spawn(Camera2d);
}

fn spawn_arena(commands: &mut Commands) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.08, 0.10, 0.12),
            Vec2::splat(ARENA_HALF_SIZE * 2.0 + 80.0),
        ),
        Transform::from_xyz(0.0, 0.0, -10.0),
        ArenaEntity,
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.12, 0.15, 0.18),
            Vec2::splat(ARENA_HALF_SIZE * 2.0),
        ),
        ArenaEntity,
    ));
}

fn spawn_player(commands: &mut Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.34, 0.82, 0.76), Vec2::new(28.0, 28.0)),
        Transform::from_xyz(0.0, 0.0, 5.0),
        Player,
        ArenaEntity,
    ));
}

fn spawn_hud(commands: &mut Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(16.0),
            top: Val::Px(16.0),
            ..default()
        },
        Hud,
    ));
}

fn restart_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    cleanup_query: Query<Entity, With<ArenaEntity>>,
) {
    if !session.game_over || !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    for entity in &cleanup_query {
        commands.entity(entity).despawn();
    }

    *session = GameSession::default();
    spawn_arena(&mut commands);
    spawn_player(&mut commands);
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut player_query: Single<&mut Transform, With<Player>>,
) {
    if session.game_over {
        return;
    }

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let direction = direction.normalize();
    player_query.translation += (direction * PLAYER_SPEED * time.delta_secs()).extend(0.0);
    player_query.translation.x = player_query
        .translation
        .x
        .clamp(-ARENA_HALF_SIZE, ARENA_HALF_SIZE);
    player_query.translation.y = player_query
        .translation
        .y
        .clamp(-ARENA_HALF_SIZE, ARENA_HALF_SIZE);
}

fn player_shooting(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    player_query: Single<&Transform, With<Player>>,
) {
    if session.game_over {
        return;
    }

    session.fire_cooldown.tick(time.delta());

    let mut shot_direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        shot_direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        shot_direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        shot_direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        shot_direction.x += 1.0;
    }

    if shot_direction == Vec2::ZERO || !session.fire_cooldown.is_finished() {
        return;
    }

    session.fire_cooldown.reset();
    let shot_direction = shot_direction.normalize();

    commands.spawn((
        Sprite::from_color(
            Color::srgb(1.0, 0.84, 0.36),
            Vec2::splat(BULLET_RADIUS * 2.0),
        ),
        Transform::from_translation(
            player_query.translation + (shot_direction * (PLAYER_RADIUS + 12.0)).extend(1.0),
        ),
        Velocity(shot_direction * BULLET_SPEED),
        Bullet,
        ArenaEntity,
    ));
}

fn bullet_motion(
    time: Res<Time>,
    mut bullet_query: Query<(&Velocity, &mut Transform), With<Bullet>>,
) {
    for (velocity, mut transform) in &mut bullet_query {
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}

fn enemy_spawner(time: Res<Time>, mut commands: Commands, mut session: ResMut<GameSession>) {
    if session.game_over {
        return;
    }

    if !session.enemy_spawn_timer.tick(time.delta()).just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let edge = rng.gen_range(0..4);
    let offset = rng.gen_range(-ARENA_HALF_SIZE..ARENA_HALF_SIZE);
    let spawn = match edge {
        0 => Vec2::new(-ARENA_HALF_SIZE, offset),
        1 => Vec2::new(ARENA_HALF_SIZE, offset),
        2 => Vec2::new(offset, -ARENA_HALF_SIZE),
        _ => Vec2::new(offset, ARENA_HALF_SIZE),
    };

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.93, 0.33, 0.32),
            Vec2::splat(ENEMY_RADIUS * 2.0),
        ),
        Transform::from_translation(spawn.extend(3.0)),
        Enemy,
        ArenaEntity,
    ));
}

fn enemy_motion(
    time: Res<Time>,
    session: Res<GameSession>,
    player_query: Single<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if session.game_over {
        return;
    }

    let player_position = player_query.translation.truncate();

    for mut enemy_transform in &mut enemy_query {
        let to_player = player_position - enemy_transform.translation.truncate();
        let direction = to_player.normalize_or_zero();
        enemy_transform.translation += (direction * ENEMY_SPEED * time.delta_secs()).extend(0.0);
    }
}

fn bullet_enemy_collisions(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_position = bullet_transform.translation.truncate();

        for (enemy_entity, enemy_transform) in &enemy_query {
            let enemy_position = enemy_transform.translation.truncate();
            let hit_distance = BULLET_RADIUS + ENEMY_RADIUS;

            if bullet_position.distance(enemy_position) <= hit_distance {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                session.score += 1;
                break;
            }
        }
    }
}

fn enemy_player_collisions(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    player_query: Single<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    if session.game_over {
        return;
    }

    let player_position = player_query.translation.truncate();

    for (enemy_entity, enemy_transform) in &enemy_query {
        let enemy_position = enemy_transform.translation.truncate();
        let hit_distance = PLAYER_RADIUS + ENEMY_RADIUS;

        if player_position.distance(enemy_position) <= hit_distance {
            commands.entity(enemy_entity).despawn();
            session.health -= 1;

            if session.health <= 0 {
                session.game_over = true;
                session.health = 0;
            }
        }
    }
}

fn cleanup_outside_arena(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
) {
    let cleanup_limit = ARENA_HALF_SIZE + 120.0;

    for (entity, transform) in &bullets {
        if transform.translation.x.abs() > cleanup_limit
            || transform.translation.y.abs() > cleanup_limit
        {
            commands.entity(entity).despawn();
        }
    }
}

fn update_hud(session: Res<GameSession>, mut hud: Single<&mut Text, With<Hud>>) {
    if session.game_over {
        **hud = format!(
            "Score: {}   HP: {}   WASD move   Arrows shoot   Press R to restart",
            session.score, session.health
        );
    } else {
        **hud = format!(
            "Score: {}   HP: {}   WASD move   Arrows shoot",
            session.score, session.health
        );
    }
}
