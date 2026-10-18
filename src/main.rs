// Snake - main

#[doc = include_str!("../README.md")]

use bevy::prelude::*;
use bevy::color::palettes::css::GRAY;
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
use common::{BackgroundImage, GameOverText};
use wall::Wall;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            LogPlugin {
                filter: "error,wgpu_core=error,wgpu_hal=error,snake=debug".into(),
                level: bevy::log::Level::DEBUG,
                ..default()
            }).set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Snake".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
            )
        .init_state::<AppState>()
        .add_plugins((splashscreen::SplashScreenPlugin, gameplay::GameplayPlugin))
        .add_systems(Startup, setup_system)
        .add_systems(Update, exit_system)
        .add_systems(OnEnter(AppState::GameOver), game_over_system)
        .add_systems(Update, game_over_input_system.run_if(in_state(AppState::GameOver)))
        .add_systems(OnExit(AppState::GameOver), despawn_game_over_system)
        .run();
}

fn setup_system(mut commands: Commands,
                mut primary_query: Query<&mut Window, With<PrimaryWindow>>) {
    debug!("Running setup system");
    let Ok(mut window) = primary_query.single_mut() else { return };
    window.title = "Snake".to_string();
    window.resizable = false;
    window.resolution = (WINDOW_WIDTH, WINDOW_HEIGHT).into();
    commands.spawn(Camera2d);
}

fn exit_system(keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn game_over_input_system(mut state: ResMut<NextState<AppState>>,
                          keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        state.set(AppState::Gameplay);
    }
}

fn game_over_system(mut commands: Commands,
                    asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("Game over"),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 100.0,
            ..default()
        },
        TextColor(GRAY.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(50.0),
            right: Val::Px(250.0),
            ..default()
        },
        GameOverText,
    ));
    commands.spawn((
        Text::new("Press space to continue"),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 45.0,
            ..default()
        },
        TextColor(GRAY.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(40.0),
            right: Val::Px(250.0),
            ..default()
        },
        GameOverText,
    ));
}

fn despawn_game_over_system(mut commands: Commands,
                            query: Query<Entity, Or<(With<Wall>, With<BackgroundImage>, With<GameOverText>)>>) {
    debug!("Running despawn game over system");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
