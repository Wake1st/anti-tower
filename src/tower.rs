use bevy::prelude::*;

use crate::{
    collisions::{Collider, CollisionGroups},
    detection::{DetectionGroups, Target},
    footman::spawn_footman,
    group::Group,
    health::Health,
    schedule::InGameSet,
};

const SPRITE_LAYER: f32 = -1.0;
const COLLIDER_RADIUS: f32 = 60.0;
const HEALTH: f32 = 500.0;
const SPAWN_RATE: f32 = 10.0;
const SPAWN_OFFSET: Vec3 = Vec3::new(0.0, -40.0, 0.0);

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tower)
            .add_systems(Update, spawn_enemy.in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Component, Debug)]
pub struct Tower {
    spawn_rate: Timer,
}

fn spawn_tower(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("auto factory.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3 {
                    x: 400.,
                    y: 0.0,
                    z: SPRITE_LAYER,
                },
                ..default()
            },
            ..default()
        },
        Collider::new(COLLIDER_RADIUS),
        CollisionGroups::new(Group::ENEMY | Group::STRUCTURE, Group::NONE),
        DetectionGroups::new(Group::ENEMY | Group::STRUCTURE, Group::NONE),
        Target,
        Health::new(HEALTH),
        Tower {
            spawn_rate: Timer::from_seconds(SPAWN_RATE, TimerMode::Repeating),
        },
        Name::new("Tower"),
    ));
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawners: Query<(&mut Tower, &Transform)>,
) {
    for (mut spawner, &spawner_transform) in &mut spawners {
        spawner.spawn_rate.tick(time.delta());

        if spawner.spawn_rate.just_finished() {
            spawn_footman(
                &mut commands,
                &asset_server,
                spawner_transform.translation + SPAWN_OFFSET,
            );
        }
    }
}
