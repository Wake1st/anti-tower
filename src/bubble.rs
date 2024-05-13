use bevy::prelude::*;

use crate::{
    collisions::{Collider, CollisionDamage},
    health::Health,
    movement::{Acceleration, KinematicBundle, Velocity},
    player::Player,
    schedule::InGameSet,
};

const SPAWNER_SPAWN_OFFSET: f32 = 32.0;
const SPAWNER_SPRITE_LAYER: f32 = -1.0;
const SPAWNER_SPAWN_RATE: f32 = 2.0;

const BUBBLE_SPAWN_OFFSET: f32 = 6.0;
const BUBBLE_SPRITE_LAYER: f32 = 1.0;
const BUBBLE_SPEED: f32 = 1.2;
const BUBBLE_LIFETIME: f32 = 6.0;
const BUBBLE_COLLIDER_RADIUS: f32 = 8.0;
const BUBBLE_HEALTH: f32 = 1.0;
const BUBBLE_COLLISION_DAMAGE: f32 = 3.0;

pub struct BubblePlugin;

impl Plugin for BubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bubble_lifetime).in_set(InGameSet::DespawnEntities))
            .add_systems(
                Update,
                (spawn_bubble_spawner, spawn_bubble, bubble_movement)
                    .in_set(InGameSet::EntityUpdates),
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
    input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::KeyW) {
        return;
    }

    let texture: Handle<Image> = asset_server.load("bubble spawner.png");

    let Ok(player_transform) = player.get_single() else {
        return;
    };

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3 {
                    x: player_transform.translation.x,
                    y: player_transform.translation.y + SPAWNER_SPAWN_OFFSET,
                    z: SPAWNER_SPRITE_LAYER,
                },
                ..default()
            },
            ..default()
        },
        BubbleSpawner {
            spawn_rate: Timer::from_seconds(SPAWNER_SPAWN_RATE, TimerMode::Repeating),
        },
        Name::new("BubbleSpawner"),
    ));
}

fn spawn_bubble(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawners: Query<(&mut BubbleSpawner, &Transform)>,
) {
    for (mut spawner, &spawner_transform) in &mut spawners {
        spawner.spawn_rate.tick(time.delta());

        if spawner.spawn_rate.just_finished() {
            let texture: Handle<Image> = asset_server.load("bubble.png");

            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3 {
                            x: spawner_transform.translation.x,
                            y: spawner_transform.translation.y + BUBBLE_SPAWN_OFFSET,
                            z: BUBBLE_SPRITE_LAYER,
                        },
                        ..default()
                    },
                    ..default()
                },
                KinematicBundle {
                    velocity: Velocity::new(Vec3::ZERO),
                    acceleration: Acceleration::new(Vec3::ZERO),
                },
                Collider::new(BUBBLE_COLLIDER_RADIUS),
                Health::new(BUBBLE_HEALTH),
                CollisionDamage::new(BUBBLE_COLLISION_DAMAGE),
                Bubble {
                    speed: BUBBLE_SPEED,
                    lifetime: Timer::from_seconds(BUBBLE_LIFETIME, TimerMode::Once),
                },
                Name::new("Bubble"),
            ));

            info!("Bubble spawned!");
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
            info!("Bubble died from old age.");
        }
    }
}

fn bubble_movement(time: Res<Time>, mut bubbles: Query<(&Bubble, &mut Transform), With<Bubble>>) {
    for (bubble, mut transform) in &mut bubbles {
        transform.translation.x += 0.4 * f32::cos(time.elapsed_seconds() * bubble.speed);
        transform.translation.y += 0.4 * f32::sin(time.elapsed_seconds() * bubble.speed);
    }
}
