use bevy::prelude::*;

use crate::{
    collisions::{Collider, CollisionDamage, CollisionGroups},
    detection::{DetectionEvent, DetectionGroups, Target, Tracker},
    footman::Footman,
    group::Group,
    health::Health,
    movement::{Acceleration, KinematicBundle, Velocity},
    player::Player,
    schedule::InGameSet,
    tower::Tower,
    Mana,
};

const SPAWNER_SPAWN_OFFSET: f32 = 32.0;
const SPAWNER_SPRITE_LAYER: f32 = -1.0;
const SPAWNER_SPAWN_RATE: f32 = 2.0;
const SPAWNER_HEALTH: f32 = 80.0;
const SPAWNER_COLLIDER_RADIUS: f32 = 16.0;
const SPAWNER_COST: f32 = 20.0;

const BUBBLE_SPAWN_OFFSET: f32 = 6.0;
const BUBBLE_SPRITE_LAYER: f32 = 1.0;
const BUBBLE_LIFETIME: f32 = 6.0;
const BUBBLE_ACCELERATION_RATE: f32 = 1800.;
const BUBBLE_COLLIDER_RADIUS: f32 = 8.0;
const BUBBLE_HEALTH: f32 = 1.0;
const BUBBLE_COLLISION_DAMAGE: f32 = 3.0;
// const BUBBLE_BOUNCINESS: f32 = 0.8;
const BUBBLE_DETECTION_RADIUS: f32 = 420.0;

pub struct BubblePlugin;

impl Plugin for BubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bubble_lifetime).in_set(InGameSet::DespawnEntities))
            .add_systems(
                Update,
                (
                    spawn_bubble_spawner,
                    spawn_bubble,
                    tracking::<Footman>,
                    tracking::<Tower>,
                )
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
    mut mana: ResMut<Mana>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::KeyW) {
        return;
    }

    let Ok(player_transform) = player.get_single() else {
        return;
    };

    if mana.0 >= SPAWNER_COST {
        mana.0 -= SPAWNER_COST;

        let texture: Handle<Image> = asset_server.load("bubble spawner.png");
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
            Collider::new(SPAWNER_COLLIDER_RADIUS),
            CollisionGroups::new(Group::ALLY, Group::NONE),
            DetectionGroups::new(Group::ALLY, Group::NONE),
            Target,
            Health::new(SPAWNER_HEALTH),
            BubbleSpawner {
                spawn_rate: Timer::from_seconds(SPAWNER_SPAWN_RATE, TimerMode::Repeating),
            },
            Name::new("BubbleSpawner"),
        ));
    }
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
                CollisionGroups::new(Group::ALLY, Group::ENEMY),
                Health::new(BUBBLE_HEALTH),
                CollisionDamage::new(BUBBLE_COLLISION_DAMAGE),
                DetectionGroups::new(Group::ALLY, Group::ENEMY),
                Tracker::new(BUBBLE_DETECTION_RADIUS),
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

fn tracking<T: Component>(
    mut detection_event_reader: EventReader<DetectionEvent>,
    mut tracker_query: Query<(&GlobalTransform, &mut Acceleration), With<Bubble>>,
    target_query: Query<&GlobalTransform, With<T>>,
) {
    for &DetectionEvent {
        tracker_entity,
        target_entity,
    } in detection_event_reader.read()
    {
        let Ok((tracker_transform, mut acceleration)) = tracker_query.get_mut(tracker_entity)
        else {
            continue;
        };

        let Ok(target_transform) = target_query.get(target_entity) else {
            continue;
        };

        let ttt: Vec3 = target_transform.translation();
        let planar_transform = Transform::from_xyz(ttt.x, ttt.y, tracker_transform.translation().z);
        let direction =
            (planar_transform.translation - tracker_transform.translation()).normalize();
        let distance = tracker_transform
            .translation()
            .distance(planar_transform.translation);

        acceleration.value = direction * BUBBLE_ACCELERATION_RATE / distance;
    }
}
