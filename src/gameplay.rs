use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::common::in_expected_state;
use crate::common::convert_to_screen_coordinates;
use crate::common::AppState;
use crate::common::BackgroundImage;
use crate::common::Position;
use crate::common::{GRID_SIZE, GRID_WIDTH, GRID_HEIGHT};
use crate::snake::Snake;
use crate::wall::Wall;
use crate::food::Food;

pub struct GameplayPlugin;

impl GameplayPlugin {
    const SNAKE_Z_DEPTH: f32 = 100.0;
    const FOOD_Z_DEPTH: f32 = 50.0;
    const WALL_Z_DEPTH: f32 = 200.0;
    const BACKGROUND_Z_DEPTH: f32 = 0.0;
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_fixed_timestep(
                std::time::Duration::from_millis(5000),
                "gameplay_food_spawn_delay",
            )
            .add_fixed_timestep(
                std::time::Duration::from_millis(250),
                "gameplay_move_delay",
            )
            .add_system_set(
                SystemSet::on_enter(AppState::Gameplay)
                    .with_system(spawn_background_system)
                    .with_system(spawn_walls_system)
                    .with_system(spawn_snake_system))
            .add_system_set(
                SystemSet::on_update(AppState::Gameplay)
                    .with_system(control_snake_system))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                wall_collision_detection_system.run_if(in_gameplay))
            .add_fixed_timestep_system(
                "gameplay_move_delay",
                0,
                move_snake_system.run_if(in_gameplay))
            .add_fixed_timestep_system(
                "gameplay_food_spawn_delay",
                0,
                spawn_food_system.run_if(in_gameplay))
            .add_system_set(
                SystemSet::on_exit(AppState::Gameplay)
                    .with_system(despawn_gameplay_system));
    }
}

fn in_gameplay(state: Res<State<AppState>>) -> bool {
    in_expected_state(state, AppState::Gameplay)
}

fn spawn_background_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Running spawn background system");
    let scale_factor = crate::common::WINDOW_HEIGHT / 100.0;

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("tile-background.png"),
            transform: Transform {
                scale: Vec3::new(scale_factor, scale_factor, 1.0),
                translation: Vec3::new(0.0, 0.0, GameplayPlugin::BACKGROUND_Z_DEPTH),
                ..default()
            },
            ..default()
        }).insert(BackgroundImage);
}

fn spawn_walls_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Running spawn walls system");
    for x in 0..GRID_WIDTH+1 {
        for y in [0, GRID_HEIGHT] {
            let position = Position::new(x as i32, y as i32);
            spawn_wall(&mut commands, &asset_server, position);
        }
    }
    for y in 0..GRID_HEIGHT {
        for x in [0, GRID_WIDTH] {
            let position = Position::new(x as i32, y as i32);
            spawn_wall(&mut commands, &asset_server, position);
        }
    }
}

fn spawn_wall(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Position) {
    let scale_factor = GRID_SIZE / 70.0;
    let (x, y) = convert_to_screen_coordinates(position);

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("wall.png"),
            transform: Transform {
                scale: Vec3::new(scale_factor, scale_factor, 1.0),
                translation: Vec3::new(x, y, GameplayPlugin::WALL_Z_DEPTH),
                ..default()
            },
            ..default()
        })
        .insert(position)
        .insert(Wall::new());
}

fn spawn_snake_system(mut commands: Commands) {
    println!("Running spawn snake system");
    let position = Position::new(GRID_WIDTH as i32 / 2, GRID_HEIGHT as i32 / 2);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Snake::HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(GRID_SIZE, GRID_SIZE, 1.0),
                translation: Vec3::new(0.0, 0.0, GameplayPlugin::SNAKE_Z_DEPTH),
                ..default()
            },
            ..default()
        })
        .insert(Snake::new())
        .insert(position);
}

fn control_snake_system(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut Snake>) {
    let mut snake = q.single_mut();

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

fn move_snake_system(mut q: Query<(&mut Position, &mut Transform, &Snake)>) {
    let (mut position, mut transform, snake) = q.single_mut();
    position.move_position(snake.direction, 1);
    println!("Snake position: {}", *position);

    (transform.translation.x, transform.translation.y) = convert_to_screen_coordinates(*position);
}

fn spawn_food_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let food = Food::default();
    let position = Position::random(GRID_WIDTH, GRID_HEIGHT);
    let (x, y) = convert_to_screen_coordinates(position);

    println!("Spawning food at position: {}", position);
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("apple.png"),
            transform: Transform {
                translation: Vec3::new(x, y, GameplayPlugin::FOOD_Z_DEPTH),
                ..default()
            },
            ..default()
        })
        .insert(food)
        .insert(position);
}

fn wall_collision_detection_system(mut state: ResMut<State<AppState>>,
                                   snake_pos_q: Query<&Position, With<Snake>>) {
    let snake_position = snake_pos_q.single();

    if (snake_position.x <= 0) || (snake_position.x >= GRID_WIDTH as i32) {
        state.set(AppState::GameOver).unwrap();
    }
    if (snake_position.y <= 0) || (snake_position.y >= GRID_HEIGHT as i32) {
        state.set(AppState::GameOver).unwrap();
    }
}

fn despawn_gameplay_system(mut commands: Commands,
                               query: Query<Entity, Or<(&Food, &Snake)>>) {
    // notice that Walls and BackgroundImage are not cleaned up
    // GameOver system will cleanup everything
    println!("Running despawn gameplay system");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
