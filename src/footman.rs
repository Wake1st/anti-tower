use bevy::prelude::*;

use crate::{
    bubble::BubbleSpawner,
    collisions::{Collider, CollisionDamage, CollisionGroups},
    detection::{DetectionEvent, DetectionGroups, Target, Tracker},
    group::Group,
    health::Health,
    movement::{Acceleration, KinematicBundle, Velocity},
    schedule::InGameSet,
};

const HEALTH: f32 = 10.0;
const COLLIDER_RADIUS: f32 = 16.0;
const DETECTION_RADIUS: f32 = 600.0;
const DAMAGE: f32 = 5.0;
const VELOCITY_RATE: f32 = 1000.;

pub struct FootmanPlugin;

impl Plugin for FootmanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_footman.in_set(InGameSet::EntityUpdates))
            .add_systems(
                Update,
                (tracking::<BubbleSpawner>).in_set(InGameSet::EntityUpdates),
            );
    }
}

#[derive(Component)]
pub struct Footman;

fn spawn_footman(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("footman.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        KinematicBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
        },
        Collider::new(COLLIDER_RADIUS),
        CollisionGroups::new(Group::ENEMY, Group::ALLY | Group::PLAYER),
        CollisionDamage::new(DAMAGE),
        DetectionGroups::new(Group::ENEMY, Group::ALLY | Group::PLAYER),
        Tracker::new(DETECTION_RADIUS),
        Target,
        Health::new(HEALTH),
        Footman,
        Name::new("Footman"),
    ));
}

fn tracking<T: Component>(
    mut detection_event_reader: EventReader<DetectionEvent>,
    mut tracker_query: Query<(&GlobalTransform, &mut Acceleration), With<Footman>>,
    target_query: Query<&GlobalTransform, With<T>>,
) {
    for &DetectionEvent {
        tracker_entity,
        target_entity,
    } in detection_event_reader.read()
    {
        let Ok((tracker_transform, mut velocity)) = tracker_query.get_mut(tracker_entity) else {
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

        velocity.value = direction * VELOCITY_RATE / distance;
        info!("velocity {:?}", velocity.value);
    }
}
