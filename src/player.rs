use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::schedule::InGameSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (character_movement).in_set(InGameSet::UserInput));
    }
}

#[derive(Component, InspectorOptions, Default)]
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
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= movement_amount;
        }
    }
}
