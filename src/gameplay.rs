use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use crate::common::AppState;
use crate::common::BackgroundImage;
use crate::common::{GridPosition, ScreenPosition};
use crate::common::{GRID_SIZE, GRID_WIDTH, GRID_HEIGHT};
use crate::common::AnimationTimer;
use crate::common::DeathTimer;
use crate::snake::{SnakeHead, SnakeBodyPiece};
use crate::wall::Wall;
use crate::food::Food;
use crate::bomb::{Bomb, ParticleSystem, Particle};

pub struct GameplayPlugin;

impl GameplayPlugin {
    const SNAKE_HEAD_Z_DEPTH: f32 = 100.0;
    const SNAKE_BODY_Z_DEPTH: f32 = 99.0;
    const FOOD_Z_DEPTH: f32 = 50.0;
    const BOMB_Z_DEPTH: f32 = 51.0;
    const WALL_Z_DEPTH: f32 = 200.0;
    const BACKGROUND_Z_DEPTH: f32 = 0.0;
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        let particle_system = ParticleSystem::new();
        app
            .add_systems(OnEnter(AppState::Gameplay),
                (spawn_background_system, spawn_walls_system, spawn_snake_system))
            .add_systems(Update, (control_snake_system, sprite_animation_system)
                .run_if(in_state(AppState::Gameplay)))
            .add_systems(Update, wall_collision_system
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))))
            .add_systems(Update, grow_snake_system
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))))
            .add_systems(Update, move_snake_system
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))))
            .add_systems(Update, food_collision_system.after(move_snake_system)
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))))
            .add_systems(Update, bomb_collision_system.after(move_snake_system)
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))))
            .add_systems(Update, snake_body_collision_system.after(move_snake_system)
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))))
            .add_systems(Update, bomb_timer_system
                .run_if(in_state(AppState::Gameplay)))
            .add_systems(Update, update_particles_system.after(bomb_timer_system)
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))))
            .add_systems(Update, death_delay_system
                .run_if(in_state(AppState::Gameplay)))
            .add_systems(Update, spawn_food_system
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(5000))))
            .add_systems(Update, spawn_bomb_system
                .run_if(in_state(AppState::Gameplay))
                .run_if(on_timer(Duration::from_millis(27000))))
            .add_systems(OnExit(AppState::Gameplay), despawn_gameplay_system)
            .insert_resource(particle_system);
    }
}

fn spawn_background_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Running spawn background system");
    let scale_factor = crate::common::WINDOW_HEIGHT / 99.0;

    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        Transform {
            scale: Vec3::new(scale_factor, scale_factor, 1.0),
            translation: Vec3::new(0.0, 0.0, GameplayPlugin::BACKGROUND_Z_DEPTH),
            ..default()
        },
        BackgroundImage,
    ));
}

fn spawn_walls_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Running spawn walls system");
    for x in 0..=GRID_WIDTH {
        for y in [0, GRID_HEIGHT] {
            let position = GridPosition::new(x, y);
            spawn_wall(&mut commands, &asset_server, position);
        }
    }
    for y in 0..GRID_HEIGHT {
        for x in [0, GRID_WIDTH] {
            let position = GridPosition::new(x, y);
            spawn_wall(&mut commands, &asset_server, position);
        }
    }
}

fn spawn_wall(commands: &mut Commands, asset_server: &Res<AssetServer>, position: GridPosition) {
    let scale_factor = 1.0;
    let screen_pos = ScreenPosition::from(position);

    commands.spawn((
        Sprite::from_image(asset_server.load("brickwall.png")),
        Transform {
            scale: Vec3::new(scale_factor, scale_factor, 1.0),
            translation: Vec3::new(screen_pos.x, screen_pos.y, GameplayPlugin::WALL_Z_DEPTH),
            ..default()
        }))
        .insert(position)
        .insert(Wall::new());
}

fn spawn_snake_system(mut commands: Commands) {
    debug!("Running spawn snake system");
    let position = GridPosition::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
    commands
        .spawn((
            Sprite {
                color: SnakeHead::HEAD_COLOR,
                ..default()
            },
            Transform {
                scale: Vec3::new(GRID_SIZE, GRID_SIZE, 1.0),
                translation: Vec3::new(0.0, 0.0, GameplayPlugin::SNAKE_HEAD_Z_DEPTH),
                ..default()
            },
        ))
        .insert(SnakeHead::new())
        .insert(position);
}

