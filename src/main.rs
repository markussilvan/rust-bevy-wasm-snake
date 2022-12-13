// Snake - main

use bevy::prelude::*;
use bevy::app::AppExit;
//use iyes_loopless::prelude::*;

mod snake;
mod food;
mod common;
mod wall;
mod splashscreen;
mod gameplay;

use common::AppState;
use common::{WINDOW_WIDTH, WINDOW_HEIGHT};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(exit_system)
        .add_state(AppState::SplashScreen)
        .add_plugin(splashscreen::SplashScreenPlugin)
        .add_plugin(gameplay::GameplayPlugin)
        .add_system_set(
            SystemSet::on_enter(AppState::GameOver)
                .with_system(game_over_system))
        .run();
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

fn exit_system(keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
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
                color: Color::GRAY,
            }
        )
        .with_text_alignment(TextAlignment::CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Percent(50.0),
                right: Val::Px(250.0),
                ..default()
            },
            ..default()
        }),
    ));
}

