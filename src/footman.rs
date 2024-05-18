use bevy::prelude::*;

use crate::{
    collisions::{Collider, CollisionDamage, CollisionGroups, Group},
    detection::Target,
    health::Health,
    schedule::InGameSet,
};

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
        Collider::new(COLLIDER_RADIUS),
        CollisionGroups::new(Group::ENEMY, Group::NONE),
        CollisionDamage::new(DAMAGE),
        Target,
        Health::new(HEALTH),
        Footman,
        Name::new("Footman"),
    ));
}