fn control_snake_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut q: Query<&mut SnakeHead>) {
    let Ok(mut snake) = q.single_mut() else { return };

    if snake.next_turn {
        return;
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        snake.turn(crate::common::Direction::Left);
    }
    else if keyboard_input.pressed(KeyCode::ArrowRight) {
        snake.turn(crate::common::Direction::Right);
    }
    else if keyboard_input.pressed(KeyCode::ArrowDown) {
        snake.turn(crate::common::Direction::Down);
    }
    else if keyboard_input.pressed(KeyCode::ArrowUp) {
        snake.turn(crate::common::Direction::Up);
    }
}

// move last body piece where the head is (before the move)
// move the head one step to current direction
fn move_snake_system(mut head_q: Query<(&mut GridPosition, &mut Transform, &mut SnakeHead)>,
                     mut body_q: Query<(&mut GridPosition, &mut Transform), (With<SnakeBodyPiece>, Without<SnakeHead>)>) {
    // the last body piece to the front
    let Ok((mut head_position, mut transform, mut snake)) = head_q.single_mut() else { return };
    if let Some(entity) = snake.get_last_body_piece() {
        if let Ok((mut position, mut transform)) = body_q.get_mut(entity) {
            *position = *head_position;
            let screen_pos = ScreenPosition::from(*position);
            transform.translation.x = screen_pos.x;
            transform.translation.y = screen_pos.y;
            snake.move_last_body_piece_to_front();
        }
    }

    // move snake head one step forward
    head_position.move_position(snake.direction, 1);
    let screen_pos = ScreenPosition::from(*head_position);
    transform.translation.x = screen_pos.x;
    transform.translation.y = screen_pos.y;
    snake.next_turn = false;
}

fn grow_snake_system(mut commands: Commands,
                     mut q: Query<(&mut SnakeHead, &GridPosition)>) {
    let Ok((mut snake, position)) = q.single_mut() else { return };
    if !snake.can_grow() {
        return;
    }

    debug!("Spawning new snake body piece at position: {}", *position);
    let screen_pos = ScreenPosition::from(*position);
    let entity = commands.spawn((
        Sprite {
            color: SnakeBodyPiece::BODY_COLOR,
            ..default()
        },
        Transform {
            scale: Vec3::new(GRID_SIZE, GRID_SIZE, 1.0),
            translation: Vec3::new(
                screen_pos.x,
                screen_pos.y,
                GameplayPlugin::SNAKE_BODY_Z_DEPTH),
            ..default()
        }))
        .insert(*position)
        .insert(SnakeBodyPiece::new())
        .id();
    snake.add_body_piece(entity);
}

fn spawn_food_system(mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     query: Query<&GridPosition>) {
    let position = find_free_position(query);
    let screen_pos = ScreenPosition::from(position);
    let food = Food::random();
    debug!("Spawning food at position: {}", position);
    commands.spawn((
        Sprite::from_image(asset_server.load(food.asset.clone())),
        Transform::from_xyz(screen_pos.x, screen_pos.y, GameplayPlugin::FOOD_Z_DEPTH)))
        .insert(food)
        .insert(position);
}

