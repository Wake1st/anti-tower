use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::RigidBody,
    geometry::{Collider, CollisionGroups, Group},
};

use crate::{collisions::CollisionDamage, detection::Target, health::Health, schedule::InGameSet};

const HEALTH: f32 = 100.0;
const COLLIDER_RADIUS: f32 = 16.0;
const DAMAGE: f32 = 5.0;

pub struct FootmanPlugin;

impl Plugin for FootmanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_footman).in_set(InGameSet::EntityUpdates));
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
        // Collider::new(COLLIDER_RADIUS),
        RigidBody::KinematicVelocityBased,
        CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1 | Group::GROUP_4 | Group::GROUP_11,
        ),
        Collider::cuboid(COLLIDER_RADIUS, COLLIDER_RADIUS),
        CollisionDamage::new(DAMAGE),
        Target,
        Health::new(HEALTH),
        Footman,
        Name::new("Footman"),
    ));
}
