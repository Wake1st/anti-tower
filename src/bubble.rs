use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{Ccd, RigidBody, Sleeping},
    geometry::{ActiveEvents, Collider, CollisionGroups, Group, Restitution, Sensor},
};

use crate::{
    collisions::CollisionDamage,
    detection::{DetectionEvent, Target, Tracker},
    health::Health,
    movement::{Acceleration, KinematicBundle, Velocity},
    player::Player,
    schedule::InGameSet,
};

const SPAWNER_SPAWN_OFFSET: f32 = 32.0;
const SPAWNER_SPRITE_LAYER: f32 = -1.0;
const SPAWNER_SPAWN_RATE: f32 = 2.0;
const SPAWNER_HEALTH: f32 = 80.0;
const SPAWNER_COLLIDER_SIZE: Vec2 = Vec2::new(32.0, 32.0);

const BUBBLE_SPAWN_OFFSET: f32 = 6.0;
const BUBBLE_SPRITE_LAYER: f32 = 1.0;
const BUBBLE_ACCELERATION_RATE: f32 = 1800.0;
const BUBBLE_LIFETIME: f32 = 6.0;
const BUBBLE_COLLIDER_RADIUS: f32 = 8.0;
const BUBBLE_HEALTH: f32 = 1.0;
const BUBBLE_COLLISION_DAMAGE: f32 = 3.0;
const BUBBLE_BOUNCINESS: f32 = 0.8;
const BUBBLE_TRACKER_VISION: f32 = 420.0;

pub struct BubblePlugin;

impl Plugin for BubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bubble_lifetime).in_set(InGameSet::DespawnEntities))
            .add_systems(
                Update,
                (spawn_bubble_spawner, spawn_bubble, bubble_tracking)
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
        RigidBody::Fixed,
        CollisionGroups::new(Group::GROUP_4, Group::GROUP_2),
        Collider::cuboid(SPAWNER_COLLIDER_SIZE.x / 2.0, SPAWNER_COLLIDER_SIZE.y / 2.0),
        Health::new(SPAWNER_HEALTH),
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
                // Collider::new(BUBBLE_COLLIDER_RADIUS),
                Sleeping::disabled(),
                Ccd::enabled(),
                RigidBody::KinematicVelocityBased,
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                CollisionGroups::new(Group::GROUP_11, Group::GROUP_2),
                Collider::ball(BUBBLE_COLLIDER_RADIUS),
                Restitution::coefficient(BUBBLE_BOUNCINESS),
                Health::new(BUBBLE_HEALTH),
                CollisionDamage::new(BUBBLE_COLLISION_DAMAGE),
                Tracker::new(BUBBLE_TRACKER_VISION),
                Bubble {
                    lifetime: Timer::from_seconds(BUBBLE_LIFETIME, TimerMode::Once),
                },
                Name::new("Bubble"),
            ));
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
        }
    }
}

fn bubble_tracking(
    mut detection_event_reader: EventReader<DetectionEvent>,
    mut bubble_query: Query<(&GlobalTransform, &mut Acceleration), With<Bubble>>,
    target_query: Query<&GlobalTransform, With<Target>>,
) {
    for &DetectionEvent {
        tracker_entity,
        target_entity,
    } in detection_event_reader.read()
    {
        let Ok((bubble_transform, mut bubble_acceleration)) = bubble_query.get_mut(tracker_entity)
        else {
            continue;
        };

        let Ok(target_transform) = target_query.get(target_entity) else {
            continue;
        };

        let direction =
            (target_transform.translation() - bubble_transform.translation()).normalize();
        let distance = bubble_transform
            .translation()
            .distance(target_transform.translation());

        bubble_acceleration.value = direction * BUBBLE_ACCELERATION_RATE / distance;
    }
}
