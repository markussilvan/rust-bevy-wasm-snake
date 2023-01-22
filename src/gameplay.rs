use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::common::in_expected_state;
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
            .add_fixed_timestep(
                std::time::Duration::from_millis(5000),
                "gameplay_food_spawn_delay",
            )
            .add_fixed_timestep(
                std::time::Duration::from_millis(27000),
                "gameplay_bomb_spawn_delay",
            )
            .add_fixed_timestep(
                std::time::Duration::from_millis(200),
                "gameplay_move_delay",
            )
            .add_system_set(
                SystemSet::on_enter(AppState::Gameplay)
                    .with_system(spawn_background_system)
                    .with_system(spawn_walls_system)
                    .with_system(spawn_snake_system))
            .add_system_set(
                SystemSet::on_update(AppState::Gameplay)
                    .with_system(control_snake_system)
                    .with_system(sprite_animation_system))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                wall_collision_system.run_if(in_gameplay))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                grow_snake_system.run_if(in_gameplay).after("move"))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                move_snake_system.run_if(in_gameplay).label("move"))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                food_collision_system.run_if(in_gameplay))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                bomb_collision_system.run_if(in_gameplay))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                snake_body_collision_system.run_if(in_gameplay).after("move"))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                bomb_timer_system.run_if(in_gameplay).after("move"))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                update_particles_system.run_if(in_gameplay).after("move"))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                death_delay_system.run_if(in_gameplay).after("move"))
            .add_fixed_timestep_system(
                "gameplay_food_spawn_delay",
                0,
                spawn_food_system.run_if(in_gameplay))
            .add_fixed_timestep_system(
                "gameplay_bomb_spawn_delay",
                0,
                spawn_bomb_system.run_if(in_gameplay))
            .add_system_set(
                SystemSet::on_exit(AppState::Gameplay)
                    .with_system(despawn_gameplay_system))
            .insert_resource(particle_system);
    }
}

fn in_gameplay(state: Res<State<AppState>>) -> bool {
    in_expected_state(state.current(), AppState::Gameplay)
}

fn spawn_background_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Running spawn background system");
    let scale_factor = crate::common::WINDOW_HEIGHT / 99.0;

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform {
                scale: Vec3::new(scale_factor, scale_factor, 1.0),
                translation: Vec3::new(0.0, 0.0, GameplayPlugin::BACKGROUND_Z_DEPTH),
                ..default()
            },
            ..default()
        }).insert(BackgroundImage);
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

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("brickwall.png"),
            transform: Transform {
                scale: Vec3::new(scale_factor, scale_factor, 1.0),
                translation: Vec3::new(screen_pos.x, screen_pos.y, GameplayPlugin::WALL_Z_DEPTH),
                ..default()
            },
            ..default()
        })
        .insert(position)
        .insert(Wall::new());
}

fn spawn_snake_system(mut commands: Commands) {
    debug!("Running spawn snake system");
    let position = GridPosition::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SnakeHead::HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(GRID_SIZE, GRID_SIZE, 1.0),
                translation: Vec3::new(0.0, 0.0, GameplayPlugin::SNAKE_HEAD_Z_DEPTH),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead::new())
        .insert(position);
}

fn control_snake_system(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut SnakeHead>) {
    let mut snake = q.single_mut();

    if snake.next_turn {
        return;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        snake.turn(crate::common::Direction::Left);
    }
    else if keyboard_input.pressed(KeyCode::Right) {
        snake.turn(crate::common::Direction::Right);
    }
    else if keyboard_input.pressed(KeyCode::Down) {
        snake.turn(crate::common::Direction::Down);
    }
    else if keyboard_input.pressed(KeyCode::Up) {
        snake.turn(crate::common::Direction::Up);
    }
}

// move last body piece where the head is (before the move)
// move the head one step to current direction
fn move_snake_system(mut head_q: Query<(&mut GridPosition, &mut Transform, &mut SnakeHead)>,
                     mut body_q: Query<(&mut GridPosition, &mut Transform), (With<SnakeBodyPiece>, Without<SnakeHead>)>) {
    let (mut head_position, mut transform, mut snake) = head_q.single_mut();
    if let Some(entity) = snake.get_last_body_piece() {
        if let Ok((mut position, mut transform)) = body_q.get_mut(entity) {
            *position = *head_position;
            let screen_pos = ScreenPosition::from(*position);
            transform.translation.x = screen_pos.x;
            transform.translation.y = screen_pos.y;
            snake.move_last_body_piece_to_front();
        }
    }

    // move snake head
    head_position.move_position(snake.direction, 1);
    let screen_pos = ScreenPosition::from(*head_position);
    transform.translation.x = screen_pos.x;
    transform.translation.y = screen_pos.y;
    snake.next_turn = false;
}

