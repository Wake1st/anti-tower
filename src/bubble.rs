use bevy::prelude::*;

use crate::player::Player;

pub struct BubblePlugin;

impl Plugin for BubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_bubble_spawner,
                spawn_bubble,
                bubble_lifetime,
                bubble_movement,
            ),
        );
    }
}

#[derive(Component)]
pub struct BubbleSpawner {
    pub spawn_rate: Timer,
}

#[derive(Component)]
pub struct Bubble {
    pub speed: f32,
    pub lifetime: Timer,
}

fn spawn_bubble_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::W) {
        return;
    }

    let texture: Handle<Image> = asset_server.load("bubble spawner.png");

    let player_transform = player.single();

    commands.spawn((
        SpatialBundle::default(),
        SpriteBundle {
            texture,
            transform: *player_transform,
            ..default()
        },
        BubbleSpawner {
            spawn_rate: Timer::from_seconds(6.0, TimerMode::Repeating),
        },
        Name::new("BubbleSpawner"),
    ));
}

fn spawn_bubble(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawners: Query<(Entity, &mut BubbleSpawner, &Transform)>,
) {
    for (entity, mut spawner, &transform) in &mut spawners {
        spawner.spawn_rate.tick(time.delta());

        if spawner.spawn_rate.just_finished() {
            commands.entity(entity).with_children(|commands| {
                let texture: Handle<Image> = asset_server.load("bubble.png");

                commands.spawn((
                    SpriteBundle {
                        texture,
                        transform,
                        ..default()
                    },
                    Bubble {
                        speed: 20.0,
                        lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                    },
                    Name::new("Bubble"),
                ));
            });
        }
    }
}

fn bubble_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut bubbles: Query<(Entity, &mut Bubble)>,
) {
    for (bubble_entity, mut bubble) in &mut bubbles {
        bubble.lifetime.tick(time.delta());

        if bubble.lifetime.finished() {
            commands.entity(bubble_entity).despawn_recursive();
            info!("Bubble popped from old age.");
        }
    }
}

fn bubble_movement(time: Res<Time>, mut bubbles: Query<(&Bubble, &mut Transform), With<Bubble>>) {
    for (bubble, mut transform) in &mut bubbles {
        transform.translation.x += bubble.speed * f32::cos(time.elapsed_seconds());
        transform.translation.y += bubble.speed * f32::sin(time.elapsed_seconds());
    }
}
