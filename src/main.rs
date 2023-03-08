// Snake - main

#[doc = include_str!("../README.md")]

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::log::LogPlugin;
use bevy::window::PrimaryWindow;

mod snake;
mod food;
mod bomb;
mod common;
mod wall;
mod splashscreen;
mod gameplay;

use common::AppState;
use common::{WINDOW_WIDTH, WINDOW_HEIGHT};
use common::{BackgroundImage, Text};
use wall::Wall;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(
            LogPlugin {
                filter: "error,wgpu_core=error,wgpu_hal=error,snake=debug".into(),
                level: bevy::log::Level::DEBUG
            }).set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Snake".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
            )
        .add_plugin(splashscreen::SplashScreenPlugin)
        .add_plugin(gameplay::GameplayPlugin)
        .add_startup_system(setup_system)
        .add_system(exit_system)
        .add_system(game_over_system.in_schedule(OnEnter(AppState::GameOver)))
        .add_system(game_over_input_system.in_set(OnUpdate(AppState::GameOver)))
        .add_system(despawn_game_over_system.in_schedule(OnExit(AppState::GameOver)))
        .run();
}

fn setup_system(mut commands: Commands,
                mut primary_query: Query<&mut Window, With<PrimaryWindow>>) {
    debug!("Running setup system");
    let mut window = primary_query.single_mut();
    window.title = "Snake".to_string();
    window.resizable = false;
    window.resolution = (WINDOW_WIDTH, WINDOW_HEIGHT).into();
    commands.spawn(Camera2dBundle::default());
}

fn exit_system(keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn game_over_input_system(mut state: ResMut<NextState<AppState>>,
                          keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        state.set(AppState::Gameplay);
    }
}

fn game_over_system(mut commands: Commands,
                    asset_server: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_section(
            "Game over",
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::GRAY,
            }
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Percent(50.0),
                right: Val::Px(250.0),
                ..default()
            },
            ..default()
        }),
    ).insert(Text);
    commands.spawn(
        TextBundle::from_section(
            "Press space to continue",
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 45.0,
                color: Color::GRAY,
            }
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Percent(40.0),
                right: Val::Px(250.0),
                ..default()
            },
            ..default()
        }),
    ).insert(Text);
}

fn despawn_game_over_system(mut commands: Commands,
                            query: Query<Entity, Or<(&Wall, &BackgroundImage, &Text)>>) {
    debug!("Running despawn game over system");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
