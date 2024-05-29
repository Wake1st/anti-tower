use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    attack::{Attack, AttackOccurance},
    bubble::BubbleSpawner,
    collisions::{Collider, CollisionDamage, CollisionGroups},
    detection::{DetectionEvent, DetectionGroups, Target, Tracker},
    group::Group,
    health::Health,
    movement::{Acceleration, KinematicBundle, Velocity},
    schedule::InGameSet,
};

const Z_LAYER: f32 = 0.0;
const HEALTH: f32 = 10.0;
const COLLIDER_RADIUS: f32 = 16.0;
const DETECTION_RADIUS: f32 = 600.0;
const DAMAGE: f32 = 5.0;
const ATTACK_RATE: f32 = 1.2;
const VELOCITY_RATE: f32 = 80.;

const ATTACK_END_TRANSLATION: Vec3 = Vec3::new(-16., -16., Z_LAYER);
const ATTACK_START_TRANSLATION: Vec3 = Vec3::new(-16., 0., Z_LAYER);
const ATTACK_END_ANGLE: f32 = PI / 2.;
const ATTACK_START_ROTATION: Quat = Quat::IDENTITY;

pub struct FootmanPlugin;

impl Plugin for FootmanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tracking::<BubbleSpawner>, spear_attack_animation).in_set(InGameSet::EntityUpdates),
        );
    }
}

#[derive(Component)]
pub struct Footman;

#[derive(Component)]
pub struct Spear;

pub fn spawn_footman(commands: &mut Commands, asset_server: &Res<AssetServer>, location: Vec3) {
    let footman_texture: Handle<Image> = asset_server.load("footman.png");
    let spear_texture: Handle<Image> = asset_server.load("spear.png");

    let footman_transform: Transform = Transform {
        translation: Vec3 {
            x: location.x,
            y: location.y,
            z: Z_LAYER,
        },
        ..default()
    };

    commands
        .spawn((
            SpriteBundle {
                texture: footman_texture,
                transform: footman_transform,
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
            Attack::new(
                DAMAGE,
                Timer::from_seconds(ATTACK_RATE, TimerMode::Repeating),
            ),
            Health::new(HEALTH),
            Footman,
            Name::new("Footman"),
        ))
        .with_children(|builder| {
            builder.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(-16., -16., Z_LAYER),
                        ..default()
                    },
                    texture: spear_texture,
                    sprite: Sprite {
                        anchor: Anchor::BottomCenter,
                        ..default()
                    },
                    ..default()
                },
                Spear,
                Name::new("Spear"),
            ));
        });

    //  must spawn a spear child; give spear Sprite.Anchor.BottomCenter
}

fn tracking<T: Component>(
    mut detection_event_reader: EventReader<DetectionEvent>,
    mut tracker_query: Query<(&GlobalTransform, &mut Velocity), With<Footman>>,
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

        velocity.value = direction * VELOCITY_RATE;
    }
}

fn spear_attack_animation(
    attacker_query: Query<(&Attack, &Children), With<AttackOccurance>>,
    mut spear_query: Query<&mut Transform, With<Spear>>,
) {
    for (attack, children) in attacker_query.iter() {
        for &child in children.iter() {
            let Ok(mut transform) = spear_query.get_mut(child) else {
                continue;
            };

            if attack.rate.just_finished() {
                //  reset transform
                transform.translation = ATTACK_START_TRANSLATION;
                transform.rotation = ATTACK_START_ROTATION;
            } else {
                let ratio = attack.rate.elapsed_secs() / ATTACK_RATE;

                //  moving up and down
                transform.translation = transform.translation.lerp(ATTACK_END_TRANSLATION, ratio);

                //  rotating around pivot
                transform.rotation = transform
                    .rotation
                    .lerp(Quat::from_rotation_z(ATTACK_END_ANGLE), ratio);
            }
        }
    }
}
