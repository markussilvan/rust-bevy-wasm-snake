use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

use crate::common::AppState;
use crate::common::BackgroundImage;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_splashscreen_system.in_schedule(OnEnter(AppState::SplashScreen)))
            .add_system(start_game_system
                .in_set(OnUpdate(AppState::SplashScreen))
                .run_if(on_timer(Duration::from_millis(3000))))
            .add_system(despawn_splashscreen_system.in_schedule(OnExit(AppState::SplashScreen)));
    }
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

fn start_game_system(mut state: ResMut<NextState<AppState>>) {
    debug!("Running start game system");
    state.set(AppState::Gameplay);
}

fn despawn_splashscreen_system(mut commands: Commands,
                               query: Query<Entity, With<BackgroundImage>>) {
    debug!("Running despawn splashscreen system");
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}
