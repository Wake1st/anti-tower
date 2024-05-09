use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_camera))
            .add_systems(Update, (character_movement, camera_follow).chain());
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.0 },
        Name::new("Player"),
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::Up) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= movement_amount;
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::MIDNIGHT_BLUE),
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

// fn camera_follow(
//     player: Query<&Transform, With<Player>>,
//     mut camera: Query<(&mut Camera, &mut Transform), Without<Player>>,
// ) {
//     let Ok(player) = player.get_single() else {
//         return;
//     };
//     let Ok((mut camera, mut camera_transform)) = camera.get_single_mut() else {
//         return;
//     };

//     let delta = player.translation - camera.focus;

//     if delta != Vec3::ZERO {
//         camera.focus = player.translation;
//         camera_transform.translation += delta;
//     }
// }
