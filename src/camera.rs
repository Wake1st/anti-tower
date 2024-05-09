use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::SEA_GREEN),
        },
        ..default()
    };

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 512.0,
        min_height: 288.0,
    };

    commands.spawn(camera);
}

fn camera_follow(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player.single();

    for mut transform in &mut camera {
        let pos = player_transform.translation;
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
