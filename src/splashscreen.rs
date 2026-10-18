use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

use crate::common::AppState;
use crate::common::BackgroundImage;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::SplashScreen), spawn_splashscreen_system)
            .add_systems(Update, start_game_system
                .run_if(in_state(AppState::SplashScreen))
                .run_if(on_timer(Duration::from_millis(3000))))
            .add_systems(OnExit(AppState::SplashScreen), despawn_splashscreen_system);
    }
}

fn spawn_splashscreen_system(mut commands: Commands,
                             asset_server: Res<AssetServer>) {
    debug!("Running setup splashscreen system");
    let scale_factor = 0.5;
    commands.spawn((
        Sprite::from_image(asset_server.load("logo.png")),
        Transform {
            scale: Vec3::new(scale_factor, scale_factor, 1.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        BackgroundImage,
    ));
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