fn grow_snake_system(mut commands: Commands,
                     mut q: Query<(&mut SnakeHead, &GridPosition)>) {
    let (mut snake, position) = q.single_mut();
    if !snake.can_grow() {
        return;
    }

    debug!("Spawning new snake body piece at position: {}", *position);
    let screen_pos = ScreenPosition::from(*position);
    let entity = commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: SnakeBodyPiece::BODY_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(GRID_SIZE, GRID_SIZE, 1.0),
                translation: Vec3::new(
                    screen_pos.x,
                    screen_pos.y,
                    GameplayPlugin::SNAKE_BODY_Z_DEPTH),
                ..default()
            },
            ..default()
        })
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
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load(food.asset.as_str()),
            transform: Transform {
                translation: Vec3::new(screen_pos.x, screen_pos.y, GameplayPlugin::FOOD_Z_DEPTH),
                ..default()
            },
            ..default()
        })
        .insert(food)
        .insert(position);
}

fn spawn_bomb_system(mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
                     query: Query<&GridPosition>) {
    let position = find_free_position(query);
    let screen_pos = ScreenPosition::from(position);
    let bomb = Bomb::default();
    let scale_factor = 3.0;
    debug!("Spawning a bomb at position: {}", position);

    let texture_handle = asset_server.load("bomb_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(30.0, 30.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(scale_factor, scale_factor, 1.0),
                translation: Vec3::new(screen_pos.x, screen_pos.y, GameplayPlugin::BOMB_Z_DEPTH),
                ..default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ))
    .insert(bomb)
    .insert(position);
}

fn wall_collision_system(mut state: ResMut<State<AppState>>,
                         snake_pos_q: Query<&GridPosition, With<SnakeHead>>) {
    let snake_position = snake_pos_q.single();

    if (snake_position.x <= 0) || (snake_position.x >= GRID_WIDTH) {
        state.set(AppState::GameOver).unwrap();
    }
    if (snake_position.y <= 0) || (snake_position.y >= GRID_HEIGHT) {
        state.set(AppState::GameOver).unwrap();
    }
}

fn food_collision_system(mut commands: Commands,
                         mut snake_query: Query<(&mut SnakeHead, &GridPosition)>,
                         food_query: Query<(Entity, &Food, &GridPosition), With<Food>>) {
    let (mut snake, snake_position) = snake_query.single_mut();

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
    let snake_position = snake_position_query.single_mut();

    for (entity, bomb_position) in bomb_query.iter() {
        if snake_position == bomb_position {
            debug!("Despawning a bomb at position: {}", bomb_position);
            commands.entity(entity).despawn();
        }
    }
}

fn snake_body_collision_system(mut state: ResMut<State<AppState>>,
                               snake_query: Query<(&SnakeHead, &GridPosition), With<SnakeHead>>,
                               body_query: Query<&GridPosition, With<SnakeBodyPiece>>) {
    let (snake, snake_position) = snake_query.single();
    let mut next_position = snake_position.clone();
    next_position.move_position(snake.direction, 1);
    for body_position in body_query.iter() {
        if next_position == *body_position {
            state.set(AppState::GameOver).unwrap();
        }
    }
}

fn bomb_timer_system(mut commands: Commands,
                     mut particle_system: ResMut<ParticleSystem>,
                     snake_query: Query<&GridPosition, Or<(&SnakeHead, &SnakeBodyPiece)>>,
                     mut bomb_query: Query<(Entity, &mut Bomb, &GridPosition), With<Bomb>>,
                     time: Res<Time>) {
    for (entity, mut bomb, position) in bomb_query.iter_mut() {
        bomb.timer.tick(time.delta());
        if bomb.timer.finished() {
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
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn death_delay_system(mut state: ResMut<State<AppState>>,
                      mut query: Query<&mut DeathTimer>,
                      time: Res<Time>) {
    for mut timer in &mut query {
        timer.tick(time.delta());
        if timer.finished() {
            state.set(AppState::GameOver).unwrap();
        }
    }
}

fn despawn_gameplay_system(mut commands: Commands,
                           mut particle_system: ResMut<ParticleSystem>,
                           query: Query<Entity, Or<(&Food, &SnakeHead, &SnakeBodyPiece, &Bomb)>>) {
    // notice that Walls and BackgroundImage are not cleaned up
    // GameOver system will cleanup everything
    debug!("Running despawn gameplay system");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    particle_system.despawn_particles(commands);
}

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
                           snake_query: &Query<&GridPosition, Or<(&SnakeHead, &SnakeBodyPiece)>>,
                           bomb_position: &GridPosition) {
    debug!("Checking if snake exploded...");
    for snake_piece_position in snake_query.iter() {
        debug!("Snake pos: {}, Bomb pos: {}", snake_piece_position, bomb_position);
        if ((snake_piece_position.x as i32 - bomb_position.x as i32).abs() <= 1) &&
            ((snake_piece_position.y as i32 - bomb_position.y as i32).abs() <= 1) {
            debug!("Snake is in the explosion zone!");
            commands.spawn(
                SpriteBundle {
                    ..default()
                })
                .insert(DeathTimer::default());
            break;
        }
    }
}
