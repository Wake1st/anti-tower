use bevy::prelude::*;

use crate::schedule::InGameSet;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

#[derive(Bundle)]
pub struct KinematicBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        // info!("real accel {:?}", acceleration.value);
        // info!("pre vel {:?}", velocity.value);
        velocity.value += acceleration.value * time.delta_seconds();
        // info!("post vel {:?}", velocity.value);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        // info!("pre trans {:?}", transform.translation);
        transform.translation += velocity.value * time.delta_seconds();
        // info!("post trans {:?}", transform.translation);
    }
}
