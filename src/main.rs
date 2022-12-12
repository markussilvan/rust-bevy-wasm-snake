// Snake - main

use bevy::prelude::*;
use bevy::app::AppExit;
use iyes_loopless::prelude::*;

mod snake;
mod food;
mod common;
mod wall;

use snake::Snake;
use food::Food;
use common::Position;
use wall::Wall;

const SNAKE_Z_DEPTH: f32 = 100.0;
const FOOD_Z_DEPTH: f32 = 50.0;
const WALL_Z_DEPTH: f32 = 200.0;

const GRID_WIDTH: u32 = 30;
const GRID_HEIGHT: u32 = 20;
const GRID_SIZE: f32 = 30.0;

const WINDOW_WIDTH: f32 = GRID_WIDTH as f32 * GRID_SIZE;
const WINDOW_HEIGHT: f32 = GRID_HEIGHT as f32 * GRID_SIZE;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    SplashScreen,
    Gameplay,
    GameOver
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(exit_system)
        .add_state(AppState::SplashScreen)
        .add_fixed_timestep(
            std::time::Duration::from_millis(3000),
            "splashscreen_delay",
        )
        .add_fixed_timestep(
            std::time::Duration::from_millis(300),
            "splashscreen_effect_delay",
        )
        .add_fixed_timestep(
            std::time::Duration::from_millis(5000),
            "gameplay_food_spawn_delay",
        )
        .add_fixed_timestep(
            std::time::Duration::from_millis(300),
            "gameplay_move_delay",
        )
        .add_system_set(
            SystemSet::on_enter(AppState::SplashScreen)
                .with_system(spawn_splashscreen_system))
        .add_fixed_timestep_system(
            "splashscreen_delay",
            0,
            start_game_system.run_if(in_splashscreen))
        .add_fixed_timestep_system(
            "splashscreen_effect_delay",
            0,
            change_color_system.run_if(in_splashscreen))
        .add_system_set(
            SystemSet::on_enter(AppState::Gameplay)
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
            SystemSet::on_enter(AppState::GameOver)
                .with_system(game_over_system))
        .run();
}

fn convert_to_screen_coordinates(position: Position) -> (f32, f32) {
    let x: f32 = position.x as f32 * GRID_SIZE - (WINDOW_WIDTH / 2.0);
    let y: f32 = position.y as f32 * GRID_SIZE - (WINDOW_HEIGHT / 2.0);
    (x, y)
}

fn in_splashscreen(state: Res<State<AppState>>) -> bool {
    in_expected_state(state, AppState::SplashScreen)
}

fn in_gameplay(state: Res<State<AppState>>) -> bool {
    in_expected_state(state, AppState::Gameplay)
}

fn in_expected_state(state: Res<State<AppState>>, expected: AppState) -> bool {
    if *state.current() == expected {
        true
    }
    else {
        false
    }
}

fn setup_system(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut windows: ResMut<Windows>) {
    println!("Running setup system");
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Snake".to_string());
    window.set_resizable(false);
    window.set_resolution(WINDOW_WIDTH, WINDOW_HEIGHT);
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("logo.png"),
        ..default()
    });
}

fn spawn_splashscreen_system(mut commands: Commands,
                             asset_server: Res<AssetServer>) {
    println!("Running setup splashscreen system");
    commands.spawn(SpriteBundle {
        texture: asset_server.load("logo.png"),
        ..default()
    });
}

fn change_color_system(time: Res<Time>, mut query: Query<&mut Sprite>, state: ResMut<State<AppState>>) {
    println!("Running change color system in state: {:?}", state.current());
    for mut sprite in &mut query {
        sprite
            .color
            .set_b((time.elapsed_seconds() * 0.1).sin() + 2.0);
    }
}

fn start_game_system(mut state: ResMut<State<AppState>>) {
    println!("Running start game system in state: {:?}", state.current());
    state.set(AppState::Gameplay).unwrap();
}

fn end_game_system(mut state: ResMut<State<AppState>>) {
    println!("Running end game system in state: {:?}", state.current());
    state.set(AppState::GameOver).unwrap();
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
                translation: Vec3::new(x, y, WALL_Z_DEPTH),
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
                translation: Vec3::new(0.0, 0.0, SNAKE_Z_DEPTH),
                ..default()
            },
            ..default()
        })
        .insert(Snake::new())
        .insert(position);
}

fn exit_system(keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn control_snake_system(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut Snake>) {
    let mut snake = q.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        snake.turn(common::Direction::Left);
    }
    else if keyboard_input.pressed(KeyCode::Right) {
        snake.turn(common::Direction::Right);
    }
    else if keyboard_input.pressed(KeyCode::Down) {
        snake.turn(common::Direction::Down);
    }
    else if keyboard_input.pressed(KeyCode::Up) {
        snake.turn(common::Direction::Up);
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
                translation: Vec3::new(x, y, FOOD_Z_DEPTH),
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

fn game_over_system(mut commands: Commands,
                    asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Game over",
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::BLACK,
            }
        )
        .with_text_alignment(TextAlignment::CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Percent(50.0),
                right: Val::Px(250.0),
                //bottom: Val::Percent(50.0),
                //right: Val::Percent(25.0),
                ..default()
            },
            ..default()
        }),
    ));
}

