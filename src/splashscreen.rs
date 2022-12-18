use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::common::AppState;
use crate::common::BackgroundImage;
use crate::common::in_expected_state;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_fixed_timestep(
                std::time::Duration::from_millis(3000),
                "splashscreen_delay",
            )
            .add_fixed_timestep(
                std::time::Duration::from_millis(300),
                "splashscreen_effect_delay",
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
                SystemSet::on_exit(AppState::SplashScreen)
                    .with_system(despawn_splashscreen_system));
    }
}

fn in_splashscreen(state: Res<State<AppState>>) -> bool {
    in_expected_state(state, AppState::SplashScreen)
}

fn spawn_splashscreen_system(mut commands: Commands,
                             asset_server: Res<AssetServer>) {
    debug!("Running setup splashscreen system");
    let scale_factor = 0.5;
    commands.spawn(SpriteBundle {
        texture: asset_server.load("logo.png"),
        transform: Transform {
            scale: Vec3::new(scale_factor, scale_factor, 1.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    }).insert(BackgroundImage);
}

fn change_color_system(time: Res<Time>, mut query: Query<&mut Sprite>, state: ResMut<State<AppState>>) {
    debug!("Running change color system in state: {:?}", state.current());
    for mut sprite in &mut query {
        sprite
            .color
            .set_b((time.elapsed_seconds() * 0.1).sin() + 2.0);
    }
}

fn start_game_system(mut state: ResMut<State<AppState>>) {
    debug!("Running start game system in state: {:?}", state.current());
    state.set(AppState::Gameplay).unwrap();
}

fn despawn_splashscreen_system(mut commands: Commands,
                               query: Query<Entity, With<BackgroundImage>>) {
    debug!("Running despawn splashscreen system");
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}