fn spawn_bomb_system(mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
                     query: Query<&GridPosition>) {
    let position = find_free_position(query);
    let screen_pos = ScreenPosition::from(position);
    let bomb = Bomb::default();
    let scale_factor = 3.0;
    debug!("Spawning a bomb at position: {}", position);

    let texture_handle = asset_server.load("bomb_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(30, 30), 4, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);
    commands.spawn((
        Sprite::from_atlas_image(
            texture_handle,
            TextureAtlas {
                layout: layout_handle,
                index: 0,
            }),
        Transform {
            scale: Vec3::new(scale_factor, scale_factor, 1.0),
            translation: Vec3::new(screen_pos.x, screen_pos.y, GameplayPlugin::BOMB_Z_DEPTH),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ))
    .insert(bomb)
    .insert(position);
}

fn wall_collision_system(mut state: ResMut<NextState<AppState>>,
                         snake_pos_q: Query<&GridPosition, With<SnakeHead>>) {
    let Ok(snake_position) = snake_pos_q.single() else { return };

    if (snake_position.x <= 0) || (snake_position.x >= GRID_WIDTH) {
        state.set(AppState::GameOver);
    }
    if (snake_position.y <= 0) || (snake_position.y >= GRID_HEIGHT) {
        state.set(AppState::GameOver);
    }
}

fn food_collision_system(mut commands: Commands,
                         mut snake_query: Query<(&mut SnakeHead, &GridPosition)>,
                         food_query: Query<(Entity, &Food, &GridPosition), With<Food>>) {
    let Ok((mut snake, snake_position)) = snake_query.single_mut() else { return };

    for (entity, food, food_position) in food_query.iter() {
        if snake_position == food_position {
            debug!("Food eaten. Despawning a food at position: {}", food_position);
            snake.grow(food.value);
            commands.entity(entity).despawn();
        }
    }
}

fn bomb_collision_system(mut commands: Commands,
                         mut snake_position_query: Query<&GridPosition, With<SnakeHead>>,
                         bomb_query: Query<(Entity, &GridPosition), With<Bomb>>) {
    let Ok(snake_position) = snake_position_query.single_mut() else { return };

    for (entity, bomb_position) in bomb_query.iter() {
        if snake_position == bomb_position {
            debug!("Bomb eaten. Despawning a bomb at position: {}", bomb_position);
            commands.entity(entity).despawn();
        }
    }
}

fn snake_body_collision_system(mut state: ResMut<NextState<AppState>>,
                               snake_query: Query<(&SnakeHead, &GridPosition), With<SnakeHead>>,
                               body_query: Query<&GridPosition, With<SnakeBodyPiece>>) {
    let Ok((snake, snake_position)) = snake_query.single() else { return };
    let mut next_position = snake_position.clone();
    next_position.move_position(snake.direction, 1);
    for body_position in body_query.iter() {
        if next_position == *body_position {
            state.set(AppState::GameOver);
        }
    }
}

fn bomb_timer_system(mut commands: Commands,
                     mut particle_system: ResMut<ParticleSystem>,
                     snake_query: Query<&GridPosition, Or<(With<SnakeHead>, With<SnakeBodyPiece>)>>,
                     mut bomb_query: Query<(Entity, &mut Bomb, &GridPosition), With<Bomb>>,
                     time: Res<Time>) {
    for (entity, mut bomb, position) in bomb_query.iter_mut() {
        bomb.timer.tick(time.delta());
        if bomb.timer.is_finished() {
            debug!("Bomb exploded at position: {}", position);
            particle_system.create_explosion(&mut commands, ScreenPosition::from(*position));
            check_if_snake_exploded(&mut commands, &snake_query, position);
            commands.entity(entity).despawn();
        }
    }
}

fn update_particles_system(commands: Commands,
                           query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
                           mut particle_system: ResMut<ParticleSystem>) {
    particle_system.update(commands, query);
}

fn sprite_animation_system(
    time: Res<Time>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut AnimationTimer, &mut Sprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let Some(atlas) = sprite.texture_atlas.as_mut() else { continue };
            let layout = texture_atlas_layouts.get(&atlas.layout).unwrap();
            atlas.index = (atlas.index + 1) % layout.textures.len();
        }
    }
}

// when a bomb is exploding, and snake is next to it,
// wait a while and show the effect on the screen before ending the game
fn death_delay_system(mut state: ResMut<NextState<AppState>>,
                      mut query: Query<&mut DeathTimer>,
                      time: Res<Time>) {
    for mut timer in &mut query {
        timer.tick(time.delta());
        if timer.is_finished() {
            state.set(AppState::GameOver);
        }
    }
}

fn despawn_gameplay_system(mut commands: Commands,
                           mut particle_system: ResMut<ParticleSystem>,
                           query: Query<Entity, Or<(With<Food>, With<SnakeHead>, With<SnakeBodyPiece>, With<Bomb>, With<DeathTimer>)>>) {
    // notice that Walls and BackgroundImage are not cleaned up
    // GameOver system will cleanup everything
    debug!("Running despawn gameplay system");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    particle_system.despawn_particles(commands);
}

// a bad way of finding a free position from the grid
// by randomly testing for a free positions
fn find_free_position(query: Query<&GridPosition>) -> GridPosition {
    let mut position = GridPosition::random(GRID_WIDTH, GRID_HEIGHT);
    loop {
        if query.iter().find_map(|p| if *p == position { Some(p) } else { None }) == None {
            break;
        }
        position = GridPosition::random(GRID_WIDTH, GRID_HEIGHT);
    }
    position
}

fn check_if_snake_exploded(commands: &mut Commands,
                           snake_query: &Query<&GridPosition, Or<(With<SnakeHead>, With<SnakeBodyPiece>)>>,
                           bomb_position: &GridPosition) {
    debug!("Checking if snake exploded...");
    for snake_piece_position in snake_query.iter() {
        debug!("Snake pos: {}, Bomb pos: {}", snake_piece_position, bomb_position);
        if ((snake_piece_position.x as i32 - bomb_position.x as i32).abs() <= 1) &&
            ((snake_piece_position.y as i32 - bomb_position.y as i32).abs() <= 1) {
            debug!("Snake is in the explosion zone!");
            commands.spawn(DeathTimer::default());
            break;
        }
    }
}
